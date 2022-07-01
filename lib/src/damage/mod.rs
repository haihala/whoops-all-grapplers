mod hitreg;
use bevy::prelude::*;

mod health;
pub use health::Health;

use time::{GameState, WAGStage};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            WAGStage::HitReg,
            SystemSet::new()
                .with_system(hitreg::register_hits)
                .with_system(hitreg::handle_grabs.after(hitreg::register_hits))
                .with_system(
                    health::check_dead
                        .after(hitreg::handle_grabs)
                        .with_run_criteria(State::on_update(GameState::Combat)),
                ),
        );
    }
}
