use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{NoUserData, QueryFilter, RapierContext, RapierPhysicsPlugin};
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

fn main() {
    App::new()
        .add_state(ClientState::WaitingToConnect)
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(LevelLoaderPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(InitPlugin)
        .add_plugin(HookPlugin)
        .add_system(cast_ray)
        .run();
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
            let (ray_pos, ray_dir) =
                ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
            info!("Casting ray from {} along {}", ray_pos, ray_dir);
            let hit = rapier_context.cast_ray(ray_pos, ray_dir, f32::MAX, true, QueryFilter::new());

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

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}
