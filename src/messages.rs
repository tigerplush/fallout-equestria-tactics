use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::AxialCoordinates;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
    PlayerConnected(u64, Entity),
    PlayerDisconnected(u64),
    PlayerName(String),
    PlayerTurn(u64),
    PlayerNameChanged(u64, String),
    LoadLevel(String),
    /// Assigns a spawnpoint in q, r, elevation
    AssignSpawnpoint(AxialCoordinates),
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    ClientReady,
    ChangeName(String),
    EndTurn,
    LevelLoaded,
}

pub enum ChatMessage {
    Public(String),
    Private(u64, String),
}
