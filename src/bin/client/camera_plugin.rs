use bevy::{prelude::*, input::mouse::MouseWheel};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ZoomSettings::default())
        .add_startup_system(spawn_camera)
        .add_system(move_camera)
        .add_system(set_zoom);
        info!("CameraPlugin has been loaded");
    }
}

fn spawn_camera(
    mut commands: Commands,
    zoom_settings: Res<ZoomSettings>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: zoom_settings.get_mapped(),
            near: -10.0,
            ..default()
        }),
        ..default()
    });
}

const MOVE_SPEED: f32 = 8.0;

#[derive(Resource)]
struct ZoomSettings {
    current_zoom: f32,
    min_zoom: f32,
    max_zoom: f32,
    zoom_speed: f32,
}

impl Default for ZoomSettings {
    fn default() -> Self {
        ZoomSettings {
            current_zoom: 0.01,
            min_zoom: 0.01,
            max_zoom: 0.04,
            zoom_speed: 2.0
        }
    }
}

impl ZoomSettings {
    pub fn add(&mut self, zoom: f32) {
        self.current_zoom = (self.current_zoom + zoom * self.zoom_speed).clamp(0.0, 1.0);
    }

    pub fn get_mapped(&self) -> f32 {
        self.min_zoom + (self.max_zoom - self.min_zoom) * self.current_zoom
    }
}

fn move_camera(
    key_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut current_zoom: ResMut<ZoomSettings>,
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

fn set_zoom(mut query: Query<&mut Projection>, current_zoom: Res<ZoomSettings>) {
    for mut projection in &mut query {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = current_zoom.get_mapped();
        }
    }
}
