use std::{time::SystemTime, net::UdpSocket};
use bevy::prelude::*;
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication}};

pub struct FoEServerPlugin;

impl Plugin for FoEServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoEServer::new());
    }
}

const PROTOCOL_ID: u64 = 7;

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
}