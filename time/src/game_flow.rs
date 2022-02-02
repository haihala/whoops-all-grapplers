use bevy::prelude::*;
use types::Player;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Combat,
    PreRound,
    PostRound,
}

pub struct InterFrameCountdown(pub Timer);

pub struct RoundResult {
    pub winner: Option<Player>,
}

pub fn restart_countdown(mut countdown: ResMut<InterFrameCountdown>) {
    countdown.0.reset();
}

pub fn tick_countdown(
    mut countdown: ResMut<InterFrameCountdown>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,
) {
    countdown.0.tick(time.delta());
    if countdown.0.finished() {
        state.set(GameState::Combat).unwrap();
    }
}
