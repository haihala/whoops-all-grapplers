mod stick_position;
pub use stick_position::StickPosition;
mod motion_input;
pub use motion_input::MotionInput;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    Heavy,
    Fast,
}

/// I.E. Quarter circle forward press punch -> fireball
pub struct Special {
    pub motion: MotionInput,
    pub button: Option<GameButton>,
}
impl Special {
    pub fn clear(&mut self) {
        self.motion.clear();
    }
}

pub struct Normal {
    pub button: GameButton,
    pub stick: Option<StickPosition>,
}
