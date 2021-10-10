use bevy_inspector_egui::Inspectable;

use types::Hit;

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    defense: f32,
    hits: Vec<Hit>,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            defense: 100.0,
            hits: Vec::new(),
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

    pub fn hit(&mut self, hit: Hit) {
        self.hits.push(hit);
    }
}
