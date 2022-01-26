mod stick_position;
pub use stick_position::StickPosition;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    Grab,
    Heavy,
    Light,
}
