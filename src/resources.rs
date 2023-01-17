use bevy::prelude::*;

use crate::assets::Names;

#[derive(Resource)]
pub struct NamesHandle(pub Handle<Names>);