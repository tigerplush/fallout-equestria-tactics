use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
    PlayerConnected(u64),
    PlayerDisconnected(u64),
    PlayerName(String),
}


#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
}