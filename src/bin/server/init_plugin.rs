use std::{env::args, net::SocketAddr, time::Duration};

use bevy::{prelude::*, app::ScheduleRunnerSettings};
use fallout_equestria_tactics::resources::*;

use crate::{common::ServerState, foe_server::FoEServer};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
        app.add_system_set(
            SystemSet::on_update(ServerState::Init)
            .with_system(on_update)
        );
        app.add_system_set(
            SystemSet::on_exit(ServerState::Init)
            .with_system(on_exit)
        );
        info!("InitPlugin has been loaded");
    }
}

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:5000";
const DEFAULT_LEVEL_NAME: &str = "level.gltf#Scene0";

fn init(
    mut commands: Commands,
    mut app_state: ResMut<State<ServerState>>,
) {
    let args: Vec<String> = args().collect();
    let server_address = if args.len() > 1 {
        // first argument is server address
        let address = &args[1];
        if let Ok(server_address) = address.parse::<SocketAddr>() {
            server_address
        } else {
            DEFAULT_SERVER_ADDRESS.parse().unwrap()
        }
    } else {
        DEFAULT_SERVER_ADDRESS.parse().unwrap()
    };
    commands.insert_resource(FoEServer::new(server_address));
    let level_name = if args.len() > 2 {
        &args[2]
    } else {
        DEFAULT_LEVEL_NAME
    };
    commands.insert_resource(LevelName::new(level_name));
    commands.insert_resource(Players::new());
    commands.insert_resource(TurnOrder::new());
    commands.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));

    app_state.overwrite_set(ServerState::Lobby).unwrap();
}

fn on_update() {

}

fn on_exit() {

}