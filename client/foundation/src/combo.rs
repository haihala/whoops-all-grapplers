use bevy::prelude::Component;

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Combo {
    pub hits: usize,
    pub old_health: i32,
}

impl Combo {
    pub fn ongoing(&self) -> bool {
        self.hits != 0
    }

    pub fn start_at(&mut self, defender_health: i32) {
        self.hits = 1;
        self.old_health = defender_health;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
