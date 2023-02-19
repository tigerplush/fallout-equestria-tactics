use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, DefaultChannel};
use fallout_equestria_tactics::{axial_coordinates::AxialCoordinates, messages::ClientMessage};

use crate::common::{ClientState, ColliderClickedEvent, PlayerSpawnpoint};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(ClientState::SpawnPhase)
            .with_system(handle_clicks)
        );
        info!("SpawnPlugin has been loaded");
    }
}

fn handle_clicks(
    mut event_reader: EventReader<ColliderClickedEvent>,
    spawn_point: Res<PlayerSpawnpoint>,
    mut client: ResMut<RenetClient>,
) {
    for event in event_reader.iter() {
        let (coordinates, elevation) = AxialCoordinates::from_world(event.0);
        if(spawn_point.0.distance(&coordinates) < 8) {
            info!("click is within range");
            let message = bincode::serialize(&ClientMessage::TrySpawnCharacter(coordinates, elevation)).unwrap();
            client.send_message(DefaultChannel::Reliable, message);
        }
        else {
            info!("click is outside of range");
        }
    }
}