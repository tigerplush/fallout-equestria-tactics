use bevy::{prelude::*, asset::LoadState};

use bevy_rapier3d::prelude::RapierColliderHandle;
use bevy_renet::renet::{RenetServer, DefaultChannel};
use fallout_equestria_tactics::{level_loader::{add_collider, AssetsLoading, load_level}, common::{Readiness, LevelLoaded}, messages::ServerMessage, resources::{LevelName, Players}};

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
            .with_system(check_for_level_loaded_and_readiness)
        );
        app.add_system_set(
            SystemSet::on_exit(ServerState::Lobby)
            .with_system(notify_clients)
        );
        app.add_system_set(
            SystemSet::on_update(ServerState::WaitingForPlayerLoadLevel)
            .with_system(check_for_players_level_loaded)
        );
        info!("LobbyPlugin has been loaded");
    }
}

fn check_for_level_loaded_and_readiness(
    readiness_query: Query<&Readiness>,
    collider_query: Query<Entity, (With<Handle<Mesh>>, Without<RapierColliderHandle>)>,
    mut app_state: ResMut<State<ServerState>>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    if readiness_query.iter().all(|r| r.0 == true) && !readiness_query.is_empty() {
        match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
            LoadState::Loaded => {
                if collider_query.is_empty() {
                    app_state.set(ServerState::WaitingForPlayerLoadLevel ).unwrap();
                }
            }
            _ => (),
        }
    }
}

fn notify_clients(
    mut server: ResMut<RenetServer>,
    level_name: Res<LevelName>,
) {
    let message = bincode::serialize(&ServerMessage::LoadLevel(level_name.0.clone())).unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message)
}

fn check_for_players_level_loaded(
    level_loaded_query: Query<&LevelLoaded>,
    mut app_state: ResMut<State<ServerState>>,
) {
    if level_loaded_query.iter().all(|r| r.0 == true) && !level_loaded_query.is_empty() {
        app_state.set(ServerState::SpawnPhase).unwrap();
    }
}
