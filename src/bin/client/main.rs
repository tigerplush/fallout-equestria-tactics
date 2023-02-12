use bevy::{prelude::*, input::mouse::MouseWheel, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{RapierContext, QueryFilter, RapierPhysicsPlugin, NoUserData};
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(HookPlugin)
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(set_zoom)
        .add_system(cast_ray)
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

fn cast_ray(
    mouse_input: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        info!("click");
        for (camera, camera_transform) in &cameras {
            let (ray_pos, ray_dir) = ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
            info!("Casting ray from {} along {}", ray_pos, ray_dir);
            let hit = rapier_context.cast_ray(
                ray_pos,
                ray_dir,
                f32::MAX,
                true,
                QueryFilter::new()
            );

            if let Some((entity, _toi)) = hit {
                info!("Hit entity {:?}", entity);
            }
        }
    }
}

fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}