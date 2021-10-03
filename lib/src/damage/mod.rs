use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

mod boxes;
pub use boxes::*;

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    pub ratio: f32,
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
    pub fn hurt(&mut self, amount: f32) {
        self.ratio -= self.scalar * amount
    }
}
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(hurtbox_manager.system())
            .add_system(handle_hits.system());
    }
}
