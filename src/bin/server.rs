use bevy::{prelude::*, log::LogPlugin};
use bevy_turborand::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use fallout_equestria_tactics::assets::Names;
use fallout_equestria_tactics::foe_server_plugin::FoEServerPlugin;
use fallout_equestria_tactics::resources::NamesHandle;


fn main() {
    let mut app = App::new();
    app
    .add_plugins(MinimalPlugins)
    .add_plugin(AssetPlugin::default())
    .add_plugin(JsonAssetPlugin::<Names>::new(&["json.names"]))
    .add_plugin(LogPlugin::default())
    .add_plugin(FoEServerPlugin)
    .add_plugin(RngPlugin::default())
    .add_startup_system(load_names);

    app.run();
}

fn load_names(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let names_handle: Handle<Names> = asset_server.load("names.json.names");
    commands.insert_resource(NamesHandle(names_handle));
}
