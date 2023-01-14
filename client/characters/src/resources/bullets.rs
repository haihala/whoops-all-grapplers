use bevy::prelude::*;

#[derive(Reflect, Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bullets {
    available: i32,
}
impl Default for Bullets {
    fn default() -> Self {
        Self { available: 6 }
    }
}
impl Bullets {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn has_one(&self) -> bool {
        self.available > 0
    }
    pub fn use_one(&mut self) {
        assert!(self.available >= 1, "Using bullets you don't have");
        self.available -= 1;
    }
}
