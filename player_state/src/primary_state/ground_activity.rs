use bevy_inspector_egui::Inspectable;

use kits::MoveSituation;
use types::Facing;

#[derive(Inspectable, PartialEq, Clone, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveSituation),
    Walk(Facing),
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
