use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use bevy::{prelude::*, winit::WinitPlugin};
use input_parsing::testing::{parse_input, PreWrittenStream};
use types::Player;
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
        let ticks = spec.len();
        println!("Starting test case '{}' ({} ticks)", case_name, ticks);

        let mut app = self.setup(spec);
        self.simulate(&mut app, ticks);
        AppWrapper::new(app)
    }

    /// Setup the game env for a test case
    fn setup(&self, spec: TestSpec) -> App {
        let mut app = App::new();
        app.add_plugins_with(DefaultPlugins, |group| {
            group.disable::<WinitPlugin>();

            group
        });
        app.add_plugins(WAGLib);
        app.add_system(parse_input::<PreWrittenStream>);
        app.update();

        // Go to combat (skip buy phase)
        let mut key_input = app.world.resource_mut::<Input<KeyCode>>();
        key_input.press(KeyCode::Return);
        app.update();

        let mut p1: Option<Entity> = None;
        let mut p2: Option<Entity> = None;

        for (entity, player) in app.world.query::<(Entity, &Player)>().iter(&app.world) {
            match player {
                Player::One => {
                    p1 = Some(entity);
                }
                Player::Two => {
                    p2 = Some(entity);
                }
            }
        }

        app.world
            .entity_mut(p1.unwrap())
            .insert_bundle(spec.p1_bundle());
        app.world
            .entity_mut(p2.unwrap())
            .insert_bundle(spec.p2_bundle());

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
                (1.0 / constants::FPS - pre_update.elapsed().as_secs_f32()).max(0.0),
            ))
        }
    }
}
