use bevy::prelude::*;

mod game_flow;
pub use game_flow::{GameState, OnlyShowInGameState, RoundLog, RoundResult};

pub const ROUNDS_TO_WIN: usize = 5;
pub const PRE_ROUND_DURATION: f32 = 2.0;
pub const COMBAT_DURATION: f32 = 99.0;
pub const POST_ROUND_DURATION: f32 = 4.0;
pub const POST_SHOP_DURATION: f32 = 11.0;

#[derive(Reflect, Resource, Debug)]
pub struct Clock {
    pub frame: usize,
    start_time: f32,
    pub elapsed_time: f32,
}
impl FromWorld for Clock {
    fn from_world(world: &mut World) -> Self {
        Self {
            start_time: world.get_resource::<Time>().unwrap().elapsed_seconds(),
            frame: 0,
            elapsed_time: 0.0,
        }
    }
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

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WAGStage {
    Inputs,
    Physics,
    HitReg,
    PlayerUpdates,
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (WAGStage::Physics, WAGStage::HitReg, WAGStage::PlayerUpdates)
                .run_if(once_per_combat_frame),
        )
        .add_state::<GameState>()
        .add_systems(Update, update_visibility_on_state_change)
        .init_resource::<Clock>()
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / crate::FPS as f64))
        .add_systems(FixedUpdate, update_clock)
        .insert_resource(RoundLog::default());
    }
}

fn update_clock(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.frame += 1;
    clock.elapsed_time = bevy_clock.elapsed_seconds() - clock.start_time;
}

fn update_visibility_on_state_change(
    state: Res<State<GameState>>,
    mut query: Query<(&mut Visibility, &OnlyShowInGameState)>,
) {
    // TODO FIXME: This is broken, and happens on every frame which is not performant
    if state.is_changed() {
        for (mut visibility, restriction) in &mut query {
            *visibility = if restriction.contains(state.get()) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn in_combat(state: Res<State<GameState>>) -> bool {
    state.get() == &GameState::Combat
}

pub fn not_in_combat(state: Res<State<GameState>>) -> bool {
    state.get() != &GameState::Combat
}

pub fn once_per_combat_frame(
    mut last_frame: Local<usize>,
    clock: Res<Clock>,
    state: Res<State<GameState>>,
) -> bool {
    let value = state.get() == &GameState::Combat && *last_frame < clock.frame;
    *last_frame = clock.frame;
    value
}
