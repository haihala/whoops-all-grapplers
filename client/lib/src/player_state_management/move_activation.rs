use bevy::prelude::*;

use characters::{Action, ActionRequirement, Character, Inventory, Situation, WAGResources};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, AvailableCancels, Clock, Facing, Stats};

use crate::event_spreading::{AllowCancel, StartAction};

#[derive(Debug, Default, Reflect, Clone, Copy)]
pub(super) struct MoveActivation {
    pub kind: ActivationType,
    pub id: ActionId,
}

#[derive(Debug, Default, Reflect, Clone, Copy)]
pub(super) enum ActivationType {
    Continuation,
    #[default]
    NewMove,
    Cancel,
}

const AUTOCORRECT: usize = (0.1 * wag_core::FPS) as usize;

#[derive(Debug, Default, Component, Reflect, Clone)]
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
            if current_frame < *frame {
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

    buffer.activation = Some(MoveActivation {
        kind: ActivationType::Continuation,
        id: trigger.event().0,
    })
}

#[allow(clippy::type_complexity)]
pub(super) fn plain_start(
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &Transform,
        &Character,
        &PlayerState,
        &Inventory,
        &WAGResources,
        &Stats,
        &InputParser,
        &Facing,
    )>,
) {
    // Set activating move if one in the buffer can start raw or be linked into
    for (mut buffer, tf, character, state, inventory, resources, stats, parser, facing) in
        &mut query
    {
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
                    tf.translation,
                    *facing,
                ),
            )
            .into_iter()
            .max_by_key(|(_, id, _)| (parser.get_complexity(*id), *id))
        else {
            continue;
        };

        buffer.activation = Some(MoveActivation {
            id,
            kind: ActivationType::NewMove,
        });
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn cancel_start(
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &AvailableCancels,
        &Transform,
        &Character,
        &PlayerState,
        &Inventory,
        &WAGResources,
        &Stats,
        &InputParser,
        &Facing,
    )>,
) {
    // Set activating move if one in the buffer can be cancelled into
    for (
        mut buffer,
        available_cancels,
        tf,
        character,
        state,
        inventory,
        resources,
        stats,
        parser,
        facing,
    ) in &mut query
    {
        if state.free_since.is_some() {
            continue;
        }

        let Some(tracker) = state.get_action_tracker() else {
            continue;
        };

        // Not free because a move is happening
        // Is current move cancellable, if so, since when
        let Some(id) = buffer
            .get_situation_moves(
                character,
                state.build_situation(
                    inventory.to_owned(),
                    resources.to_owned(),
                    parser.to_owned(),
                    stats.to_owned(),
                    clock.frame,
                    tf.translation,
                    *facing,
                ),
            )
            .into_iter()
            .filter_map(|(_, id, action)| {
                if available_cancels.can_cancel_to(action.category, id, tracker.has_hit) {
                    Some(id)
                } else {
                    None
                }
            })
            .max_by_key(|id| (parser.get_complexity(*id), *id))
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
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        &mut MoveBuffer,
        &mut PlayerState,
        &Transform,
        &WAGResources,
        &Character,
        &Stats,
        &Inventory,
        &InputParser,
        &Facing,
        Entity,
    )>,
) {
    // Activate and clear activating move
    for (
        mut buffer,
        mut state,
        tf,
        properties,
        character,
        stats,
        inventory,
        parser,
        facing,
        entity,
    ) in &mut query
    {
        let Some(activation) = buffer.activation.take() else {
            continue;
        };

        if state.active_cinematic().is_some() {
            continue;
        }

        for event in state.start_move(
            activation.id,
            character.get_move(activation.id).unwrap(),
            clock.frame,
            inventory.to_owned(),
            properties.to_owned(),
            parser.to_owned(),
            stats.to_owned(),
            tf.translation,
            *facing,
        ) {
            commands.trigger_targets(event, entity);
        }

        buffer.clear_all()
    }
}
