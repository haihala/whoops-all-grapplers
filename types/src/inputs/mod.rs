mod stick_position;
use bevy_inspector_egui::Inspectable;
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Inspectable)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    Default, // To satisfy Inspectable

    Grab,
    Strong,
    Fast,
    Equipment,
    Taunt,
}
impl Default for GameButton {
    fn default() -> Self {
        GameButton::Default
    }
}
