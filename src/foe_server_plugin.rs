use std::{time::SystemTime, net::UdpSocket};
use bevy::prelude::*;
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication, ServerEvent, DefaultChannel}};
use bevy_turborand::prelude::*;

use crate::{PROTOCOL_ID, messages::{ServerMessage, ClientMessage}, handle_errors, resources::NamesHandle, assets::Names};

pub struct FoEServerPlugin;

impl Plugin for FoEServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoEServer::new());
        app.add_plugin(RenetServerPlugin::default());
        app.add_system(FoEServer::handle_server_messages);
        app.add_system(FoEServer::handle_client_messages);
        app.add_system(handle_errors);
    }
}

struct FoEServer;

impl FoEServer {
    fn new() -> RenetServer {
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind(server_addr).unwrap();
        let connection_config = RenetConnectionConfig::default();
        let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        RenetServer::new(
            current_time,
            server_config,
            connection_config,
            socket,
        ).unwrap()
    }

    fn handle_server_messages(
        mut server_events: EventReader<ServerEvent>,
        mut server: ResMut<RenetServer>,
        mut rng: ResMut<GlobalRng>,
        names_handle: Res<NamesHandle>,
        mut names_asset: ResMut<Assets<Names>>,
    ) {
        for event in server_events.iter() {
            match event {
                ServerEvent::ClientConnected(id, _) => {
                    info!("{} connected", id);
                    let message = bincode::serialize(&ServerMessage::PlayerConnected(*id)).unwrap();
                    server.broadcast_message(DefaultChannel::Reliable, message);

                    let mut name = String::from("Fallback McFallbackerson");
                    if let Some(names) = names_asset.get_mut(&names_handle.0) {
                        let element = rng.usize(0..names.names.len());
                        name = names.names.remove(element);
                    }
                    let name_message = bincode::serialize(&ServerMessage::PlayerName(name)).unwrap();
                    server.send_message(*id, DefaultChannel::Unreliable, name_message)
                },
                ServerEvent::ClientDisconnected(id) => {
                    info!("{} disconnected", id);
                    let message = bincode::serialize(&ServerMessage::PlayerDisconnected(*id)).unwrap();
                    server.broadcast_message(DefaultChannel::Reliable, message);
                }
            }
        }
    }

    fn handle_client_messages(
        mut server: ResMut<RenetServer>,
    ) {
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
                let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
                match client_message {
                    ClientMessage::ClientReady => info!("Player {} reports readiness", client_id),
                    _ => ()
                }
            }
        }
    }
}