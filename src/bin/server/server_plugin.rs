use bevy::prelude::*;
use bevy_renet::{
    renet::{DefaultChannel, RenetServer, ServerEvent},
    RenetServerPlugin,
};

use fallout_equestria_tactics::{
    common::{CurrentPlayer, LevelLoaded, Player, Readiness, Username},
    messages::{ClientMessage, ServerMessage},
    resources::{Players, TurnOrder},
};

use crate::common::ServerState;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin::default())
            .add_system(handle_server_events)
            .add_system(handle_reliable_messages)
            .add_system(handle_unreliable_messages)
            .add_system_set(
                SystemSet::on_enter(ServerState::PlayerTurn)
                .with_system(handle_new_turn),
            )
            .add_system_set(
                //todo: this should be on enter, but changing a state on enter crashes
                // maybe resetting a state works? eliminate NextTurn altogether?
                SystemSet::on_update(ServerState::NextTurn)
                .with_system(next_turn),
            );
        info!("ServerPlugin has been loaded");
    }
}

fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut players: ResMut<Players>,
    player_query: Query<(&Player, Entity, &Name)>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                let user_name = Username::from_user_data(user_data);
                info!("{} ({}) connected", user_name.0, id);

                let entity = commands
                    .spawn(Player(*id))
                    .insert(Readiness(false))
                    .insert(LevelLoaded(false))
                    .insert(Name::from(user_name.0.clone()))
                    .id();

                for (player, server_entity, player_name) in &player_query {
                    let message = bincode::serialize(&ServerMessage::PlayerConnected(
                        player.0,
                        player_name.to_string(),
                        server_entity,
                    ))
                    .unwrap();
                    server.send_message(*id, DefaultChannel::Reliable, message);
                }

                players.players.insert(*id, entity);

                // notify everyone of the new player
                let message =
                    bincode::serialize(&ServerMessage::PlayerConnected(*id, user_name.0, entity)).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("{} disconnected", id);
                if let Some(player_entity) = players.players.remove(id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessage::PlayerDisconnected(*id)).unwrap();
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
    mut level_loaded_query: Query<&mut LevelLoaded>,
) {
    for client_id in server.clients_id().into_iter() {
        if let Some(&entity) = players.get(&client_id) {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
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
                    ClientMessage::LevelLoaded => {
                        let mut level_loaded = level_loaded_query.get_mut(entity).unwrap();
                        level_loaded.0 = true;
                        info!("Player {} reports level loaded", client_id,);
                    }
                    _ => (),
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
            while let Some(message) = server.receive_message(client_id, DefaultChannel::Unreliable)
            {
                let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
                match client_message {
                    _ => (),
                }
            }
        }
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
            commands
                .entity(current_player_entity)
                .remove::<CurrentPlayer>();
            turn_order.order.push_back(current_player.0);
        }
        if let Some(entity) = players.players.get(&next_player) {
            commands.entity(*entity).insert(CurrentPlayer(next_player));
        }
        let message = bincode::serialize(&ServerMessage::PlayerTurn(next_player)).unwrap();
        server.broadcast_message(DefaultChannel::Reliable, message);
    } else {
        error!("Turn order is empty");
    }
}

fn next_turn(mut app_state: ResMut<State<ServerState>>) {
    app_state.set(ServerState::PlayerTurn).unwrap();
}
