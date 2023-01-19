use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;

use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use fallout_equestria_tactics::{
    messages::ServerMessage, resources::Players, PROTOCOL_ID, common::Player,
};

use crate::common::ClientState;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .insert_resource(FoEClient::new())
            .insert_resource(Players::new())
            .add_system(handle_messages.with_run_criteria(run_if_client_connected));
        info!("ClientPlugin loaded");
    }
}

struct FoEClient;

impl FoEClient {
    fn new() -> RenetClient {
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let connection_config = RenetConnectionConfig::default();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
    }
}

fn handle_messages(
    mut client: ResMut<RenetClient>,
    mut players: ResMut<Players>,
    mut app_state: ResMut<State<ClientState>>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerConnected(id) => {
                info!("{} connected", id);
                let mut entity = commands
                    .spawn_empty();

                if id == client.client_id() {
                    entity.insert(Player);
                    app_state.set(ClientState::Connected).unwrap();
                }
                players.players.insert(id, entity.id());
            }
            ServerMessage::PlayerDisconnected(id) => {
                info!("{} disconnected", id);
                if let Some(player) = players.players.remove(&id) {
                    commands.entity(player).despawn();
                }
            }
            ServerMessage::PlayerTurn(id) => {
                if id == client.client_id() {
                    app_state.set(ClientState::Acting).unwrap();
                }
                else if app_state.current() != &ClientState::Idling {
                    app_state.set(ClientState::Idling).unwrap();
                }
            }
            _ => (),
        }
    }
}