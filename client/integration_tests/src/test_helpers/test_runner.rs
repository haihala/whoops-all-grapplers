use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    asset::AssetPlugin,
    input::InputPlugin,
    prelude::*,
    render::RenderPlugin,
};

use input_parsing::testing::{parse_input, PreWrittenStream};
use wag_core::{GameState, Players};
use whoops_all_grapplers_lib::WAGLib;

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
        app.add_plugins(MinimalPlugins.build().disable::<ScheduleRunnerPlugin>());

        app.add_plugins((
            ScheduleRunnerPlugin {
                run_mode: RunMode::Loop { wait: None },
            },
            AssetPlugin::default(),
            WindowPlugin::default(),
            InputPlugin::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            WAGLib::integration().build(),
        ));

        app.add_systems(Update, parse_input::<PreWrittenStream>);
        app.update();

        // Go to combat
        app.world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Combat);
        app.update();

        let players = app.world.resource::<Players>();
        let p1 = players.one;
        let p2 = players.two;
        // TODO: Migration to bevy 0.11.2 broke this
        // drop(players); // Needs to drop because couldn't figure out how to get the Players resource without by value.

        app.world.entity_mut(p1).insert(spec.p1_bundle);
        app.world.entity_mut(p2).insert(spec.p2_bundle);

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
