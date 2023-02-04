use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, egui::Key};
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
        .insert_resource(CurrentZoom(0.0))
        .add_state(ClientState::WaitingToConnect)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(HookPlugin)
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(set_zoom)
        .add_startup_system(spawn_scene)
        .run();
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

const MOVE_SPEED: f32 = 8.0;
const MIN_ZOOM: f32 = 0.01;
const MAX_ZOOM: f32 = 0.04;
const ZOOM_SPEED: f32 = 2.0;

#[derive(Resource)]
struct CurrentZoom(f32);

impl CurrentZoom {
    pub fn add(&mut self, zoom: f32) {
        self.0 = (self.0 + zoom * ZOOM_SPEED).clamp(0.0, 1.0);
    }

    pub fn get_mapped(&self) -> f32 {
        MIN_ZOOM + (MAX_ZOOM - MIN_ZOOM) * self.0
    }
}

fn move_camera(
    key_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut current_zoom: ResMut<CurrentZoom>,
) {
    let mut dir = Vec3::splat(0.0);
    if key_input.pressed(KeyCode::A) {
        dir.x += 1.0;
        dir.z += 1.0;
    }
    if key_input.pressed(KeyCode::D) {
        dir.x -= 1.0;
        dir.z -= 1.0;
    }
    if key_input.pressed(KeyCode::W) {
        dir.x -= 1.0;
        dir.z += 1.0;
    }
    if key_input.pressed(KeyCode::S) {
        dir.x += 1.0;
        dir.z -= 1.0;
    }

    for event in mouse_wheel_events.iter() {
        current_zoom.add(-event.y * time.delta_seconds());
        info!("{}", current_zoom.0);
    }

    for mut transform in &mut query {
        transform.translation += dir.normalize_or_zero() * time.delta_seconds() * MOVE_SPEED;
    }
}

fn set_zoom(
    mut query: Query<&mut Projection>,
    current_zoom: Res<CurrentZoom>
) {
    for mut projection in &mut query {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = current_zoom.get_mapped();
        }
    }
}

fn spawn_scene(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}