use bevy::prelude::*;

use characters::{Character, Hurtboxes, Inventory, Situation, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, AvailableCancels, Clock, Combo, Facing, OpenCancelWindow, Stats};

use crate::event_spreading::{AllowCancel, StartAction};

const AUTOCORRECT: usize = (0.1 * wag_core::FPS) as usize;

#[derive(Debug, Default, Component, Reflect, Clone)]
pub struct MoveBuffer {
    buffer: Vec<(usize, ActionId)>,
    activation: Option<ActionId>,
}
impl MoveBuffer {
    fn add_events(&mut self, events: Vec<ActionId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|(frame, _)| {
            if current_frame < *frame {
                // Default case, retain those who are fresh
                current_frame - frame < AUTOCORRECT
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
        windows: &Vec<OpenCancelWindow>,
        situation: Situation,
    ) -> Vec<(usize, ActionId)> {
        self.buffer
            .iter()
            .filter_map(|(frame, id)| {
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

pub fn manage_cancel_windows(
    trigger: Trigger<AllowCancel>,
    clock: Res<Clock>,
    mut query: Query<&mut AvailableCancels>,
) {
    let mut cancels = query.get_mut(trigger.entity()).unwrap();
    cancels.open(trigger.event().0.to_owned(), clock.frame);
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
    clock: Res<Clock>,
    mut query: Query<(
        &mut Hurtboxes,
        &mut MoveBuffer,
        &AvailableCancels,
        &Transform,
        &Character,
        &mut PlayerState,
        &Inventory,
        &WAGResources,
        &Stats,
        &InputParser,
        &Facing,
        Option<&Combo>,
    )>,
) {
    // Activate and clear activating move
    for (
        mut hurtboxes,
        mut buffer,
        available_cancels,
        tf,
        character,
        mut state,
        inventory,
        resources,
        stats,
        parser,
        facing,
        combo,
    ) in &mut query
    {
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
                combo.copied(),
            );

            let situation_moves =
                buffer.get_situation_moves(character, &available_cancels.0, situation);

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

        buffer.buffer.retain(|(_, id)| *id != to_activate);
        state.start_move(to_activate, clock.frame);
    }
}
