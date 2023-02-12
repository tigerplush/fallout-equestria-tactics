use std::collections::VecDeque;

use bevy::{asset::LoadState, prelude::*};

use bevy_rapier3d::prelude::RapierColliderHandle;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use fallout_equestria_tactics::{
    common::{LevelLoaded, Readiness},
    level_loader::*,
    messages::ServerMessage,
    resources::{LevelName, Players, TurnOrder},
};
use rand::seq::SliceRandom;

use crate::common::ServerState;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading(Vec::new()));
        app.add_system_set(
            SystemSet::on_enter(ServerState::WaitingForPlayerLoadLevel)
                .with_system(load_level)
                .with_system(notify_players_load_level),
        );
        app.add_system_set(
            SystemSet::on_update(ServerState::WaitingForPlayerLoadLevel)
                .with_system(add_collider)
                .with_system(handle_level_loaded),
        );
        app.add_system_set(SystemSet::on_exit(ServerState::WaitingForPlayerLoadLevel));
        info!("LevelLoaderPlugin has been loaded");
    }
}

fn notify_players_load_level(
    mut server: ResMut<RenetServer>,
    query: Query<Entity, With<Readiness>>,
    mut commands: Commands,
    level_name: Res<LevelName>,
) {
    info!("Players should load level: {}", level_name.0);

    for entity in &query {
        commands
            .entity(entity)
            .remove::<Readiness>()
            .insert(LevelLoaded(false));
    }

    let message = bincode::serialize(&ServerMessage::LoadLevel(level_name.0.clone())).unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message);
}

fn handle_level_loaded(
    query: Query<&LevelLoaded>,
    collider_query: Query<Entity, (With<Handle<Mesh>>, Without<RapierColliderHandle>)>,
    mut app_state: ResMut<State<ServerState>>,
    players: Res<Players>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut turn_order: ResMut<TurnOrder>,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            if collider_query.is_empty() && query.iter().all(|r| r.0 == true) && !query.is_empty() {
                info!("everything loaded, all players report level loaded");

                let mut rng = rand::thread_rng();
                let mut random_order: Vec<u64> = players.players.keys().map(|f| *f).collect();
                random_order.shuffle(&mut rng);
                turn_order.order = VecDeque::from(random_order);

                app_state.set(ServerState::SpawnPhase).unwrap();
            }
        }
        _ => (),
    }
}
