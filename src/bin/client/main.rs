use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::*;

mod client_plugin;
use client_plugin::*;

mod common;
use common::ClientState;

mod gui_plugin;
use gui_plugin::GuiPlugin;

fn main() {
    App::new()
        .add_state(ClientState::WaitingToConnect)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ClientPlugin)
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

#[derive(Component)]
struct Spawnpoint;

fn spawn_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("level.gltf#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str().split('.').collect::<Vec<&str>>()[0]) {
                Some("Spawnpoint") => cmds.insert(Spawnpoint),
                _ => cmds,
            };
        }),
    })
    .insert(Name::from("Level"));
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}