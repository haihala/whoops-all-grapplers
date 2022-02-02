use bevy_inspector_egui::Inspectable;

use crate::MoveState;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum AirActivity {
    Freefall,
    Move(MoveState),
    Idle,
}

impl Default for AirActivity {
    fn default() -> Self {
        AirActivity::Idle
    }
}
