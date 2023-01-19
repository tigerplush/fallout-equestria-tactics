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