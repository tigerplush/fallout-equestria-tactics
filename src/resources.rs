use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::assets::Names;

#[derive(Resource)]
pub struct NamesHandle(pub Handle<Names>);

#[derive(Resource)]
pub struct Players {
    pub players: HashMap<u64, Entity>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn get(&self, k: &u64) -> Option<&Entity> {
        self.players.get(k)
    }
}

#[derive(Resource)]
pub struct TurnOrder {
    pub order: VecDeque<u64>,
}

impl TurnOrder {
    pub fn new() -> Self {
        Self {
            order: VecDeque::new(),
        }
    }
}

#[derive(Clone, Resource)]
pub struct LevelName(pub String);

impl LevelName {
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

impl Default for LevelName {
    fn default() -> Self {
        Self(String::new())
    }
}