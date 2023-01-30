use bevy::prelude::*;

#[derive(Component)]
pub struct Readiness(pub bool);

#[derive(Component)]
pub struct Player;

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
