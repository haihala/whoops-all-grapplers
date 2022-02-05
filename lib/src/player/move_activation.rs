use bevy::prelude::*;

use input_parsing::InputParser;
use moves::{CancelLevel, Move, MoveBank};
use player_state::{MoveState, PlayerState};
use time::Clock;
use types::MoveId;

use crate::meter::Meter;
const EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
const FRAMES_TO_LIVE_IN_BUFFER: usize = (EVENT_REPEAT_PERIOD * constants::FPS) as usize;

#[derive(Debug, Default, Component)]
pub struct MoveBuffer {
    queue: Vec<(usize, Vec<MoveId>)>,
}
impl MoveBuffer {
    fn add_events(&mut self, events: Vec<MoveId>, frame: usize) {
        self.queue.push((frame, events));
    }

    fn use_move(
        &mut self,
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

        self.flatten()
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

    fn flatten(&self) -> Vec<MoveId> {
        self.queue
            .clone()
            .into_iter()
            .flat_map(|(_, moves)| moves)
            .collect()
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.queue
            .retain(|(frame, _)| current_frame - frame < FRAMES_TO_LIVE_IN_BUFFER);
    }
}

pub fn move_activator(
    clock: Res<Clock>,
    mut query: Query<(
        &mut InputParser,
        &mut PlayerState,
        &mut MoveBuffer,
        &MoveBank,
        &mut Meter,
    )>,
) {
    for (mut reader, mut state, mut buffer, bank, mut meter) in query.iter_mut() {
        buffer.clear_old(clock.frame);
        buffer.add_events(reader.drain_events(), clock.frame);

        if state.stunned() {
            continue;
        }

        if let Some((move_id, move_data)) =
            buffer.use_move(bank, state.get_move_state(), state.is_grounded(), &meter)
        {
            state.start_move(move_id, clock.frame);
            meter.pay(move_data.meter_cost);
        }
    }
}
