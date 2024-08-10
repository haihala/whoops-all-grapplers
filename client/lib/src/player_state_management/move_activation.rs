use bevy::prelude::*;

use characters::{
    Action, ActionEvent, ActionRequirement, Character, Inventory, Situation, WAGResources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, Clock, Facing, Player, Stats};

use crate::{movement::PlayerVelocity, ui::Notifications};

#[derive(Debug, Default, Reflect)]
pub(super) struct MoveActivation {
    pub kind: ActivationType,
    pub id: ActionId,
}

#[derive(Debug, Default, Reflect)]
pub(super) enum ActivationType {
    Continuation,
    #[default]
    Plain,
    Cancel,
}

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
pub(super) fn automatic_activation(
    mut notifications: ResMut<Notifications>,
    mut query: Query<(
        &mut MoveBuffer,
        &mut PlayerState,
        &mut PlayerVelocity,
        &Player,
        &Facing,
    )>,
) {
    // Read from state, set activating move if an Action demands it
    for (mut buffer, mut state, mut velocity, player, facing) in &mut query {
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
                // This may happen if follow up and grab land on the same frame
                velocity.add_impulse(facing.mirror_vec2(Vec2::X * -10.0));
                notifications.add(*player, "Twin starters".to_owned());
            }
        }
    }
}
pub(super) fn plain_start(
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
        if state.free_since.is_none() {
            // Can't link if not free
            continue;
        };

        let Some((_, id, _)) = buffer
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
        else {
            continue;
        };

        buffer.activation = Some(MoveActivation {
            id,
            kind: ActivationType::Plain,
        });
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
        if state.free_since.is_some() {
            continue;
        }

        let Some(tracker) = state.get_action_tracker() else {
            continue;
        };

        // Not free because a move is happening
        // Is current move cancellable, if so, since when
        let Some((_, id, _)) = buffer
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
        else {
            continue;
        };

        buffer.activation = Some(MoveActivation {
            id,
            kind: ActivationType::Cancel,
        });
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn move_activator(
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &mut PlayerState,
        &mut WAGResources,
        &Character,
        &Stats,
        &Inventory,
        &InputParser,
    )>,
) {
    // Activate and clear activating move
    for (mut buffer, mut state, properties, character, stats, inventory, parser) in &mut query {
        let Some(activation) = buffer.activation.take() else {
            continue;
        };

        if state.active_cinematic().is_some() {
            continue;
        }

        state.start_move(
            activation.id,
            character.get_move(activation.id).unwrap(),
            clock.frame,
            inventory.to_owned(),
            properties.to_owned(),
            parser.to_owned(),
            stats.to_owned(),
        );

        buffer.clear_all()
    }
}
