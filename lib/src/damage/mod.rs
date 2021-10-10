use bevy::prelude::*;

mod health;
pub use health::Health;

mod boxes;
pub use boxes::{HitboxManager, Hurtbox};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(boxes::spawn_hitboxes.system())
            .add_system(boxes::register_hits.system());
    }
}
