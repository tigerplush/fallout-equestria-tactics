use bevy::{prelude::*, log::LogPlugin};
use fallout_equestria_tactics::foe_server_plugin::FoEServerPlugin;

fn main() {
    let mut app = App::new();
    app
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin::default())
    .add_plugin(FoEServerPlugin);

    app.run();
}
