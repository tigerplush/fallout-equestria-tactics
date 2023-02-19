use bevy::prelude::*;
use bevy_rapier3d::prelude::{RapierContext, QueryFilter};
use fallout_equestria_tactics::{resources::LevelName, axial_coordinates::AxialCoordinates};

use crate::common::ColliderClickedEvent;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(LevelName::default())
        .add_event::<ColliderClickedEvent>()
        .add_startup_system(spawn_cursor)
        .add_system(cast_ray);
        info!("InitPlugin has been loaded");
    }
}

#[derive(Component)]
pub struct Cursor;

fn spawn_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 4,
        })),
        material: materials.add(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Cursor);
}

fn cast_ray(
    mouse_input: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<ColliderClickedEvent>,
    mut query: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
) {
    for (camera, camera_transform) in &cameras {
        let (ray_pos, ray_dir) =
            ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
        let hit = rapier_context.cast_ray(ray_pos, ray_dir, f32::MAX, true, QueryFilter::new());

        if let Some((_entity, toi)) = hit {
            let hit_pos = ray_pos + ray_dir * toi;
            for (mut cursor, mut visibility) in &mut query {
                let (hex, elevation) = AxialCoordinates::from_world(hit_pos);
                cursor.translation = hex.to_world(elevation);
                visibility.is_visible = true;
            }
            if mouse_input.just_pressed(MouseButton::Left) {
                event_writer.send(ColliderClickedEvent(hit_pos));
            }
        }
        else {
            for (_cursor, mut visibility) in &mut query {
                visibility.is_visible = false;
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