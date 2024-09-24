use bevy::prelude::*;
use characters::{Character, Inventory, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{AvailableCancels, Clock, Facing, Stats};

use crate::event_spreading::EndAction;

#[allow(clippy::type_complexity)]
pub(super) fn move_advancement(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        &mut PlayerState,
        &Transform,
        &Inventory,
        &Character,
        &WAGResources,
        &InputParser,
        &Stats,
        &Facing,
        Entity,
    )>,
) {
    for (mut state, tf, inventory, character, resources, parser, stats, facing, entity) in
        &mut query
    {
        if state.action_in_progress() {
            for event in state.proceed_move(
                inventory.to_owned(),
                character,
                resources.to_owned(),
                parser.to_owned(),
                stats.to_owned(),
                clock.frame,
                tf.translation,
                *facing,
            ) {
                commands.trigger_targets(event, entity)
            }
        }
    }
}

pub fn end_moves(
    trigger: Trigger<EndAction>,
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &mut AvailableCancels)>,
) {
    let (mut state, mut windows) = query.get_mut(trigger.entity()).unwrap();
    state.recover(clock.frame);
    windows.clear();
}
