use bevy_inspector_egui::Inspectable;
use types::LRDirection;

use crate::MoveState;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveState),
    Walk(LRDirection),
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
