use bevy::prelude::EventReader;
use bevy_renet::renet::RenetError;

pub mod assets;
pub mod common;
pub mod foe_server_plugin;
pub mod foe_client_plugin;
pub mod messages;
pub mod resources;

const PROTOCOL_ID: u64 = 7;

pub fn handle_errors(
    mut renet_error: EventReader<RenetError>
) {
    for error in renet_error.iter() {
        panic!("{}", error);
    }
}