use bevy_inspector_egui::Inspectable;
use types::{MoveId, RelativeDirection};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveId),
    Walk(usize, RelativeDirection),
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
