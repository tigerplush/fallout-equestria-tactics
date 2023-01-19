use std::hash::Hash;

use bevy::utils::hashbrown::HashMap;
use bevy::{prelude::*, log::LogPlugin};
use bevy_turborand::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use fallout_equestria_tactics::assets::Names;
use fallout_equestria_tactics::resources::{NamesHandle, Players};

mod server_plugin;
use server_plugin::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ServerState {
    WaitingForPlayers,
    PlayerTurn(u64),
}

fn main() {
    let mut app = App::new();
    app
    .add_state(ServerState::WaitingForPlayers)
    .add_plugins(MinimalPlugins)
    .add_plugin(AssetPlugin::default())
    .add_plugin(JsonAssetPlugin::<Names>::new(&["json.names"]))
    .add_plugin(LogPlugin::default())
    .add_plugin(ServerPlugin)
    .add_plugin(RngPlugin::default())
    .add_startup_system(setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let names_handle: Handle<Names> = asset_server.load("names.json.names");
    commands.insert_resource(NamesHandle(names_handle));
    commands.insert_resource(Players {
        players: HashMap::new(),
    });
}
