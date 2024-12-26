mod condition_management;
mod force_state;
mod move_activation;
mod move_advancement;
mod player_flash;
mod player_setup;
mod recovery;
mod side_switcher;
mod size_adjustment;

use foundation::{MatchState, RollbackSchedule, SystemStep};

use bevy::prelude::*;

pub use move_activation::MoveBuffer;
pub use player_setup::reset_combat;

pub struct PlayerStateManagementPlugin;

impl Plugin for PlayerStateManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                player_setup::setup_players
                    .run_if(in_state(MatchState::Loading))
                    .in_set(SystemStep::SpawnPlayers),
                side_switcher::sideswitcher.in_set(SystemStep::SideSwitch),
                (
                    condition_management::expire_conditions,
                    condition_management::update_combined_status_effect,
                )
                    .chain()
                    .in_set(SystemStep::Conditions),
                (
                    move_activation::manage_buffer,
                    move_activation::move_activator,
                    move_advancement::move_advancement,
                )
                    .chain()
                    .in_set(SystemStep::MovePipeline),
                (recovery::stun_recovery, recovery::ground_recovery)
                    .chain()
                    .in_set(SystemStep::Recovery),
                (
                    size_adjustment::update_box_sizes_from_state,
                    size_adjustment::remove_old_hurtbox_expansions,
                )
                    .chain()
                    .in_set(SystemStep::PlayerUpdates),
            ),
        );
    }
}
