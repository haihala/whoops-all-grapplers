use bevy::{platform::collections::HashMap, prelude::*};

use characters::{Character, Gauges, Hurtboxes, Inventory, Situation};
use foundation::{ActionId, CharacterClock, CharacterFacing, Clock, Combo, Stats};
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
    pub fn add_events(&mut self, events: impl Iterator<Item = ActionId>, frame: usize) {
        for event in events {
            self.buffer.insert(event, frame);
        }
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|_, buffer_entry_frame| {
            if *buffer_entry_frame > current_frame {
                // Last frame input, but clock has reset
                false
            } else {
                // Default case
                current_frame - *buffer_entry_frame < INPUT_BUFFER
            }
        });
    }

    pub fn reset(&mut self) {
        *self = MoveBuffer::default();
    }

    fn get_situation_moves(
        &self,
        character: &Character,
        situation: Situation,
    ) -> Vec<(usize, ActionId)> {
        self.buffer
            .iter()
            .filter_map(|(id, frame)| {
                if let Some(action) = character.get_move(*id) {
                    if action.requirement.check(*id, &situation, false) {
                        return Some((*frame, *id));
                    }
                }
                None
            })
            .collect()
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn manage_buffer(
    abs_clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &mut InputParser,
        &Character,
        &Inventory,
        &Gauges,
        &Stats,
        &CharacterFacing,
        &CharacterClock,
        &Combo,
        &Transform,
        &PlayerState,
    )>,
) {
    // Read from the input parser and fill the buffer
    for (
        mut buffer,
        mut parser,
        character,
        inventory,
        resources,
        stats,
        facing,
        char_clock,
        combo,
        tf,
        state,
    ) in &mut query
    {
        let situation = state.build_situation(
            inventory.to_owned(),
            resources.to_owned(),
            parser.to_owned(),
            stats.to_owned(),
            char_clock.frame,
            abs_clock.frame,
            tf.translation,
            *facing,
            combo.to_owned(),
        );

        buffer.clear_old(char_clock.frame);
        buffer.add_events(
            parser.get_events().into_iter().filter(|id| {
                // Some requirements are checked at input reg time (charge)

                let Some(action) = character.get_move(*id) else {
                    panic!("Move is not in a char's kit");
                };

                action.requirement.check(*id, &situation, true)
            }),
            char_clock.frame,
        );
        parser.clear();
    }
}

pub(super) fn automatic_activation(
    trigger: Trigger<StartAction>,
    mut query: Query<&mut MoveBuffer>,
) {
    let mut buffer = query.get_mut(trigger.target()).unwrap();

    buffer.activation = Some(trigger.event().0)
}

#[allow(clippy::type_complexity)]
pub(super) fn move_activator(
    abs_clock: Res<Clock>,
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
        &CharacterClock,
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
        char_clock,
        combo,
    ) in &mut query
    {
        if char_clock.hitstop_frames > 0 {
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
                char_clock.frame,
                abs_clock.frame,
                tf.translation,
                *facing,
                combo.to_owned(),
            );

            let situation_moves = buffer.get_situation_moves(character, situation);

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

        let action = character.get_move(to_activate).unwrap();

        buffer.buffer.retain(|id, _| *id != to_activate);
        if action.transient {
            state.set_transient(to_activate);
        } else {
            // Remove old extra expanded hurtboxes (if a move is canceled)
            hurtboxes.extra.clear();

            state.start_move(to_activate, char_clock.frame);
        }
    }
}
