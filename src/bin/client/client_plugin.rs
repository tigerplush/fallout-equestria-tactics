use std::{net::{UdpSocket, SocketAddr}, time::SystemTime};

use bevy::prelude::*;

use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    RenetClientPlugin,
};
use fallout_equestria_tactics::{
    common::{Player, ServerEntity, Username},
    messages::{ClientMessage, ServerMessage},
    resources::{LevelName, Players},
    PROTOCOL_ID,
};

use crate::common::ClientState;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .insert_resource(
                FoEClient::new(
                    "127.0.0.1:5000".parse().unwrap(),
                    &Username("fartbag".to_string())
                )
            )
            .insert_resource(Players::new())
            .add_system(handle_reliable_messages)
            .add_system(handle_unreliable_messages);
        info!("ClientPlugin loaded");
    }
}

struct FoEClient;

impl FoEClient {
    fn new(server_addr: SocketAddr, user_name: &Username) -> RenetClient {
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
            user_data: Some(user_name.to_netcode_user_data()),
        };
        RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
    }
}

fn handle_reliable_messages(
    mut client: ResMut<RenetClient>,
    mut players: ResMut<Players>,
    mut app_state: ResMut<State<ClientState>>,
    mut commands: Commands,
    mut level_name: ResMut<LevelName>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerConnected(id, player_name, server_entity) => {
                info!("{} connected", id);
                let mut entity = commands
                    .spawn(ServerEntity(server_entity));

                entity.insert(Name::from(player_name));
                if id == client.client_id() {
                    entity.insert(Player(id));
                    app_state.set(ClientState::Lobby).unwrap();
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
                } else if app_state.current() != &ClientState::Idling {
                    app_state.set(ClientState::Idling).unwrap();
                }
            }
            ServerMessage::LoadLevel(level) => {
                info!("Shoud load level {}", level);
                level_name.0 = level;
                app_state.set(ClientState::LoadingLevel).unwrap();
            }
            ServerMessage::AssignSpawnpoint(spawn_point, elevation) => {
                info!("This players spawnpoint is {:?}, {:?}", spawn_point, elevation);
                for mut camera in &mut cameras {
                    camera.translation = camera.translation + spawn_point.to_world(elevation);
                }
                app_state.set(ClientState::SpawnPhase).unwrap();
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
        let server_message: ServerMessage = bincode::deserialize(&message).unwrap();
        match server_message {
            _ => (),
        }
    }
}
