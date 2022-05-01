use bevy_inspector_egui::Inspectable;

use kits::MoveSituation;

#[derive(Inspectable, PartialEq, Clone, Debug)]
pub enum AirActivity {
    Freefall,
    Move(MoveSituation),
    Idle,
}

impl Default for AirActivity {
    fn default() -> Self {
        AirActivity::Idle
    }
}
