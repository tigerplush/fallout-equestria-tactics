use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy_renet::renet::{RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig};
use fallout_equestria_tactics::PROTOCOL_ID;

pub struct FoEServer;

impl FoEServer {
    pub fn new(server_addr: SocketAddr) -> RenetServer {
        let socket = UdpSocket::bind(server_addr).unwrap();
        let connection_config = RenetConnectionConfig::default();
        let server_config =
            ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
    }
}
