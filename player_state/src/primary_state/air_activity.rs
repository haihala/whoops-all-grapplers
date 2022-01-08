use bevy_inspector_egui::Inspectable;
use types::MoveId;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum AirActivity {
    Freefall,
    Move(MoveId),
    Idle,
}

impl Default for AirActivity {
    fn default() -> Self {
        AirActivity::Idle
    }
}
