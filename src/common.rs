use bevy::prelude::*;
use bevy_renet::renet::NETCODE_USER_DATA_BYTES;

#[derive(Component)]
pub struct Readiness(pub bool);

#[derive(Component)]
pub struct LevelLoaded(pub bool);

#[derive(Component)]
pub struct Player(pub u64);

#[derive(Component, Deref, DerefMut)]
pub struct CurrentPlayer(pub u64);

#[derive(Component)]
pub struct ServerEntity(pub Entity);

pub struct Special {
    pub strength: u8,
    pub perception: u8,
    pub endurance: u8,
    pub charisma: u8,
    pub intelligence: u8,
    pub agility: u8,
    pub luck: u8,
}

impl Special {
    pub fn new() -> Self {
        Self {
            strength: 5,
            perception: 5,
            endurance: 5,
            charisma: 5,
            intelligence: 5,
            agility: 5,
            luck: 5,
        }
    }
}

pub enum TileType {
    Passable(u8),
    Impassable,
}

pub enum Race {
    EarthPony,
    Unicorn,
    Pegasus,
}

#[derive(Component)]
pub struct Spawnpoint;

pub struct Username(pub String);

impl Username {
    /// Packs the username into a byte array with 256 bytes payload
    /// 
    /// First 8 bytes are the length of the string in u64
    pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            panic!("Username is too long!");
        }
        user_data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
        user_data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&user_data[0..8]);
        let mut len = u64::from_le_bytes(buffer) as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 8);
        let data = user_data[8..len + 8].to_vec();
        let username = String::from_utf8(data).unwrap();
        Self(username)
    }
}