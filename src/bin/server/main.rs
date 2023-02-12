use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_turborand::prelude::*;

use fallout_equestria_tactics::assets::Names;
use fallout_equestria_tactics::map::Map;
use fallout_equestria_tactics::resources::{Players, TurnOrder, LevelName};

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
    app.add_state(ServerState::WaitingForPlayerReadiness)
        .insert_resource(LevelName::new("level.gltf#Scene0"))
        .insert_resource(Players::new())
        .insert_resource(TurnOrder::new())
        .insert_resource(Map::generate(20, 20))
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<Names>::new(&["json.names"]))
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(ServerPlugin)
        .add_plugin(SpawnPlugin)
        .add_plugin(RngPlugin::default())
        .add_startup_system(setup);

    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            near: -10.0,
            ..default()
        }),
        ..default()
    });
}
