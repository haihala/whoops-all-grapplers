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
    pub elapsed_time: f32,
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

#[derive(Debug, StageLabel, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WAGStage {
    Inputs,
    Physics,
    HitReg,
}

#[derive(Debug, SystemLabel, Hash, PartialEq, Eq, Clone, Copy)]
enum TimeSystemLabel {
    UpdateClock,
    ResetClock,
    UpdateCountdown,
    ResetCountdown,
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate,
            WAGStage::Inputs,
            SystemStage::parallel(),
        )
        .add_stage_after(CoreStage::Update, WAGStage::HitReg, SystemStage::parallel())
        .add_stage_before(
            CoreStage::PostUpdate,
            WAGStage::Physics,
            SystemStage::parallel(),
        )
        .insert_resource(Clock::default())
        .add_state_to_stage(CoreStage::Last, GameState::PreRound)
        .add_system_set_to_stage(CoreStage::PostUpdate, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::Update, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::PreUpdate, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::First, State::<GameState>::get_driver())
        .insert_resource(game_flow::InterFrameCountdown(Timer::from_seconds(
            3.0, false,
        )))
        .add_system_to_stage(
            CoreStage::First,
            update_clock
                .with_run_criteria(FixedTimestep::steps_per_second(constants::FPS as f64))
                .label(TimeSystemLabel::UpdateClock),
        )
        .add_system_to_stage(
            CoreStage::First,
            reset_clock
                .with_run_criteria(State::on_enter(GameState::Combat))
                .label(TimeSystemLabel::ResetClock)
                .after(TimeSystemLabel::UpdateClock),
        )
        .add_system_to_stage(
            CoreStage::First,
            game_flow::update_countdown
                .with_run_criteria(not_in_combat)
                .label(TimeSystemLabel::UpdateCountdown)
                .after(TimeSystemLabel::ResetClock),
        )
        .add_system_to_stage(
            CoreStage::First,
            game_flow::reset_countdown
                .with_run_criteria(State::on_enter(GameState::PostRound))
                .label(TimeSystemLabel::ResetCountdown)
                .after(TimeSystemLabel::UpdateCountdown),
        );
    }
}

fn update_clock(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.frame += 1;
    clock.elapsed_time += bevy_clock.delta_seconds();
}

fn reset_clock(mut clock: ResMut<Clock>) {
    clock.reset();
}

pub fn not_in_combat(state: Res<State<GameState>>) -> ShouldRun {
    if *state.current() != GameState::Combat {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn once_per_combat_frame(
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
