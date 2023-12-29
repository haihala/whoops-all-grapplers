use bevy::prelude::*;
use std::cmp::Ordering;

use characters::{
    Action, ActionEvent, ActionRequirement, Character, Inventory, ResourceType, Situation,
    WAGResources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, Clock, Facing, Player, Stats};

use crate::{damage::Combo, physics::PlayerVelocity, ui::Notifications};

#[derive(Debug, Default, Reflect)]
pub(super) struct MoveActivation {
    pub kind: ActivationType,
    pub id: ActionId,
}

#[derive(Debug, Default, Reflect)]
pub(super) enum ActivationType {
    Continuation,
    #[default]
    Raw,
    Link(Link),
    Cancel(Cancellation),
}

// +- frames. 0 is frame perfect, 1 means +-1 aka 3 frame window
const PERFECT_TIMING_DELTA: i32 = 0;
const GOOD_TIMING_DELTA: i32 = 3;

#[derive(Debug, Default, Reflect)]
enum ErrorDirection {
    Late,
    #[default]
    Early,
}
impl From<i32> for ErrorDirection {
    fn from(error: i32) -> Self {
        if error.signum() == 1 {
            Self::Late
        } else {
            Self::Early
        }
    }
}

#[derive(Debug, Default, Reflect)]
enum LinkPrecision {
    #[default]
    Perfect,
    Good(ErrorDirection),
    Fine(ErrorDirection),
}
impl LinkPrecision {
    fn from_frame_diff(frame_diff: i32) -> Self {
        if frame_diff.abs() <= PERFECT_TIMING_DELTA {
            Self::Perfect
        } else if frame_diff.abs() <= GOOD_TIMING_DELTA {
            Self::Good(frame_diff.into())
        } else {
            Self::Fine(frame_diff.into())
        }
    }

    fn meter_gain(&self) -> Option<i32> {
        match self {
            LinkPrecision::Perfect => Some(30),
            LinkPrecision::Good(_) => Some(10),
            LinkPrecision::Fine(_) => None,
        }
    }

    fn message(&self) -> String {
        match self {
            LinkPrecision::Perfect => "Perfect link!".to_owned(),
            LinkPrecision::Good(error) => format!("Good link! ({:?})", error),
            LinkPrecision::Fine(error) => format!("Linked ({:?})", error),
        }
    }
}

#[derive(Debug, Default, Reflect)]
pub(super) struct Link {
    pub correction: usize,
    precision: LinkPrecision,
}
impl Link {
    pub(super) fn new(stored_frame: usize, freedom_frame: usize) -> Self {
        let error = stored_frame as i32 - freedom_frame as i32;

        Self {
            correction: freedom_frame,
            precision: LinkPrecision::from_frame_diff(error),
        }
    }

    pub(super) fn meter_gain(&self) -> Option<i32> {
        self.precision.meter_gain()
    }

    pub(super) fn message(&self) -> String {
        self.precision.message()
    }
}

#[derive(Debug, Default, Reflect)]
pub(super) struct Cancellation {
    pub message: String,
}
impl Cancellation {
    pub(super) fn new(input_frame: usize, cancellable_since: usize) -> Self {
        Self {
            message: match input_frame.cmp(&cancellable_since) {
                Ordering::Equal => "Frame perfect cancel".to_owned(),
                Ordering::Greater => {
                    // Input frame came after it was cancellable
                    format!("Cancelled on frame {}", input_frame - cancellable_since)
                }
                Ordering::Less => format!(
                    // Input frame came before it was cancellable
                    "Cancel buffered for {} frames",
                    cancellable_since - input_frame
                ),
            },
        }
    }
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
                notifications.add(*player, "Throw clash".to_owned());
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
        let Some(freedom_frame) = state.free_since else {
            continue;
        };
        // Character has recently been freed

        let Some((stored, id, _)) = buffer
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

        let error = stored as i32 - freedom_frame as i32;
        let kind = if error.abs() < AUTOCORRECT as i32 {
            ActivationType::Link(Link::new(stored, freedom_frame))
        } else {
            ActivationType::Raw
        };

        buffer.activation = Some(MoveActivation { id, kind });
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
        let Some((stored, id, cancellable_since)) = buffer
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
            kind: ActivationType::Cancel(Cancellation::new(stored, cancellable_since)),
        });
    }
}

#[allow(clippy::type_complexity)]
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
        &Inventory,
        &InputParser,
    )>,
) {
    // Activate and clear activating move
    for (mut buffer, mut state, mut properties, player, character, stats, inventory, parser) in
        &mut query
    {
        let Some(activation) = buffer.activation.take() else {
            continue;
        };

        if state.unlock_frame().is_some() {
            continue;
        }

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
            inventory.to_owned(),
            properties.to_owned(),
            parser.to_owned(),
            stats.to_owned(),
        );
    }
}
