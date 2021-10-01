// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use input_parsing::InputParsingPlugin;
use oops_all_grapplers_lib::*;

fn main() {
    // Happens roughly in order, so add stages, click and assets before using them
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin)
        .add_plugin(StagePlugin)
        .add_plugin(ClockPlugin)
        .add_plugin(InputParsingPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(BarsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
