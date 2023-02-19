use bevy::{asset::LoadState, prelude::*};
use bevy_rapier3d::prelude::RapierColliderHandle;
use bevy_renet::renet::{DefaultChannel, RenetClient};

use fallout_equestria_tactics::level_loader::*;
use fallout_equestria_tactics::messages::ClientMessage;

use crate::common::ClientState;
use crate::init_plugin::Cursor;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading(Vec::new()));
        app.add_system_set(SystemSet::on_enter(ClientState::LoadingLevel).with_system(load_level));
        app.add_system_set(
            SystemSet::on_update(ClientState::LoadingLevel)
                .with_system(add_collider)
                .with_system(check_load_completed),
        );
        app.add_system_set(
            SystemSet::on_exit(ClientState::LoadingLevel).with_system(notify_server),
        );
        info!("LevelLoaderPlugin has been loaded");
    }
}

fn check_load_completed(
    query: Query<Entity, (With<Handle<Mesh>>, (Without<RapierColliderHandle>, Without<Cursor>))>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<ClientState>>,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            if query.is_empty() {
                info!("everything loaded");
                app_state.set(ClientState::LevelLoaded).unwrap();
            }
        }
        _ => (),
    }
}

fn notify_server(mut client: ResMut<RenetClient>) {
    info!("Notifying server");
    let message = bincode::serialize(&ClientMessage::LevelLoaded).unwrap();
    client.send_message(DefaultChannel::Reliable, message);
}
