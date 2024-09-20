use bevy::prelude::*;
use characters::{ActionEvent, ActionEvents, Character, Inventory, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{AvailableCancels, Clock, Facing, Stats};

#[allow(clippy::type_complexity)]
pub(super) fn move_advancement(
    clock: Res<Clock>,
    mut query: Query<(
        &mut PlayerState,
        &mut ActionEvents,
        &Transform,
        &Inventory,
        &Character,
        &WAGResources,
        &InputParser,
        &Stats,
        &Facing,
    )>,
) {
    for (mut state, mut events, tf, inventory, character, resources, parser, stats, facing) in
        &mut query
    {
        if state.action_in_progress() {
            events.add_events(state.proceed_move(
                inventory.to_owned(),
                character.to_owned(),
                resources.to_owned(),
                parser.to_owned(),
                stats.to_owned(),
                clock.frame,
                tf.translation,
                *facing,
            ));
        }
    }
}

pub fn end_moves(
    clock: Res<Clock>,
    mut query: Query<(&ActionEvents, &mut PlayerState, &mut AvailableCancels)>,
) {
    for (events, mut state, mut windows) in &mut query {
        let end_event_present = !events
            .get_matching_events(|action| {
                if *action == ActionEvent::End {
                    Some(0)
                } else {
                    None
                }
            })
            .is_empty();

        if end_event_present {
            state.recover(clock.frame);
            windows.clear();
        }
    }
}
