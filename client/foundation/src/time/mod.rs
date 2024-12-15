use std::time::Instant;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

mod game_flow;
pub use game_flow::{
    GameResult, GameState, InCharacterSelect, InMatch, LocalState, MatchState, OnlineState,
    RoundLog, RoundResult,
};

pub const ROUNDS_TO_WIN: usize = 3;
pub const PRE_ROUND_DURATION: f32 = 2.0;
pub const COMBAT_DURATION: f32 = 99.0;
pub const POST_ROUND_DURATION: f32 = 4.0;
pub const POST_SHOP_DURATION: f32 = 11.0;

#[derive(Reflect, Resource, Debug, Clone, Copy)]
pub struct Clock {
    pub frame: usize,
    start_time: f32,
    pub done: bool,
    pub timer_value: usize,
}
impl FromWorld for Clock {
    fn from_world(world: &mut World) -> Self {
        Self {
            start_time: world.get_resource::<Time>().unwrap().elapsed_secs(),
            frame: 0,
            done: false,
            timer_value: COMBAT_DURATION as usize,
        }
    }
}
impl Clock {
    pub fn reset(&mut self, time: f64) {
        self.frame = 0;
        self.done = false;
        self.start_time = time as f32;
    }
}

#[derive(Debug, Resource, Deref, Clone, Copy)]
pub struct Hitstop(pub Instant);

// This needs to be defined here because it gets used here
// It is a workaround that allows running the same systems in both online and offline
#[derive(Debug, ScheduleLabel, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RollbackSchedule;

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemStep {
    HouseKeeping,
    MenuNavigation,
    StateTransitions,
    Inputs,
    Physics,
    HitReg,
    MovePipeline,
    PlayerUpdates,
    ResourceUpdates,
    Presentation,
    HitStop,
    Camera,
    Final,
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            RollbackSchedule,
            (
                SystemStep::HouseKeeping,
                SystemStep::MenuNavigation,
                SystemStep::StateTransitions,
            )
                .chain()
                .before(SystemStep::Inputs),
        )
        .configure_sets(
            RollbackSchedule,
            (
                SystemStep::Physics,
                SystemStep::HitReg,
                SystemStep::MovePipeline,
                SystemStep::PlayerUpdates,
                SystemStep::ResourceUpdates,
            )
                .chain()
                .run_if(in_state(MatchState::Combat))
                .after(SystemStep::Inputs),
        )
        .configure_sets(
            RollbackSchedule,
            (
                SystemStep::Presentation,
                SystemStep::HitStop,
                SystemStep::Camera,
                SystemStep::Final,
            )
                .chain()
                .after(SystemStep::ResourceUpdates),
        )
        .init_resource::<Clock>()
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / crate::FPS as f64))
        .add_systems(
            RollbackSchedule,
            update_clock.in_set(SystemStep::HouseKeeping),
        )
        .add_systems(OnExit(MatchState::EndScreen), clear_round_log)
        .insert_resource(RoundLog::default());
    }
}

fn update_clock(
    mut clock: ResMut<Clock>,
    bevy_clock: Res<Time>,
    maybe_hitstop: Option<Res<Hitstop>>,
) {
    if maybe_hitstop.is_some() {
        return;
    }

    clock.frame += 1;

    if clock.done {
        return;
    }

    // This updates timer
    let elapsed = bevy_clock.elapsed_secs() - clock.start_time;
    clock.timer_value = (COMBAT_DURATION + PRE_ROUND_DURATION - elapsed)
        .clamp(0.0, COMBAT_DURATION)
        .ceil() as usize;
    clock.done = clock.timer_value == 0;
}

fn clear_round_log(mut log: ResMut<RoundLog>) {
    log.clear();
}
