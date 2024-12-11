use bevy::prelude::*;

mod hit_tracker;
mod hitboxes;
mod hitreg;
mod hitstop;

pub use hitboxes::{handle_despawn_flags, spawn_hitbox, LifetimeFlags};
pub use hitreg::{blockstun_events, hitstun_events, launch_events, snap_and_switch};

pub use hit_tracker::HitTracker;
pub use hitboxes::HitboxSpawner;

use wag_core::{RollbackSchedule, SystemStep};
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                hitreg::clash_parry,
                hitreg::detect_hits.pipe(hitreg::apply_connections),
                hitboxes::handle_despawn_flags,
            )
                .chain()
                .in_set(SystemStep::HitReg),
        )
        .add_systems(
            RollbackSchedule,
            hitstop::clear_hitstop.in_set(SystemStep::HitStop),
        )
        .observe(hitstop::start_hitstop);
    }
}
