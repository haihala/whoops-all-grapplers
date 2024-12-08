use bevy::prelude::*;
use characters::{Character, Hurtboxes, Inventory, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Clock, Combo, Facing, Stats, StatusFlag};

use crate::event_spreading::{ColorShift, EndAction};

#[allow(clippy::type_complexity)]
pub(super) fn move_advancement(
    mut commands: Commands,
    clock: Res<Clock>,
    mut last_processed_frame: Local<usize>,
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
        Option<&Combo>,
    )>,
) {
    if *last_processed_frame == clock.frame {
        return;
    }

    for (mut state, tf, inventory, character, resources, parser, stats, facing, entity, combo) in
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
                combo.copied(),
            ) {
                commands.trigger_targets(event, entity)
            }
        }
    }

    *last_processed_frame = clock.frame;
}

pub fn end_moves(
    trigger: Trigger<EndAction>,
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &mut Hurtboxes)>,
) {
    let (mut state, mut hurtboxes) = query.get_mut(trigger.entity()).unwrap();
    state.recover(clock.frame);
    hurtboxes.extra.clear();
    if state.has_flag(StatusFlag::Weaken) {
        state.clear_conditions(StatusFlag::Weaken);
        // This clears the current color
        commands.trigger_targets(ColorShift(Color::default(), 0), trigger.entity());
    }
}
