// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    window::WindowMode,
};
use wag_core::WagArgs;
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    let args = WagArgs::from_cli();

    // Happens roughly in order, so add stages, click and assets before using them
    App::new()
        .edit_schedule(Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Error,
                hierarchy_detection: LogLevel::Error,
                ..default()
            });
        })
        .add_plugins(
            DefaultPlugins
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
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin::default(),
        // ))
        .run();
}