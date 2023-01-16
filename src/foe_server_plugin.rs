use std::{time::SystemTime, net::UdpSocket};
use bevy::prelude::*;
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication, ServerEvent, DefaultChannel}};

use crate::{PROTOCOL_ID, messages::ServerMessage, handle_errors};

pub struct FoEServerPlugin;

impl Plugin for FoEServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoEServer::new());
        app.add_plugin(RenetServerPlugin::default());
        app.add_system(FoEServer::handle_messages);
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

    fn handle_messages(
        mut server_events: EventReader<ServerEvent>,
        mut server: ResMut<RenetServer>,
    ) {
        for event in server_events.iter() {
            match event {
                ServerEvent::ClientConnected(id, _) => {
                    info!("{} connected", id);
                    let message = bincode::serialize(&ServerMessage::PlayerConnected(*id)).unwrap();
                    server.broadcast_message(DefaultChannel::Reliable, message);
                },
                ServerEvent::ClientDisconnected(id) => {
                    info!("{} disconnected", id);
                    let message = bincode::serialize(&ServerMessage::PlayerDisconnected(*id)).unwrap();
                    server.broadcast_message(DefaultChannel::Reliable, message);
                }
            }
        }
    }
}