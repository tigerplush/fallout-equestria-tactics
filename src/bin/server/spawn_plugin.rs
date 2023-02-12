use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use fallout_equestria_tactics::{common::Spawnpoint, resources::Players, map::AxialCoordinates};

use crate::common::ServerState;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ServerState::SpawnPhase)
                .with_system(notify_players)
        );
        info!("SpawnPlugin has been loaded");
    }
}

fn notify_players(
    query: Query<&Transform, With<Spawnpoint>>,
    mut server: ResMut<RenetServer>,
    players: Res<Players>
) {
    for transform in &query {
        let axial_coordinates = AxialCoordinates::from_world(transform.translation);
    }
}