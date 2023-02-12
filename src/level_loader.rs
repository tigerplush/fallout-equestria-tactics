use bevy::{asset::LoadState, prelude::*};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, RapierColliderHandle};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::{common::Spawnpoint, resources::LevelName};

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

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

    info!("Level {} loaded", level_name.0);
}

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
