use bevy::prelude::*;
use oops_all_grapplers::*;

fn main() {
    // Happens roughly in order, so add stages, click and assets before using them
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StagePlugin)
        .add_plugin(ClockPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
