use bevy::prelude::*;
use oops_all_grapplers::*;

fn main() {
    App::build()
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
