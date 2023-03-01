use bevy::prelude::*;
use characters::{Action, Character, Inventory};
use player_state::PlayerState;
use wag_core::{Clock, StatusCondition, StatusEffect};

pub fn manage_conditions(mut query: Query<&mut PlayerState>, clock: Res<Clock>) {
    // Could be split in two if need arises
    // Adding new conditions could even be completely within player state, but seeing as that's not guaranteed to last, put it outside
    for mut state in &mut query {
        state.expire_conditions(clock.frame);

        for new_condition in state.drain_matching_actions(|action| {
            if let Action::Condition(cond) = action {
                Some(*cond)
            } else {
                None
            }
        }) {
            state.add_condition(StatusCondition {
                expiration: new_condition.expiration.map(|delta| clock.frame + delta),
                ..new_condition
            });
        }
    }
}

pub fn update_combined_status_effect(
    mut query: Query<(&mut StatusEffect, &PlayerState, &Inventory, &Character)>,
) {
    for (mut effect, state, inventory, character) in &mut query {
        *effect = state
            .combined_status_effects()
            .combine(&inventory.get_effects(character))
            .combine(&character.base_stats);
    }
}
