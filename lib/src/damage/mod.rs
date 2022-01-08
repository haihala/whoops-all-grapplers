use bevy::prelude::*;

mod health;
pub use health::Health;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(health::apply_hits.system());
    }
}
