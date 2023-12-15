use bevy::prelude::*;

use characters::{
    Action, ActionEvent, ActionRequirement, Character, Inventory, ResourceType, Situation,
    WAGResources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, Clock, Player, Stats};

use crate::{damage::Combo, ui::Notifications};

mod helper_types;
use helper_types::{ActivationType, Cancellation, Link, MoveActivation};

const AUTOCORRECT: usize = (0.1 * wag_core::FPS) as usize;

#[derive(Debug, Default, Component, Reflect)]
pub struct MoveBuffer {
    buffer: Vec<(usize, ActionId)>,
    activation: Option<MoveActivation>,
}
impl MoveBuffer {
    fn add_events(&mut self, events: Vec<ActionId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|(frame, _)| {
            if current_frame > *frame {
                // Default case, retain those who are fresh
                // TODO: autocorrect and buffer should be separate.
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
    ) -> Vec<(usize, ActionId, Action)> {
        self.buffer
            .iter()
            .filter_map(|(frame, id)| {
                if let Some(action) = character.get_move(*id) {
                    if ActionRequirement::check(&action.requirements, &situation) {
                        return Some((*frame, *id, action));
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
pub(super) fn automatic_activation(mut query: Query<(&mut MoveBuffer, &mut PlayerState)>) {
    // Read from state, set activating move if an Action demands it
    for (mut buffer, mut state) in &mut query {
        let move_continuations = state.drain_matching_actions(|action| {
            if let ActionEvent::StartAction(move_id) = action {
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
        &WAGResources,
        &Stats,
        &InputParser,
    )>,
) {
    // Set activating move if one in the buffer can start raw or be linked into
    for (mut buffer, character, state, inventory, resources, stats, parser) in &mut query {
        if let Some(freedom_frame) = state.free_since {
            // Character has recently been freed

            if let Some((stored, id, _)) = buffer
                .get_situation_moves(
                    character,
                    state.build_situation(
                        inventory.to_owned(),
                        resources.to_owned(),
                        parser.to_owned(),
                        stats.to_owned(),
                        clock.frame,
                    ),
                )
                .into_iter()
                .max_by_key(|(_, id, _)| (parser.get_complexity(*id), *id))
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
        &WAGResources,
        &Stats,
        &InputParser,
    )>,
) {
    // Set activating move if one in the buffer can be cancelled into
    for (mut buffer, character, state, inventory, resources, stats, parser) in &mut query {
        if state.free_since.is_none() {
            if let Some(tracker) = state.get_action_tracker() {
                // Not free because a move is happening
                // Is current move cancellable, if so, since when
                if let Some((stored, id, cancellable_since)) = buffer
                    .get_situation_moves(
                        character,
                        state.build_situation(
                            inventory.to_owned(),
                            resources.to_owned(),
                            parser.to_owned(),
                            stats.to_owned(),
                            clock.frame,
                        ),
                    )
                    .into_iter()
                    .filter_map(|(frame, id, action)| {
                        tracker
                            .cancellable_into_since(id, action.clone())
                            .map(|freedom| (frame, id, freedom))
                    })
                    .max_by_key(|(_, id, _)| (parser.get_complexity(*id), *id))
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
        &mut WAGResources,
        &Player,
        &Character,
        &Stats,
    )>,
) {
    // Activate and clear activating move
    for (mut buffer, mut state, mut properties, player, character, stats) in &mut query {
        if let Some(activation) = buffer.activation.take() {
            let start_frame = match activation.kind {
                ActivationType::Link(link) => {
                    if combo.is_some() {
                        notifications.add(*player, link.message());

                        if let Some(meter_gain) = link.meter_gain() {
                            properties
                                .get_mut(ResourceType::Meter)
                                .unwrap()
                                .gain((meter_gain as f32 * stats.link_bonus_multiplier) as i32);
                        }
                    }

                    // Autocorrect so that the move starts sooner.
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

            state.start_move(
                activation.id,
                character.get_move(activation.id).unwrap(),
                start_frame,
                clock.frame - start_frame,
            );
        }
    }
}
