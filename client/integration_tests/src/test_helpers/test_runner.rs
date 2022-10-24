use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use bevy::{app::ScheduleRunnerSettings, asset::AssetPlugin, input::InputPlugin, prelude::*};
use bevy_hanabi::HanabiPlugin;
use input_parsing::testing::{parse_input, PreWrittenStream};
use wag_core::Players;
use whoops_all_grapplers_lib::{DevPlugin, WAGLib};

use super::{AppWrapper, TestSpec};

/// A framework that runs through a list of specs from a common starting position.
pub struct TestRunner {
    // TODO: Starting scenario
    // TODO: Several tests per runner
}
impl TestRunner {
    /// Setup the game env for a test case
    pub fn new() -> Self {
        // TODO: Scenario data for setup
        Self {}
    }

    /// Run a spec, return the world
    pub fn run(&mut self, case_name: &str, spec: TestSpec) -> AppWrapper {
        let ticks = spec.len;
        println!("Starting test case '{case_name}' ({ticks} ticks)");

        let mut app = self.setup(spec);
        self.simulate(&mut app, ticks);
        AppWrapper::new(app)
    }

    /// Setup the game env for a test case
    fn setup(&self, spec: TestSpec) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ScheduleRunnerSettings {
            run_mode: bevy::app::RunMode::Loop { wait: None },
        });
        app.add_plugin(AssetPlugin::default());
        app.add_plugin(InputPlugin::default());

        app.add_plugins_with(WAGLib, |group| {
            group.disable::<HanabiPlugin>();
            group.disable::<DevPlugin>();
            group
        });
        app.add_system(parse_input::<PreWrittenStream>);
        app.update();

        // Go to combat (skip buy phase)
        app.world
            .resource_mut::<Input<KeyCode>>()
            .press(KeyCode::Return);
        app.update();

        let players = app.world.resource::<Players>();
        let p1 = players.one;
        let p2 = players.two;
        drop(players); // Needs to drop because couldn't figure out how to get the Players resource without by value.

        app.world.entity_mut(p1).insert_bundle(spec.p1_bundle);
        app.world.entity_mut(p2).insert_bundle(spec.p2_bundle);

        app.update();
        app
    }

    /// Run the spec inputs
    fn simulate(&self, app: &mut App, ticks: usize) {
        for _ in 0..ticks {
            let pre_update = Instant::now();
            app.update();
            // Must sleep real time here, as bevy clock doesn't care about our fake time
            sleep(Duration::from_secs_f32(
                (1.0 / wag_core::FPS - pre_update.elapsed().as_secs_f32()).max(0.0),
            ))
        }
    }
}
