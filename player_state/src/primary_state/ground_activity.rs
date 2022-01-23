use bevy_inspector_egui::Inspectable;
use types::{AbsoluteDirection, MoveId};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Move(MoveId),
    Walk(usize, AbsoluteDirection),
    PreJump {
        launch_frame: usize,
        direction: Option<AbsoluteDirection>,
        held: bool,
    },
    Crouching,
    Standing,
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
