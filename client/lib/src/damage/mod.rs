use bevy::prelude::*;

mod combo;
mod defense;
mod dynamic_colliders;
mod hit_tracker;
mod hitboxes;
mod hitreg;
mod hitstop;

pub use combo::Combo;
pub use defense::Defense;
pub use hit_tracker::HitTracker;
pub use hitboxes::HitboxSpawner;

use wag_core::{GameState, WAGStage};

use crate::entity_management::despawn_marked;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                dynamic_colliders::create_colliders,
                dynamic_colliders::update_colliders,
                hitboxes::spawn_new_hitboxes,
                hitreg::clash_parry,
                hitreg::detect_hits.pipe(hitreg::apply_connections),
                hitboxes::handle_despawn_flags,
                despawn_marked,
                hitreg::stun_actions,
                hitreg::snap_and_switch,
                defense::timeout_defense_streak,
            )
                .chain()
                .in_set(WAGStage::HitReg),
        )
        .add_systems(
            FixedUpdate,
            (hitstop::clear_hitstop, hitstop::handle_hitstop_events)
                .chain()
                .after(WAGStage::PlayerUpdates),
        )
        .add_systems(OnExit(GameState::Combat), hitboxes::despawn_everything);
    }
}
