use std::{net::UdpSocket, time::SystemTime, collections::VecDeque};

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
        ServerEvent,
    },
    RenetServerPlugin,
};

use bevy_turborand::GlobalRng;
use rand::seq::{IteratorRandom, SliceRandom};

use fallout_equestria_tactics::{
    common::{Readiness, CurrentPlayer},
    messages::{ClientMessage, ServerMessage},
    resources::{Players, TurnOrder},
    PROTOCOL_ID,
};

use crate::common::ServerState;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoEServer::new())
            .add_plugin(RenetServerPlugin::default())
            .add_system(handle_server_messages)
            .add_system(handle_client_messages)
            .add_system_set(
                SystemSet::on_update(ServerState::WaitingForPlayers)
                    .with_system(handle_readiness)
            )
            .add_system_set(
                SystemSet::on_enter(ServerState::PlayerTurn)
                    .with_system(handle_new_turn)
            );
    }
}

struct FoEServer;

impl FoEServer {
    fn new() -> RenetServer {
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind(server_addr).unwrap();
        let connection_config = RenetConnectionConfig::default();
        let server_config =
            ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
    }
}

fn handle_server_messages(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut players: ResMut<Players>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("{} connected", id);

                let entity = commands
                    .spawn_empty()
                    .insert(Readiness(false))
                    .id();

                //notify the connected player of all other players
                for &player_id in players.players.keys() {
                    let message = bincode::serialize(&ServerMessage::PlayerConnected(player_id)).unwrap();
                    server.send_message(*id, DefaultChannel::Reliable, message);
                }

                players.players.insert(*id, entity);

                // notify everyone of the new player
                let message = bincode::serialize(&ServerMessage::PlayerConnected(*id)).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("{} disconnected", id);
                if let Some(player_entity) = players.players.remove(id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessage::PlayerDisconnected(*id)).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            }
        }
    }
}

fn handle_client_messages(
    mut server: ResMut<RenetServer>,
    players: Res<Players>,
    mut query: Query<&mut Readiness>,
) {
    for client_id in server.clients_id().into_iter() {
        if let Some(entity) = players.players.get(&client_id) {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::Reliable)
            {
                let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
                let mut readiness = query.get_mut(*entity).unwrap();
                match client_message {
                    ClientMessage::ClientReady => {
                        readiness.0 = !readiness.0;
                        info!(
                            "Player {} reports {}readiness",
                            client_id,
                            match readiness.0 {
                                true => "",
                                false => "un",
                            }
                        );
                    }
                    _ => (),
                }
            }
        }
    }
}

fn handle_readiness(
    query: Query<&Readiness>,
    mut app_state: ResMut<State<ServerState>>,
    players: Res<Players>,
    mut turn_order: ResMut<TurnOrder>,
) {
    if query.iter().all(|r| r.0 == true) && !query.is_empty() {
        info!("All players report readiness");
        let mut rng = rand::thread_rng();
        let mut random_order: Vec<u64> = players.players.keys().map(|f| *f).collect();
        random_order.shuffle(&mut rng);
        turn_order.order = VecDeque::from(random_order);
        app_state.set(ServerState::PlayerTurn).unwrap();
    }
}

fn handle_new_turn(
    app_state: Res<State<ServerState>>,
    mut server: ResMut<RenetServer>,
    mut turn_order: ResMut<TurnOrder>,
    players: Res<Players>,
    mut commands: Commands,
    query: Query<Entity, With<CurrentPlayer>>,
) {
    info!("handling new turn");
    match app_state.current() {
        ServerState::PlayerTurn => {
            if let Some(player) = turn_order.order.pop_front() {
                info!("It's {}'s turn", player);
                for current_player in &query {
                    commands.entity(current_player).remove::<CurrentPlayer>();
                }
                if let Some(entity) = players.players.get(&player) {
                    commands.entity(*entity).insert(CurrentPlayer);
                }
                let message = bincode::serialize(&ServerMessage::PlayerTurn(player)).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            }
            else {
                error!("Turn order is empty");
            }
        }
        _ => error!("Somehow an empty server started"),
    }
}