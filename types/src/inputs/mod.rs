mod stick_position;
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    Grab,
    Strong,
    Fast,
    Equipment,
    Taunt,
}
