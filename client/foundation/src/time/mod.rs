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
pub const COMBAT_DURATION: f32 = 100.0;
pub const MAX_COMBAT_DURATION: f32 = COMBAT_DURATION + PRE_ROUND_DURATION;
pub const POST_ROUND_DURATION: f32 = 4.0;
pub const POST_SHOP_DURATION: f32 = 11.0;

#[derive(Reflect, Resource, Debug, Clone, Copy, Default)]
pub struct Clock {
    pub frame: usize,      // This will always tick every frame
    pub base_frame: usize, // This points to the previous round start
}
impl Clock {
    pub fn reset(&mut self) {
        self.base_frame = self.frame;
    }

    pub fn relative_frame(&self) -> usize {
        self.frame - self.base_frame
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
    EntityManagement,
    Menus,
    StateTransitions,
    Inputs,
    SideSwitch,
    Pickups,
    Conditions,
    SpawnPlayers,
    RoundReset,
    Movement,
    HitReg,
    MovePipeline,
    Recovery,
    PlayerUpdates,
    Economy,
    Shop,
    Presentation,
    UI,
    HitStop,
    Camera,
    SetupStage,
    DevTools,
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            RollbackSchedule,
            (
                (
                    SystemStep::Clock,
                    SystemStep::EntityManagement,
                    SystemStep::Menus,
                    SystemStep::Conditions,
                    SystemStep::SpawnPlayers,
                    SystemStep::RoundReset,
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
                    SystemStep::Recovery,
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
                    SystemStep::SetupStage,
                    SystemStep::DevTools,
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
        .insert_resource(RoundLog::default());
    }
}

fn global_clock_update(mut clock: ResMut<Clock>) {
    dbg!(&clock);
    clock.frame += 1;
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
