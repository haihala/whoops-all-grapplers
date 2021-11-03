use bevy_inspector_egui::Inspectable;

use crate::animation::Animation;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum AirActivity {
    Freefall,
    Animation(Animation),
    Idle,
}

impl Default for AirActivity {
    fn default() -> Self {
        AirActivity::Idle
    }
}
