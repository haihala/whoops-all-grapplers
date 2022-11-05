use bevy::prelude::*;

use bevy_inspector_egui::Inspectable;
use characters::{Action, Character, Inventory, Move, MoveHistory, Resources, Situation};
use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use wag_core::{MoveId, Player};

use crate::{damage::Combo, ui::Notifications};

mod helper_types;
use helper_types::{ActivationType, Cancellation, Link, MoveActivation};

const AUTOCORRECT: usize = (0.2 * wag_core::FPS) as usize;

#[derive(Debug, Default, Component, Inspectable)]
pub struct MoveBuffer {
    buffer: Vec<(usize, MoveId)>,
    activation: Option<MoveActivation>,
}
impl MoveBuffer {
    fn add_events(&mut self, events: Vec<MoveId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|(frame, _)| {
            if current_frame > *frame {
                // Default case, retain those who are fresh
                current_frame - frame < AUTOCORRECT
            } else {
                // Round has restarted, clear the buffer
                false
            }
        });
    }

    pub fn clear_all(&mut self) {
        *self = MoveBuffer::default();
    }

    fn get_situation_moves(
        &self,
        character: &Character,
        situation: Situation,
    ) -> Vec<(usize, MoveId, Move)> {
        self.buffer
            .iter()
            .filter_map(|(frame, id)| {
                if let Some(move_data) = character.get_move(*id) {
                    if (move_data.requirement)(situation.to_owned()) {
                        return Some((*frame, *id, move_data));
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
pub(super) fn move_continuation(mut query: Query<(&mut MoveBuffer, &mut PlayerState)>) {
    // Read from state, set activating move if an Action demands it
    for (mut buffer, mut state) in &mut query {
        let move_continuations = state.drain_matching_actions(|action| {
            if let Action::Move(move_id) = action {
                Some(*move_id)
            } else {
                None
            }
        });
        match move_continuations.len() {
            1 => {
                buffer.activation = Some(MoveActivation {
                    kind: ActivationType::Continuation,
                    id: move_continuations[0],
                })
            }
            0 => {
                // Nothing to do, so do nothing
            }
            _ => {
                todo!("Multiple moves to continue")
            }
        }
    }
}
pub(super) fn raw_or_link(
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &Character,
        &PlayerState,
        &Inventory,
        &Resources,
        &InputParser,
    )>,
) {
    // Set activating move if one in the buffer can start raw or be linked into
    for (mut buffer, character, state, inventory, resources, parser) in &mut query {
        if let Some(freedom_frame) = state.free_since {
            // Character has recently been freed

            if let Some((stored, id, _)) = buffer
                .get_situation_moves(
                    character,
                    Situation {
                        inventory,
                        history: state.get_move_history().map(|history| history.to_owned()),
                        grounded: state.is_grounded(),
                        resources,
                        parser,
                        current_frame: clock.frame,
                        conditions: state.get_conditions(),
                    },
                )
                .into_iter()
                .min_by(|(_, id1, _), (_, id2, _)| id1.cmp(id2))
            {
                let error = stored as i32 - freedom_frame as i32;
                let kind = if error.abs() < AUTOCORRECT as i32 {
                    ActivationType::Link(Link::new(stored, freedom_frame))
                } else {
                    ActivationType::Raw
                };

                buffer.activation = Some(MoveActivation { id, kind });
            }
        }
    }
}
pub(super) fn special_cancel(
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &Character,
        &PlayerState,
        &Inventory,
        &Resources,
        &InputParser,
    )>,
) {
    // Set activating move if one in the buffer can be cancelled into
    for (mut buffer, character, state, inventory, resources, parser) in &mut query {
        if state.free_since.is_none() {
            if let Some(history) = state.get_move_history() {
                // Not free because a move is happening
                // Is current move cancellable, if so, since when
                if let Some((stored, id, cancellable_since)) = buffer
                    .get_situation_moves(
                        character,
                        Situation {
                            inventory,
                            history: state.get_move_history().map(|history| history.to_owned()),
                            grounded: state.is_grounded(),
                            resources,
                            parser,
                            current_frame: clock.frame,
                            conditions: state.get_conditions(),
                        },
                    )
                    .into_iter()
                    .filter_map(|(frame, id, data)| {
                        history
                            .cancellable_into_since(&data)
                            .map(|freedom| (frame, id, freedom))
                    })
                    .min_by(|(_, id1, _), (_, id2, _)| id1.cmp(id2))
                {
                    buffer.activation = Some(MoveActivation {
                        id,
                        kind: ActivationType::Cancel(Cancellation::new(stored, cancellable_since)),
                    });
                }
            }
        }
    }
}

pub(super) fn move_activator(
    clock: Res<Clock>,
    combo: Option<Res<Combo>>,
    mut notifications: ResMut<Notifications>,
    mut query: Query<(
        &mut MoveBuffer,
        &mut PlayerState,
        &mut Resources,
        &Player,
        &Character,
    )>,
) {
    // Activate and clear activating move
    for (mut buffer, mut state, mut resources, player, character) in &mut query {
        if let Some(activation) = buffer.activation.take() {
            let started = match activation.kind {
                ActivationType::Link(link) => {
                    if combo.is_some() {
                        notifications.add(*player, link.message());

                        if let Some(meter_gain) = link.meter_gain() {
                            resources.meter.gain(meter_gain);
                        }
                    }

                    link.correction
                }
                ActivationType::Cancel(cancellation) => {
                    if combo.is_some() {
                        notifications.add(*player, cancellation.message);
                    }
                    clock.frame
                }
                _ => clock.frame,
            };

            state.start_move(MoveHistory {
                move_id: activation.id,
                move_data: character.get_move(activation.id).unwrap(),
                started,
                frame_skip: clock.frame - started,
                ..default()
            });
            buffer.buffer.clear();
        }
    }
}
