use bevy::prelude::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        info!("LobbyPlugin has been loaded");
    }
}