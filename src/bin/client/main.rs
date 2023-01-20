use bevy::prelude::*;

mod client_plugin;
use client_plugin::*;

mod common;
use common::ClientState;

mod gui_plugin;
use gui_plugin::GuiPlugin;

fn main() {
    App::new()
        .add_state(ClientState::WaitingToConnect)
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(GuiPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}