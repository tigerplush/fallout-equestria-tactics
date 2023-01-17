use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::assets::Names;

#[derive(Resource)]
pub struct NamesHandle(pub Handle<Names>);

#[derive(Resource)]
pub struct Players {
    pub players: HashMap<u64, Entity>,
}