mod stick_position;
use bevy::prelude::*;
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Reflect, Default)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    #[default]
    Default, // To satisfy Inspectable

    Start,

    Fast,
    Strong,
    Wrestling,
    Gimmick,
}
