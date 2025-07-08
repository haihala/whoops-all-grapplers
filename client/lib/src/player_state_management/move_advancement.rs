use bevy::prelude::*;
use characters::{Character, Gauges, Hurtboxes, Inventory};
use foundation::{CharacterClock, CharacterFacing, Clock, Combo, Stats, StatusFlag};
use input_parsing::InputParser;
use player_state::PlayerState;

use crate::event_spreading::{ColorShift, EndAction};

#[allow(clippy::type_complexity)]
pub(super) fn move_advancement(
    mut commands: Commands,
    abs_clock: Res<Clock>,
    mut query: Query<(
        &mut PlayerState,
        &CharacterClock,
        &Transform,
        &Inventory,
        &Character,
        &Gauges,
        &InputParser,
        &Stats,
        &CharacterFacing,
        Entity,
        &Combo,
    )>,
) {
    for (
        mut state,
        char_clock,
        tf,
        inventory,
        character,
        resources,
        parser,
        stats,
        facing,
        entity,
        combo,
    ) in &mut query
    {
        if char_clock.hitstop_frames > 0 {
            continue;
        }

        for event in state.proceed_move(
            inventory.to_owned(),
            character,
            resources.to_owned(),
            parser.to_owned(),
            stats.to_owned(),
            char_clock.frame,
            abs_clock.frame,
            tf.translation,
            *facing,
            combo.to_owned(),
        ) {
            commands.trigger_targets(event, entity)
        }
    }
}

pub fn end_moves(
    trigger: Trigger<EndAction>,
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &mut Hurtboxes)>,
) {
    let (mut state, mut hurtboxes) = query.get_mut(trigger.target()).unwrap();
    state.recover(clock.frame);
    hurtboxes.extra.clear();
    if state.has_flag(StatusFlag::Weaken) {
        state.clear_conditions(StatusFlag::Weaken);
        // This clears the current color
        commands.trigger_targets(ColorShift(Color::default(), 0), trigger.target());
    }
}
