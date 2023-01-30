use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::{log::LogPlugin, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_turborand::prelude::*;

use fallout_equestria_tactics::assets::Names;
use fallout_equestria_tactics::map::Map;
use fallout_equestria_tactics::resources::{NamesHandle, Players, TurnOrder};

mod server_plugin;
use server_plugin::*;

mod common;
use common::ServerState;

fn main() {
    let mut app = App::new();
    app.add_state(ServerState::WaitingForPlayerReadiness)
        .insert_resource(Players::new())
        .insert_resource(TurnOrder::new())
        .insert_resource(Map::generate(20, 20))
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(JsonAssetPlugin::<Names>::new(&["json.names"]))
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin)
        .add_plugin(RngPlugin::default())
        .add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let names_handle: Handle<Names> = asset_server.load("names.json.names");
    commands.insert_resource(NamesHandle(names_handle));
}
