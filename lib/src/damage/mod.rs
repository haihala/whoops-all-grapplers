use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

mod boxes;
pub use boxes::*;

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    scalar: f32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            scalar: 1.0,
        }
    }
}
impl Health {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }
    pub fn reset(&mut self) {
        self.ratio = 1.0;
    }

    pub fn hurt(&mut self, amount: f32) {
        self.ratio -= self.scalar * amount
    }
}
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_hitboxes.system())
            .add_system(register_hits.system());
    }
}
