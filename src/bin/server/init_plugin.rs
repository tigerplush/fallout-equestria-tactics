use std::{env::args, net::SocketAddr, time::Duration};

use bevy::{prelude::*, app::ScheduleRunnerSettings, ecs::schedule::ShouldRun};
use fallout_equestria_tactics::{resources::*, level_loader::AssetsLoading};

use crate::{common::ServerState, foe_server::FoEServer};

/// Initialises the server and loads a level
/// 
/// When initialisation is done, the server moves to [`ServerState::Lobby`]
pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
        info!("InitPlugin has been loaded");
    }
}

/// Default Server Adress.
/// 
/// Is overwritten by the first argument when starting the server
const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:5000";
/// Default level to load.
/// 
/// Is overwritten by the second argument when starting the server
const DEFAULT_LEVEL_NAME: &str = "level.gltf#Scene0";

/// Initialises all default values and inserts necessary resources for the server to start
fn init(
    mut commands: Commands,
    mut app_state: ResMut<State<ServerState>>
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
    commands.insert_resource(AssetsLoading(Vec::new()));
    commands.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 115.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    app_state.overwrite_set(ServerState::Lobby).unwrap();
}
