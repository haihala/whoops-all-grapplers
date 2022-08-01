use bevy::prelude::*;

mod hitreg;

mod grabbing;
mod health;
pub use health::Health;

mod hitboxes;
pub use hitboxes::HitboxSpawner;

use time::{GameState, WAGStage};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            WAGStage::HitReg,
            SystemSet::new()
                .with_system(hitboxes::spawn_new)
                .with_system(hitboxes::despawn_expired.after(hitboxes::spawn_new))
                .with_system(hitreg::clash_parry.after(hitboxes::despawn_expired))
                .with_system(hitreg::register_hits.after(hitreg::clash_parry))
                .with_system(grabbing::spawn_grabs.after(hitreg::register_hits))
                .with_system(grabbing::register_grabs.after(grabbing::spawn_grabs))
                .with_system(
                    health::check_dead
                        .after(grabbing::register_grabs)
                        .with_run_criteria(State::on_update(GameState::Combat)),
                )
                .with_system(
                    hitboxes::despawn_everything
                        .with_run_criteria(State::on_exit(GameState::Combat)),
                ),
        );
    }
}
