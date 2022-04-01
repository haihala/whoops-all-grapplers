use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::GameResource;

#[derive(Inspectable, Component, Clone, Copy)]
pub struct Charge {
    /// Last tick charge was updated (keep track of when to decay)
    pub last_update: usize,
    /// Ticks charged
    pub progress: usize,
    /// Ticks required for full charge
    full_progress: usize,
}

impl Default for Charge {
    fn default() -> Self {
        Self {
            last_update: 0,
            progress: 0,
            full_progress: (1.0 * constants::FPS) as usize,
        }
    }
}
impl GameResource<bool> for Charge {
    fn can_afford(&self, amount: bool) -> bool {
        self.is_charged() && amount
    }

    fn pay(&mut self, amount: bool) {
        if amount {
            assert!(self.is_charged());
            self.reset();
        }
    }

    fn get_ratio(&self) -> f32 {
        if self.is_charged() {
            1.0
        } else {
            self.progress as f32 / self.full_progress as f32
        }
    }

    fn reset(&mut self) {
        self.progress = 0;
    }
}
impl Charge {
    pub fn new(full_charge_seconds: f32) -> Self {
        Charge {
            full_progress: (full_charge_seconds * constants::FPS) as usize,
            ..Default::default()
        }
    }

    pub fn is_charged(&self) -> bool {
        self.progress >= self.full_progress
    }
}
