use bevy::prelude::*;

#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq, Eq)]
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
            full_progress: (0.75 * wag_core::FPS) as usize,
        }
    }
}
impl Charge {
    pub fn is_charged(&self) -> bool {
        self.progress >= self.full_progress
    }

    pub fn consume_charge(&mut self) {
        assert!(self.is_charged());
        self.reset();
    }

    pub fn reset(&mut self) {
        self.progress = 0;
    }
    pub fn get_percentage(&self) -> f32 {
        (self.progress as f32 / self.full_progress as f32).min(1.0) * 100.0
    }
}
