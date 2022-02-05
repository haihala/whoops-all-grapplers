use bevy::prelude::*;
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun};
use bevy_inspector_egui::Inspectable;

mod game_flow;
pub use game_flow::{GameState, RoundResult};

pub const ROUND_TIME: f32 = 99.0;

/// The component for measuring time in frames
#[derive(Inspectable, Default)]
pub struct Clock {
    pub frame: usize,
    elapsed_time: f32,
}
impl Clock {
    pub fn time_out(&self) -> bool {
        self.elapsed_time >= ROUND_TIME - 1.0
    }

    fn reset(&mut self) {
        self.frame = 0;
        self.elapsed_time = 0.0;
    }
}
/// The component for the round timer
#[derive(Debug, Component)]
pub struct RoundTimer;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Clock::default())
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(constants::FPS as f64))
                    .with_system(tick),
            )
            .add_system_set(SystemSet::on_update(GameState::Combat).with_system(update_timer))
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(reset_timer))
            .add_system_set(
                SystemSet::on_enter(GameState::PostRound).with_system(game_flow::restart_countdown),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PostRound).with_system(game_flow::tick_countdown),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PreRound).with_system(game_flow::tick_countdown),
            )
            .add_state(GameState::PreRound)
            .insert_resource(game_flow::InterFrameCountdown(Timer::from_seconds(
                3.0, false,
            )));
    }
}

fn tick(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.frame += 1;
    clock.elapsed_time += bevy_clock.delta_seconds();
}

fn update_timer(mut query: Query<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.single_mut().sections[0].value = (ROUND_TIME - clock.elapsed_time).floor().to_string();
}

fn reset_timer(mut clock: ResMut<Clock>) {
    clock.reset();
}

pub fn run_max_once_per_combat_frame(
    mut last_frame: Local<usize>,
    clock: Res<Clock>,
    state: Res<State<GameState>>,
) -> ShouldRun {
    if *state.current() == GameState::Combat && *last_frame < clock.frame {
        *last_frame = clock.frame;
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}