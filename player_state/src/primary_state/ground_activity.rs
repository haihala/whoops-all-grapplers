use bevy_inspector_egui::Inspectable;

use kits::MoveSituation;
use types::LRDirection;

#[derive(Inspectable, PartialEq, Clone, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveSituation),
    Walk(LRDirection),
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
