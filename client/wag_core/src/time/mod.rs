use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};

mod game_flow;
pub use game_flow::{GameState, OnlyShowInGameState, RoundLog, RoundResult};

pub const ROUNDS_TO_WIN: usize = 5;
pub const COMBAT_DURATION: f32 = 99.0;
pub const POST_ROUND_DURATION: f32 = 4.0;

/// The component for measuring time in frames
#[derive(Reflect, Default, Resource)]
pub struct Clock {
    pub frame: usize,
    start_time: f32,
    pub elapsed_time: f32,
}
impl Clock {
    pub fn done(&self) -> bool {
        self.elapsed_time >= COMBAT_DURATION - 1.0
    }

    pub fn time_out(&mut self) {
        self.elapsed_time = COMBAT_DURATION;
    }

    pub fn reset(&mut self, time: f64) {
        self.frame = 0;
        self.elapsed_time = 0.0;
        self.start_time = time as f32;
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
        .add_system(update_visibility_on_state_change)
        .add_state_to_stage(CoreStage::Last, GameState::Loading)
        .add_system_set_to_stage(CoreStage::PostUpdate, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::Update, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::PreUpdate, State::<GameState>::get_driver())
        .add_system_set_to_stage(CoreStage::First, State::<GameState>::get_driver())
        .add_system_set_to_stage(WAGStage::HitReg, State::<GameState>::get_driver())
        .add_system_set_to_stage(WAGStage::Inputs, State::<GameState>::get_driver())
        .add_system_set_to_stage(WAGStage::Physics, State::<GameState>::get_driver())
        .add_system_to_stage(
            CoreStage::First,
            update_clock.with_run_criteria(FixedTimestep::steps_per_second(crate::FPS as f64)),
        )
        .add_system_to_stage(
            CoreStage::First,
            reset_clock
                .with_run_criteria(State::on_enter(GameState::Combat))
                .after(update_clock),
        )
        .insert_resource(RoundLog::default());
    }
}

fn update_clock(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.frame += 1;
    clock.elapsed_time = bevy_clock.elapsed_seconds() - clock.start_time;
}

fn reset_clock(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.reset(bevy_clock.elapsed_seconds_f64());
}

fn update_visibility_on_state_change(
    state: Res<State<GameState>>,
    mut query: Query<(&mut Visibility, &OnlyShowInGameState)>,
) {
    // TODO FIXME: This is broken, and happens on every frame which is not performant
    if state.is_changed() {
        let new_state = state.current();
        for (mut visibility, restriction) in &mut query {
            visibility.is_visible = restriction.contains(new_state);
        }
    }
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
    let value = if *state.current() == GameState::Combat && *last_frame < clock.frame {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    };
    *last_frame = clock.frame;
    value
}
