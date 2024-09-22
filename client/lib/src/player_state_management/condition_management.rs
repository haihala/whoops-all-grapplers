use bevy::prelude::*;
use characters::{Character, Inventory};
use player_state::PlayerState;
use wag_core::{Clock, Stats, StatusCondition};

use crate::event_spreading::AddCondition;

pub fn manage_conditions(
    trigger: Trigger<AddCondition>,
    mut query: Query<&mut PlayerState>,
    clock: Res<Clock>,
) {
    let mut state = query.get_mut(trigger.entity()).unwrap();
    state.expire_conditions(clock.frame);

    let new_condition = trigger.event().0;
    state.add_condition(StatusCondition {
        expiration: new_condition
            .expiration
            .map(|duration| clock.frame + duration),
        ..new_condition
    });
}

pub fn update_combined_status_effect(
    mut query: Query<(&mut Stats, &PlayerState, &Inventory, &Character)>,
) {
    for (mut effect, state, inventory, character) in &mut query {
        *effect = state
            .combined_status_effects()
            .combine(&inventory.get_effects(character))
            .combine(&character.base_stats);
    }
}
