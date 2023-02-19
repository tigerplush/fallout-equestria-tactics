use bevy::prelude::*;

#[cfg(feature = "fps")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_scene_hook::HookPlugin;

mod camera_plugin;
use camera_plugin::CameraPlugin;

mod client_plugin;
use client_plugin::*;

mod common;
use common::ClientState;

mod gui_plugin;
use gui_plugin::GuiPlugin;

mod init_plugin;
use init_plugin::InitPlugin;

mod level_loader_plugin;
use level_loader_plugin::LevelLoaderPlugin;

mod lobby_plugin;
use lobby_plugin::LobbyPlugin;

mod spawn_plugin;
use spawn_plugin::SpawnPlugin;

fn main() {
    let mut app = App::new();
    app
        .add_state(ClientState::Init)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(SpawnPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(InitPlugin)
        .add_plugin(HookPlugin);

    #[cfg(feature = "fps")]
    {
        app
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }

    app
        .run();
}

