// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use foundation::WagArgs;
use whoops_all_grapplers_lib::Lib;

fn main() {
    let args = WagArgs::from_cli();
    let base_size = 30.0;

    // Happens roughly in order, so add stages, click and assets before using them
    App::new()
        // TODO: Fix that
        //
        //.edit_schedule(Update, |schedule| {
        //    schedule.set_build_settings(ScheduleBuildSettings {
        //        ambiguity_detection: LogLevel::Error,
        //        hierarchy_detection: LogLevel::Error,
        //        ..default()
        //    });
        //})
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(if args.dev.is_some() {
                Window {
                    mode: WindowMode::Windowed,
                    resizable: true,
                    resolution: WindowResolution::new(base_size * 16.0, base_size * 9.0),
                    ..default()
                }
            } else {
                Window {
                    mode: WindowMode::BorderlessFullscreen,
                    resizable: false,
                    ..default()
                }
            }),
            ..default()
        }))
        .add_plugins(Lib::with_args(args))
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin::default(),
        // ))
        .run();
}
