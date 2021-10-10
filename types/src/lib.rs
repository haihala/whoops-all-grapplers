mod player_state;
pub use player_state::{AnimationState, PlayerState};
mod inputs;
pub use inputs::{GameButton, Normal, StickPosition};

use bevy_inspector_egui::Inspectable;
use std::fmt::{Debug, Display};

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.

pub type MoveType = u32;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Player {
    One,
    Two,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
