use bevy::{prelude::*, asset::LoadState};

use bevy_rapier3d::prelude::RapierColliderHandle;
use fallout_equestria_tactics::level_loader::{add_collider, AssetsLoading, load_level};

use crate::common::ServerState;
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ServerState::Lobby)
            .with_system(load_level)
        );
        app.add_system_set(
            SystemSet::on_update(ServerState::Lobby)
            .with_system(add_collider)
            //.with_system(check_for_level_loaded)
        );
        info!("LobbyPlugin has been loaded");
    }
}

fn check_for_level_loaded(
    collider_query: Query<Entity, (With<Handle<Mesh>>, Without<RapierColliderHandle>)>,
    mut app_state: ResMut<State<ServerState>>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            if collider_query.is_empty() {
                app_state.set(ServerState::Lobby).unwrap();
            }
        }
        _ => (),
    }
}