use bevy::prelude::Resource;
use wag_core::Player;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Combat,
    Shop,
}

#[derive(Debug, Resource)]
pub struct RoundResult {
    pub winner: Option<Player>,
}
