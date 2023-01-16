use bevy::prelude::*;
use fallout_equestria_tactics::foe_client_plugin::FoEClientPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FoEClientPlugin)
        .run();
}