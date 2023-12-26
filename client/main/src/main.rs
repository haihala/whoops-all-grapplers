// use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
// use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::{log::LogPlugin, prelude::*, window::WindowMode};
use wag_core::WagArgs;
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    let args = WagArgs::from_cli();

    // Happens roughly in order, so add stages, click and assets before using them
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wgpu=error,naga=warn,bevy_gltf::loader=error".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: if args.dev {
                            WindowMode::Windowed
                        } else {
                            WindowMode::BorderlessFullscreen
                        },
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WAGLib::with_args(args))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .run();
}
