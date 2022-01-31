use bevy::prelude::*;

use input_parsing::InputParser;
use moves::{CancelLevel, Move, MoveBank};
use player_state::PlayerState;
use types::MoveId;

use crate::meter::Meter;

pub fn move_activator(
    mut query: Query<(&mut InputParser, &mut PlayerState, &MoveBank, &mut Meter)>,
) {
    for (mut reader, mut state, bank, mut meter) in query.iter_mut() {
        let events = reader.get_events();
        if events.is_empty() {
            continue;
        }

        if let Some((starting_move, move_data)) = move_to_activate(
            events,
            bank,
            state.cancel_requirement(),
            state.is_grounded(),
            &meter,
        ) {
            meter.pay(move_data.meter_cost);
            state.start_move(starting_move, move_data);
            reader.consume_event(starting_move);
        }
    }
}

fn move_to_activate(
    options: Vec<MoveId>,
    bank: &MoveBank,
    cancel_requirement: CancelLevel,
    grounded: bool,
    meter: &Meter,
) -> Option<(MoveId, Move)> {
    options
        .into_iter()
        .map(|id| (id, bank.get(id).to_owned()))
        .filter(|(_, action)| {
            if grounded {
                action.ground_ok
            } else {
                action.air_ok
            }
        })
        .filter(|(_, action)| action.cancel_level > cancel_requirement)
        .filter(|(_, action)| meter.can_afford(action.meter_cost))
        .min_by(|(id1, _), (id2, _)| id1.cmp(id2))
}
