use bevy::prelude::*;

mod combo;
mod defense;
mod hitboxes;
mod hitreg;

pub use combo::Combo;
pub use defense::Defense;
pub use hitboxes::HitboxSpawner;

use wag_core::{GameState, WAGStage};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            WAGStage::HitReg,
            SystemSet::new()
                .with_system(hitboxes::spawn_new)
                .with_system(hitboxes::despawn_expired.after(hitboxes::spawn_new))
                .with_system(hitreg::clash_parry.after(hitboxes::despawn_expired))
                .with_system(
                    hitreg::detect_hits
                        .pipe(hitreg::apply_hits)
                        .after(hitreg::clash_parry)
                        .label("autolabels don't work for this"),
                )
                .with_system(hitreg::stun_actions.after("autolabels don't work for this"))
                .with_system(hitreg::snap_and_switch.after(hitreg::stun_actions))
                .with_system(defense::timeout_defense_streak.after(hitreg::snap_and_switch))
                .with_system(hitboxes::update_followers.after(defense::timeout_defense_streak))
                .with_system(
                    hitboxes::despawn_everything
                        .with_run_criteria(State::on_exit(GameState::Combat)),
                ),
        );
    }
}
