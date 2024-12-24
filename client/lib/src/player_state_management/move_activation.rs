use bevy::{prelude::*, utils::HashMap};

use characters::{Character, Gauges, Hurtboxes, Inventory, Situation};
use foundation::{ActionId, CancelType, CharacterClock, CharacterFacing, Clock, Combo, Stats};
use input_parsing::InputParser;
use player_state::PlayerState;

use crate::event_spreading::StartAction;

// In frames
const INPUT_BUFFER: usize = 6;

#[derive(Debug, Default, Component, Reflect, Clone)]
pub struct MoveBuffer {
    buffer: HashMap<ActionId, usize>,
    activation: Option<ActionId>,
}
impl MoveBuffer {
    pub fn add_events(&mut self, events: Vec<ActionId>, frame: usize) {
        for event in events {
            self.buffer.insert(event, frame);
        }
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|_, frame| {
            if current_frame <= *frame {
                // Default case, retain those who are fresh
                current_frame - *frame < INPUT_BUFFER
            } else {
                // Round has restarted, clear the buffer
                false
            }
        });
    }

    pub fn reset(&mut self) {
        *self = MoveBuffer::default();
    }

    fn get_situation_moves(
        &self,
        character: &Character,
        windows: &Vec<CancelType>,
        situation: Situation,
    ) -> Vec<(usize, ActionId)> {
        self.buffer
            .iter()
            .filter_map(|(id, frame)| {
                if let Some(action) = character.get_move(*id) {
                    if action.requirement.check(*id, windows, &situation) {
                        return Some((*frame, *id));
                    }
                }
                None
            })
            .collect()
    }
}

pub(super) fn manage_buffer(
    clock: Res<Clock>,
    mut query: Query<(&mut MoveBuffer, &mut InputParser)>,
) {
    // Read from the input parser and fill the buffer
    for (mut buffer, mut parser) in &mut query {
        buffer.clear_old(clock.frame);
        buffer.add_events(parser.get_events(), clock.frame);
        parser.clear();
    }
}

pub(super) fn automatic_activation(
    trigger: Trigger<StartAction>,
    mut query: Query<&mut MoveBuffer>,
) {
    let mut buffer = query.get_mut(trigger.entity()).unwrap();

    buffer.activation = Some(trigger.event().0)
}

#[allow(clippy::type_complexity)]
pub(super) fn move_activator(
    mut query: Query<(
        &mut Hurtboxes,
        &mut MoveBuffer,
        &Transform,
        &Character,
        &mut PlayerState,
        &Inventory,
        &Gauges,
        &Stats,
        &InputParser,
        &CharacterFacing,
        &mut CharacterClock,
        &Combo,
    )>,
) {
    // Activate and clear activating move
    for (
        mut hurtboxes,
        mut buffer,
        tf,
        character,
        mut state,
        inventory,
        resources,
        stats,
        parser,
        facing,
        mut clock,
        combo,
    ) in &mut query
    {
        if clock.move_activation_processed {
            continue;
        }

        let primary = buffer.activation.take();
        let to_activate = if let Some(id) = primary {
            id
        } else {
            let situation = state.build_situation(
                inventory.to_owned(),
                resources.to_owned(),
                parser.to_owned(),
                stats.to_owned(),
                clock.frame,
                tf.translation,
                *facing,
                combo.to_owned(),
            );

            let situation_moves =
                buffer.get_situation_moves(character, &state.cancels(), situation);

            if situation_moves.is_empty() {
                continue;
            }

            situation_moves
                .into_iter()
                .max_by_key(|(buffer_entry_frame, id)| {
                    // Sort by most complex input
                    (parser.get_complexity(*id), *buffer_entry_frame, *id)
                })
                .unwrap()
                .1
        };

        // Remove old extra expanded hurtboxes (if a move is cancelled)
        hurtboxes.extra.clear();

        buffer.buffer.retain(|id, _| *id != to_activate);
        state.start_move(to_activate, clock.frame);
        clock.move_activation_processed = true;
    }
}
