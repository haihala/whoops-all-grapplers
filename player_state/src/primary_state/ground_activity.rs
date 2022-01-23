use bevy_inspector_egui::Inspectable;
use types::{AbsoluteDirection, MoveId};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveId),
    Walk(usize, AbsoluteDirection),
    PreJump(usize),
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
