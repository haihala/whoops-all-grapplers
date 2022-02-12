use bevy_inspector_egui::Inspectable;

use moves::MoveState;
use types::LRDirection;

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
