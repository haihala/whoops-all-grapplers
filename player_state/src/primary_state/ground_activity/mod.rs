use bevy_inspector_egui::Inspectable;

use crate::{animation::Animation, FreedomLevel};

mod movement;
pub use movement::{DashState, Movement};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum GroundActivity {
    Stun(usize),
    Animation(Animation),
    Movement(Movement),
    Crouching,
    Standing,
}

impl GroundActivity {
    pub fn freedom_level(&self) -> FreedomLevel {
        match *self {
            GroundActivity::Animation(_) => FreedomLevel::Busy,
            GroundActivity::Stun(_) => FreedomLevel::Stunned,
            GroundActivity::Movement(movement) => {
                if movement.in_dash_startup() {
                    FreedomLevel::Busy
                } else if movement.in_dash_recovery() {
                    FreedomLevel::LightBusy
                } else {
                    // Walking
                    FreedomLevel::Free
                }
            }
            _ => FreedomLevel::Free,
        }
    }
}

impl Default for GroundActivity {
    fn default() -> Self {
        GroundActivity::Standing
    }
}
