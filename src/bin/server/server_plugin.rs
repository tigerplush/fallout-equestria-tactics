use std::{net::UdpSocket, time::SystemTime, collections::VecDeque};

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
        ServerEvent,
    },
    RenetServerPlugin,
};

use rand::seq::SliceRandom;

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
            .add_system(handle_server_events)
            .add_system(handle_reliable_messages)
            .add_system(handle_unreliable_messages)
            .add_system_set(
                SystemSet::on_update(ServerState::WaitingForPlayers)
                    .with_system(handle_readiness)
            )
            .add_system_set(
                SystemSet::on_enter(ServerState::PlayerTurn)
                    .with_system(handle_new_turn)
            )
            .add_system_set(
                //todo: this should be on enter, but changing a state on enter crashes
                // maybe resetting a state works? eliminate NextTurn altogether?
                SystemSet::on_update(ServerState::NextTurn)
                    .with_system(next_turn)
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

fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut players: ResMut<Players>,
    query: Query<&Name>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("{} connected", id);

                let entity = commands
                    .spawn_empty()
                    .insert(Readiness(false))
                    .id();

                for (&player_id, &server_entity) in players.players.iter() {
                    let message = bincode::serialize(&ServerMessage::PlayerConnected(player_id, server_entity)).unwrap();
                    server.send_message(*id, DefaultChannel::Reliable, message);

                    let name = query.get(server_entity).unwrap();
                    let message = bincode::serialize(&ServerMessage::PlayerNameChanged(player_id, name.into())).unwrap();
                    server.send_message(*id, DefaultChannel::Unreliable, message);
                }

                players.players.insert(*id, entity);

                // notify everyone of the new player
                let message = bincode::serialize(&ServerMessage::PlayerConnected(*id, entity)).unwrap();
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

fn handle_reliable_messages(
    mut server: ResMut<RenetServer>,
    players: Res<Players>,
    mut query: Query<&mut Readiness>,
    mut app_state: ResMut<State<ServerState>>,
) {
    for client_id in server.clients_id().into_iter() {
        if let Some(&entity) = players.get(&client_id) {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::Reliable)
            {
                let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
                match client_message {
                    ClientMessage::ClientReady => {
                        let mut readiness = query.get_mut(entity).unwrap();
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
                    ClientMessage::EndTurn => {
                        app_state.set(ServerState::NextTurn).unwrap();
                        return;
                    }
                    _ => ()
                }
            }
        }
    }
}

fn handle_unreliable_messages(
    mut server: ResMut<RenetServer>,
    players: Res<Players>,
    mut commands: Commands,
) {
    for client_id in server.clients_id().into_iter() {
        if let Some(&entity) = players.get(&client_id) {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::Unreliable)
            {
                let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
                match client_message {
                    ClientMessage::ChangeName(new_name) => {
                        commands
                            .entity(entity)
                            .insert(Name::from(new_name.clone()));
                        let message = bincode::serialize(&ServerMessage::PlayerNameChanged(client_id, new_name)).unwrap();
                        server.broadcast_message(DefaultChannel::Unreliable, message);
                    }
                    _ => ()
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

/// Runs once when PlayerTurn is entered
fn handle_new_turn(
    mut server: ResMut<RenetServer>,
    mut turn_order: ResMut<TurnOrder>,
    players: Res<Players>,
    mut commands: Commands,
    query: Query<(Entity, &CurrentPlayer)>,
) {
    info!("handling new turn");
    if let Some(next_player) = turn_order.order.pop_front() {
        info!("It's {}'s turn", next_player);
        for (current_player_entity, current_player) in &query {
            commands.entity(current_player_entity).remove::<CurrentPlayer>();
            turn_order.order.push_back(current_player.0);
        }
        if let Some(entity) = players.players.get(&next_player) {
            commands.entity(*entity).insert(CurrentPlayer(next_player));
        }
        let message = bincode::serialize(&ServerMessage::PlayerTurn(next_player)).unwrap();
        server.broadcast_message(DefaultChannel::Reliable, message);
    }
    else {
        error!("Turn order is empty");
    }
}

fn next_turn(
    mut app_state: ResMut<State<ServerState>>,
) {
    app_state.set(ServerState::PlayerTurn).unwrap();
}