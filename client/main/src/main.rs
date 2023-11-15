// use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
// use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::{log::LogPlugin, prelude::*};
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    // Happens roughly in order, so add stages, click and assets before using them
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "wgpu=error,naga=warn,bevy_gltf::loader=error".to_string(),
            ..default()
        }))
        .add_plugins(WAGLib::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .run();
}
