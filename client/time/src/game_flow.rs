use bevy::prelude::Resource;
use wag_core::Player;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Loading,
    Combat,
    Shop,
}
impl GameState {
    pub fn next(self) -> GameState {
        match self {
            GameState::Loading => GameState::Combat,
            GameState::Combat => GameState::Shop,
            GameState::Shop => GameState::Combat,
        }
    }
}

#[derive(Debug, Resource)]
pub struct RoundResult {
    pub winner: Option<Player>,
}
