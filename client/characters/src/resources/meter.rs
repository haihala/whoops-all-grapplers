use bevy::prelude::*;

#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Meter {
    value: i32,
    max: i32,
}
impl Default for Meter {
    fn default() -> Self {
        Self { value: 0, max: 100 }
    }
}
impl Meter {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn get_percentage(&self) -> f32 {
        (self.value as f32 / self.max as f32) * 100.0
    }
    pub fn can_afford(&self, amount: i32) -> bool {
        self.value >= amount
    }
    pub fn pay(&mut self, amount: i32) {
        assert!(self.value >= amount, "Meter overdraft");
        self.gain(-amount);
    }
    pub fn gain(&mut self, amount: i32) {
        self.value = (self.value + amount).min(self.max);
    }
}
