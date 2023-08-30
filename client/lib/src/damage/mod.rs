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
        app.add_systems(
            Update,
            (
                hitboxes::spawn_new,
                hitboxes::despawn_expired,
                hitreg::clash_parry,
                hitreg::detect_hits.pipe(hitreg::apply_hits),
                hitreg::stun_actions,
                hitreg::snap_and_switch,
                defense::timeout_defense_streak,
                hitboxes::update_followers,
            )
                .in_set(WAGStage::HitReg),
        )
        .add_systems(OnExit(GameState::Combat), hitboxes::despawn_everything);
    }
}
