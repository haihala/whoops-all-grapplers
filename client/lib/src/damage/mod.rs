use bevy::prelude::*;

mod combo;
mod defense;
mod dynamic_colliders;
mod hit_tracker;
mod hitboxes;
mod hitreg;
mod hitstop;

pub use hitboxes::{handle_despawn_flags, spawn_new_hitboxes, LifetimeFlags};
pub use hitreg::{blockstun_events, hitstun_events, launch_events, snap_and_switch};

pub use combo::Combo;
pub use defense::Defense;
pub use hit_tracker::HitTracker;
pub use hitboxes::HitboxSpawner;

use wag_core::{RollbackSchedule, WAGStage};
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                dynamic_colliders::create_colliders,
                dynamic_colliders::update_colliders,
                hitreg::clash_parry,
                hitreg::detect_hits.pipe(hitreg::apply_connections),
                hitboxes::handle_despawn_flags,
                defense::timeout_defense_streak,
            )
                .chain()
                .in_set(WAGStage::HitReg),
        )
        .add_systems(
            RollbackSchedule,
            hitstop::clear_hitstop.in_set(WAGStage::HitStop),
        )
        .observe(hitstop::start_hitstop);
    }
}
