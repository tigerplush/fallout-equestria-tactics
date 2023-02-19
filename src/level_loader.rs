use bevy::{asset::LoadState, prelude::*, render::primitives::Aabb};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, RapierColliderHandle};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::{common::Spawnpoint, resources::LevelName};

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

/// Loads a level and hooks unit components to the entities by name
pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_name: Res<LevelName>,
    mut loading: ResMut<AssetsLoading>,
) {
    let level_handle = asset_server.load(level_name.0.clone());

    loading.0.push(level_handle.clone_untyped());

    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: level_handle,
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                match entity
                    .get::<Name>()
                    .map(|t| t.as_str().split('.').collect::<Vec<&str>>()[0])
                {
                    Some("Spawnpoint") => cmds.insert(Spawnpoint),
                    _ => cmds,
                };
            }),
        })
        .insert(Name::from("Level"));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 10.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            color: Color::rgb_u8(255, 250, 240),
            ..default()
        },
        ..default()
    });

    info!("Level {} loaded", level_name.0);
}

/// Finds all Mesh-handles and adds a [`RapierColliderHandle`] to them based on the loaded meshes
pub fn add_collider(
    query: Query<(Entity, &Handle<Mesh>), Without<RapierColliderHandle>>,
    meshes: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, mesh) in &query {
        match asset_server.get_load_state(mesh) {
            LoadState::Loaded => {
                let collider = Collider::from_bevy_mesh(
                    meshes.get(mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap();
                commands.entity(entity).insert(collider);
            }
            _ => (),
        }
    }
}

/// adds cuboid colliders to all aabb components.
/// 
/// This is slightly more performant than [`add_collider`]
pub fn add_cuboid_collider(
    query: Query<(Entity, &Aabb), Without<RapierColliderHandle>>,
    mut commands: Commands,
) {
    for (entity, aabb) in &query {
        let coll = Collider::cuboid(aabb.half_extents.x, aabb.half_extents.y, aabb.half_extents.z);
        commands.entity(entity).insert(coll);
    }
}
