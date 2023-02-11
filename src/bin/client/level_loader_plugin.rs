use bevy::{prelude::*, asset::LoadState};
use bevy_rapier3d::prelude::{ComputedColliderShape, Collider, RapierColliderHandle};
use bevy_renet::renet::{RenetClient, DefaultChannel};
use bevy_scene_hook::*;

use fallout_equestria_tactics::{common::Spawnpoint, resources::LevelName, messages::ClientMessage};

use crate::common::ClientState;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading(Vec::new()));
        app.add_system_set(
            SystemSet::on_enter(ClientState::LoadingLevel)
                .with_system(load_level)
                .with_system(check_load_completed)
        );
        app.add_system_set(
            SystemSet::on_update(ClientState::LoadingLevel)
                .with_system(add_collider)
        );
        app.add_system_set(
            SystemSet::on_exit(ClientState::LoadingLevel).with_system(notify_server)
        );
        info!("LevelLoaderPlugin has been loaded");
    }
}

#[derive(Resource)]
struct AssetsLoading(Vec<HandleUntyped>);

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_name: Res<LevelName>,
    mut loading: ResMut<AssetsLoading>,
) {
    let level_handle = asset_server.load(level_name.0.clone());

    loading.0.push(level_handle.clone_untyped());

    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: level_handle,
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

    info!("Level {} loaded", level_name.0);
}

fn add_collider(
    query: Query<(Entity, &Handle<Mesh>), Without<RapierColliderHandle>>,
    meshes: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
)
{
    for (entity, mesh) in &query {
        match asset_server.get_load_state(mesh) {
            LoadState::Loaded => {
                let collider = Collider::from_bevy_mesh(
                    meshes.get(mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                ).unwrap();
                commands.entity(entity).insert(collider);
            }
            _ => ()
        }
    }
}

fn check_load_completed(
    query: Query<Entity, (With<Handle<Mesh>>, Without<RapierColliderHandle>)>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<ClientState>>
)
{
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            if query.is_empty() {
                info!("everything loaded");
                app_state.set(ClientState::LevelLoaded).unwrap();
            }
        }
        _ => ()
    }
}

fn notify_server(
    mut client: ResMut<RenetClient>,
) {
    info!("Notifying server");
    let message = bincode::serialize(&ClientMessage::LevelLoaded).unwrap();
    client.send_message(DefaultChannel::Reliable, message);
}