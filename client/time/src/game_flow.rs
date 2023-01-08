use bevy::prelude::*;
use wag_core::Player;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Loading,
    Combat,
    PostRound,
    Shop,
}
impl GameState {
    pub fn next(self) -> GameState {
        match self {
            GameState::Loading => GameState::Combat,

            GameState::Combat => GameState::PostRound,
            GameState::PostRound => GameState::Shop,
            GameState::Shop => GameState::Combat,
        }
    }
}

#[derive(Debug, Resource)]
pub struct RoundResult {
    pub winner: Option<Player>,
}

#[derive(Debug, Component, Deref)]
pub struct OnlyShowInGameState(pub Vec<GameState>);
