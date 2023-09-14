use bevy::prelude::*;
use characters::{Inventory, WAGResources};
use player_state::PlayerState;
use wag_core::Clock;

pub(super) fn move_advancement(
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &Inventory, &WAGResources)>,
) {
    for (mut state, inventory, resources) in &mut query {
        if state.action_in_progress() {
            state.proceed_move(inventory.clone(), resources.clone(), clock.frame);
        }
    }
}
