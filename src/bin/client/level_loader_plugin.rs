use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, DefaultChannel};
use bevy_scene_hook::*;

use fallout_equestria_tactics::{common::Spawnpoint, resources::LevelName, messages::ClientMessage};

use crate::common::ClientState;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ClientState::LoadingLevel).with_system(load_level)
        );
        app.add_system_set(
            SystemSet::on_update(ClientState::LoadingLevel).with_system(update_state)
        );
        app.add_system_set(
            SystemSet::on_exit(ClientState::LoadingLevel).with_system(notify_server)
        );
        info!("LevelLoaderPlugin has been loaded");
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_name: Res<LevelName>,
) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load(level_name.0.clone()),
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

fn update_state(
    query: Query<&Spawnpoint>,
    mut app_state: ResMut<State<ClientState>>,
) {
    for _spawnpoint in &query {
        info!("spawnpoint detected");
    }
    if !query.is_empty() {
        info!("Exiting loading level state");
        app_state.set(ClientState::LevelLoaded).unwrap();
    }
}

fn notify_server(
    mut client: ResMut<RenetClient>,
) {
    info!("Notifying server");
    let message = bincode::serialize(&ClientMessage::LevelLoaded).unwrap();
    client.send_message(DefaultChannel::Reliable, message);
}