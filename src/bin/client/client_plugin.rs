use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;

use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    RenetClientPlugin,
};
use fallout_equestria_tactics::{
    messages::{ServerMessage, ClientMessage}, resources::Players, PROTOCOL_ID, common::{Player, ServerEntity},
};

use crate::common::ClientState;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .insert_resource(FoEClient::new())
            .insert_resource(Players::new())
            .add_system(handle_reliable_messages)
            .add_system(handle_unreliable_messages);
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

fn handle_reliable_messages(
    mut client: ResMut<RenetClient>,
    mut players: ResMut<Players>,
    mut app_state: ResMut<State<ClientState>>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerConnected(id, server_entity) => {
                info!("{} connected", id);
                let mut entity = commands
                    .spawn(ServerEntity(server_entity));

                if id == client.client_id() {
                    entity.insert(Player);
                    let message = bincode::serialize(&ClientMessage::ChangeName("Fartbag".to_string())).unwrap();
                    client.send_message(DefaultChannel::Unreliable, message);
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

fn handle_unreliable_messages(
    mut client: ResMut<RenetClient>,
    players: Res<Players>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerNameChanged(id, new_name) => {
                if let Some(&entity) = players.get(&id) {
                    commands.entity(entity).insert(Name::from(new_name));
                }
            }
            _ => (),
        }
    }
}