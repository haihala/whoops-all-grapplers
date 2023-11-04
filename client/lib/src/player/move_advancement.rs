use bevy::prelude::*;
use characters::{Inventory, WAGResources};
use player_state::PlayerState;
use wag_core::{Clock, Stats};

pub(super) fn move_advancement(
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &Inventory, &WAGResources, &Stats)>,
) {
    for (mut state, inventory, resources, stats) in &mut query {
        if state.action_in_progress() {
            state.proceed_move(
                inventory.to_owned(),
                resources.to_owned(),
                stats.to_owned(),
                clock.frame,
            );
        }
    }
}
