use bevy::prelude::*;
use characters::{Inventory, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Clock, Stats};

pub(super) fn move_advancement(
    clock: Res<Clock>,
    mut query: Query<(
        &mut PlayerState,
        &Transform,
        &Inventory,
        &WAGResources,
        &InputParser,
        &Stats,
    )>,
) {
    for (mut state, tf, inventory, resources, parser, stats) in &mut query {
        if state.action_in_progress() {
            state.proceed_move(
                inventory.to_owned(),
                resources.to_owned(),
                parser.to_owned(),
                stats.to_owned(),
                clock.frame,
                tf.translation,
            );
        }
    }
}
