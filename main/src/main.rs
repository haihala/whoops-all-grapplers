// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use input_parsing::InputParsingPlugin;
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    // Happens roughly in order, so add stages, click and assets before using them
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugins(WAGLib)
        .add_plugin(InputParsingPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
