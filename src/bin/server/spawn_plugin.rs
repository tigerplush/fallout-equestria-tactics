use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use fallout_equestria_tactics::{
    common::{Player, Spawnpoint},
    map::AxialCoordinates,
    messages::ServerMessage,
};

use crate::common::ServerState;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ServerState::SpawnPhase).with_system(notify_players),
        );
        info!("SpawnPlugin has been loaded");
    }
}

/// Notifies players of their spawnpoint on the Default Reliable channel
fn notify_players(
    query: Query<&Transform, With<Spawnpoint>>,
    mut player_query: Query<&Player>,
    mut server: ResMut<RenetServer>,
) {
    info!("assigning spawn points");
    let mut player_iter = player_query.iter_mut();
    for transform in &query {
        info!("assigning spawn point {:?}", transform);
        if let Some(player) = player_iter.next() {
            info!("assigning spawn point {:?} to {}", transform, player.0);
            let axial_coordinates = AxialCoordinates::from_world(transform.translation);
            let message =
                bincode::serialize(&ServerMessage::AssignSpawnpoint(axial_coordinates)).unwrap();
            server.send_message(player.0, DefaultChannel::Reliable, message);
        }
    }
}
