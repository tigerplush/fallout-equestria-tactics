
use bevy::prelude::*;
use bevy_turborand::prelude::*;

mod foe_server;

mod init_plugin;
use init_plugin::InitPlugin;

mod lobby_plugin;
use lobby_plugin::LobbyPlugin;

mod server_plugin;
use server_plugin::*;

mod spawn_plugin;
use spawn_plugin::*;

mod level_loader_plugin;
use level_loader_plugin::*;

mod common;
use common::ServerState;

fn main() {
    let mut app = App::new();
    app.add_state(ServerState::Init)
        .add_plugins(DefaultPlugins)
        .add_plugin(InitPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(ServerPlugin)
        .add_plugin(SpawnPlugin)
        .add_plugin(RngPlugin::default());

    app.run();
}

