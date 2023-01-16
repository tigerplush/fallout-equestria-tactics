use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use fallout_equestria_tactics::foe_server_plugin::FoEServerPlugin;

fn main() {
    let mut app = App::new();
    app
    .add_plugins(MinimalPlugins)
    .add_plugin(RenetServerPlugin::default())
    .add_plugin(FoEServerPlugin);

    app.run();
}
