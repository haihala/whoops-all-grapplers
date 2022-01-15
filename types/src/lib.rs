mod combat;
pub use combat::*;

mod direction;
pub use direction::*;

mod inputs;
pub use inputs::{GameButton, StickPosition};

use bevy_inspector_egui::Inspectable;
use std::fmt::{Debug, Display};

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.

pub type MoveId = u32;

#[derive(EnumIter, Inspectable, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Player {
    One,
    Two,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Inspectable, Default)]
pub struct HeightWindow {
    pub top: f32,
    pub bottom: f32,
}
