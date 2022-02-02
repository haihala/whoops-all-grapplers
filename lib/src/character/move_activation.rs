use bevy::prelude::*;

use input_parsing::InputParser;
use moves::{CancelLevel, Move, MoveBank};
use player_state::{MoveState, PlayerState};
use time::Clock;
use types::MoveId;

use crate::meter::Meter;

pub fn move_activator(
    clock: Res<Clock>,
    mut query: Query<(&mut InputParser, &mut PlayerState, &MoveBank, &mut Meter)>,
) {
    for (mut reader, mut state, bank, mut meter) in query.iter_mut() {
        if state.stunned() {
            continue;
        }

        let events = reader.get_events();
        if events.is_empty() {
            continue;
        }

        if let Some((move_id, move_data)) = move_to_activate(
            events,
            bank,
            state.get_move_state(),
            state.is_grounded(),
            &meter,
        ) {
            let move_state = MoveState {
                start_frame: clock.frame,
                phase_index: 0,
                move_id,
            };
            state.set_move_state(move_state);
            meter.pay(move_data.meter_cost);
            reader.consume_event(move_id);
        }
    }
}

fn move_to_activate(
    options: Vec<MoveId>,
    bank: &MoveBank,
    active_move: Option<MoveState>,
    grounded: bool,
    meter: &Meter,
) -> Option<(MoveId, Move)> {
    let cancel_requirement = if let Some(move_state) = active_move {
        let move_data = bank.get(move_state.move_id);
        if move_data.get_phase(move_state.phase_index).cancellable {
            move_data.cancel_level
        } else {
            CancelLevel::Uncancellable
        }
    } else {
        CancelLevel::Anything
    };

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
