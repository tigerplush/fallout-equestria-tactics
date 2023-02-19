use bevy::prelude::*;
use fallout_equestria_tactics::resources::LevelName;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(LevelName::default());
        info!("InitPlugin has been loaded");
    }
}