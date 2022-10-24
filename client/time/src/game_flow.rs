use wag_core::Player;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Combat,
    Shop,
}

pub struct RoundResult {
    pub winner: Option<Player>,
}
