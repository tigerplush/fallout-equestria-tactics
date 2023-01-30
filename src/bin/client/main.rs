use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::HookPlugin;

mod client_plugin;
use client_plugin::*;

mod common;
use common::ClientState;

mod gui_plugin;
use fallout_equestria_tactics::resources::LevelName;
use gui_plugin::GuiPlugin;

mod level_loader_plugin;
use level_loader_plugin::LevelLoaderPlugin;

fn main() {
    App::new()
        .insert_resource(LevelName::default())
        .add_state(ClientState::WaitingToConnect)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(HookPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_scene)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..default()
        }),
        ..default()
    });
}


fn spawn_scene(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}