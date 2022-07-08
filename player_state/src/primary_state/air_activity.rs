use bevy_inspector_egui::Inspectable;

use characters::MoveSituation;

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
