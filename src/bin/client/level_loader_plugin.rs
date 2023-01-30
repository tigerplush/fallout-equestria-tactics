use bevy::prelude::*;
use bevy_scene_hook::*;

use fallout_equestria_tactics::{common::Spawnpoint, resources::LevelName};

use crate::common::ClientState;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ClientState::LoadLevel).with_system(load_level)
        );
        app.add_system_set(
            SystemSet::on_update(ClientState::LoadLevel).with_system(update_state)
        );
        app.add_system_set(
            SystemSet::on_exit(ClientState::LoadLevel).with_system(notify_server)
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
}

fn update_state(
    query: Query<Entity, With<Spawnpoint>>,
    mut app_state: ResMut<State<ClientState>>,
) {
    for entity in &query {
        info!("spawnpoint detected {:?}", entity);
    }
    app_state.set(ClientState::Idling).unwrap();
}

fn notify_server() {

}