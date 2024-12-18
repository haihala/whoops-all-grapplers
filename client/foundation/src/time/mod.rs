use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

mod game_flow;
pub use game_flow::{
    GameResult, GameState, InCharacterSelect, InMatch, LocalState, MatchState, OnlineState,
    RoundLog, RoundResult,
};

pub const FPS: f32 = 60.0;
pub const ON_BLOCK_HITSTOP: usize = 4;
pub const ON_HIT_HITSTOP: usize = 8;
pub const ON_THROW_HITSTOP: usize = 12;

pub const ROUNDS_TO_WIN: usize = 3;
pub const PRE_ROUND_DURATION: f32 = 2.0;
pub const COMBAT_DURATION: f32 = 99.0;
pub const POST_ROUND_DURATION: f32 = 4.0;
pub const POST_SHOP_DURATION: f32 = 11.0;

#[derive(Reflect, Resource, Debug, Clone, Copy)]
pub struct Clock {
    pub frame: usize,
    pub done: bool,
    pub timer_value: usize,
}
impl FromWorld for Clock {
    fn from_world(_world: &mut World) -> Self {
        Self {
            frame: 0,
            done: false,
            timer_value: COMBAT_DURATION as usize,
        }
    }
}
impl Clock {
    pub fn reset(&mut self) {
        self.frame = 0;
        self.done = false;
    }
}

#[derive(Reflect, Component, Debug, Clone, Copy, Default)]
pub struct CharacterClock {
    pub frame: usize,
    pub move_activation_processed: bool,
    pub move_events_processed: bool,
}

impl CharacterClock {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Component, Deref, Clone, Copy)]
pub struct Hitstop(pub usize);

// This needs to be defined here because it gets used here
// It is a workaround that allows running the same systems in both online and offline
#[derive(Debug, ScheduleLabel, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RollbackSchedule;

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemStep {
    Clock,
    Visibility,
    Menus,
    StateTransitions,
    Inputs,
    SideSwitch,
    Pickups,
    Conditions,
    Movement,
    HitReg,
    MovePipeline,
    PlayerUpdates,
    Economy,
    Shop,
    Presentation,
    UI,
    HitStop,
    Camera,
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            RollbackSchedule,
            (
                (
                    SystemStep::Clock,
                    SystemStep::Visibility,
                    SystemStep::Menus,
                    SystemStep::Conditions,
                    SystemStep::StateTransitions,
                    SystemStep::Inputs,
                )
                    .chain(),
                (
                    SystemStep::SideSwitch,
                    SystemStep::Pickups,
                    SystemStep::Movement,
                    SystemStep::HitReg,
                    SystemStep::MovePipeline,
                    SystemStep::PlayerUpdates,
                    SystemStep::Economy,
                )
                    .chain()
                    .run_if(in_state(MatchState::Combat)),
                (
                    SystemStep::Shop,
                    SystemStep::Presentation,
                    SystemStep::UI,
                    SystemStep::HitStop,
                    SystemStep::Camera,
                )
                    .chain(),
            )
                .chain(),
        )
        .init_resource::<Clock>()
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / crate::FPS as f64))
        .add_systems(
            RollbackSchedule,
            (global_clock_update, character_clock_update).in_set(SystemStep::Clock),
        )
        .add_systems(OnExit(MatchState::EndScreen), clear_round_log)
        .insert_resource(RoundLog::default());
    }
}

fn global_clock_update(mut clock: ResMut<Clock>) {
    clock.frame += 1;

    if clock.done {
        return;
    }

    // This updates timer
    let elapsed = clock.frame as f32 / FPS;
    clock.timer_value = (COMBAT_DURATION + PRE_ROUND_DURATION - elapsed)
        .clamp(0.0, COMBAT_DURATION)
        .ceil() as usize;
    clock.done = clock.timer_value == 0;
}

fn character_clock_update(mut query: Query<(&mut CharacterClock, Option<&Hitstop>)>) {
    for (mut clock, maybe_hitstop) in &mut query {
        if maybe_hitstop.is_none() {
            clock.frame += 1;
            clock.move_activation_processed = false;
            clock.move_events_processed = false;
        }
    }
}

fn clear_round_log(mut log: ResMut<RoundLog>) {
    log.clear();
}
