use std::{time::SystemTime, net::UdpSocket};

use bevy::prelude::*;

use bevy_renet::{renet::{RenetClient, RenetConnectionConfig, ClientAuthentication, DefaultChannel}, RenetClientPlugin};
use fallout_equestria_tactics::{PROTOCOL_ID, messages::ServerMessage, common::PlayerName};
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default());
        app.insert_resource(FoEClient::new());
        app.add_system(FoEClient::handle_messages);
    }
}

struct FoEClient;

impl FoEClient {
    fn new() -> RenetClient {
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let connection_config = RenetConnectionConfig::default();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        RenetClient::new(
            current_time,
            socket,
            connection_config,
            authentication).unwrap()
    }

    fn handle_messages(
        mut client: ResMut<RenetClient>,
        mut commands: Commands
    ) {
        while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
            let server_message = bincode::deserialize(&message).unwrap();
            match server_message {
                ServerMessage::PlayerConnected(id) => info!("{} connected", id),
                ServerMessage::PlayerDisconnected(id) => info!("{} disconnected", id),
                _ => (),
            }
        }

        while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
            let server_message = bincode::deserialize(&message).unwrap();
            match server_message {
                ServerMessage::PlayerName(name) => {
                    
                    info!("{} received the name {}", client.client_id(), name);
                    commands.insert_resource(PlayerName(name));
                },
                _ => (),
            }
        }
    }
}