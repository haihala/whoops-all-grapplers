use bevy::prelude::*;

use characters::{Action, Character, Inventory, Move, MoveHistory, Resources, Situation};
use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use types::{MoveId, Player};

use crate::ui::Notifications;

const AUTOCORRECT: usize = (0.2 * constants::FPS) as usize;

// +- frames. 0 is frame perfect, 1 means +-1 aka 3 frame window
const PERFECT_TIMING_DELTA: usize = 1;
const GOOD_TIMING_DELTA: usize = 5;

#[derive(Debug)]
struct MoveActivation {
    kind: ActivationType,
    id: MoveId,
}

#[derive(Debug)]
enum ActivationType {
    Continuation,
    Raw,
    Link(Timing),
    Cancel(Timing),
}

#[derive(Debug)]
struct Timing {
    /// How much meter / what toast to give
    error: i32,
    /// The frame when the move is said to have started
    correction: usize,
}

#[derive(Debug, Default, Component)]
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
                let move_data = character.get_move(*id);
                if (move_data.requirement)(situation.to_owned()) {
                    Some((*frame, *id, move_data.to_owned()))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub(super) fn manage_buffer(
    clock: Res<Clock>,
    mut query: Query<(&mut MoveBuffer, &mut InputParser)>,
) {
    // Read from the input parser and fill the buffer
    for (mut buffer, mut parser) in query.iter_mut() {
        buffer.clear_old(clock.frame);
        buffer.add_events(parser.drain_events(), clock.frame);
    }
}
pub(super) fn move_continuation(mut query: Query<(&mut MoveBuffer, &mut PlayerState)>) {
    // Read from state, set activating move if an Action demands it
    for (mut buffer, mut state) in query.iter_mut() {
        let move_continuations = state.drain_matching_actions(|action| {
            if let Action::Move(move_id) = action {
                Some(*move_id)
            } else {
                None
            }
        });
        if move_continuations.len() == 1 {
            buffer.activation = Some(MoveActivation {
                kind: ActivationType::Continuation,
                id: move_continuations[0],
            })
        } else if move_continuations.len() > 1 {
            // TODO: Maybe handle this by resolving until one of them can start?
            todo!("Multiple moves to continue")
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
    for (mut buffer, character, state, inventory, resources, parser) in query.iter_mut() {
        if let Some(freedom) = state.free_since {
            // Character has recently been freed

            if let Some((stored, id, _)) = buffer
                .get_situation_moves(
                    &character,
                    Situation {
                        inventory,
                        history: state.get_move_history().map(|history| history.to_owned()),
                        grounded: state.is_grounded(),
                        resources,
                        parser,
                        current_frame: clock.frame,
                    },
                )
                .into_iter()
                .min_by(|(_, id1, _), (_, id2, _)| id1.cmp(id2))
            {
                let error = stored as i32 - freedom as i32;
                let kind = if error.abs() < AUTOCORRECT as i32 {
                    ActivationType::Link(Timing {
                        error,
                        correction: freedom,
                    })
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
    for (mut buffer, character, state, inventory, resources, parser) in query.iter_mut() {
        if state.free_since.is_none() {
            if let Some(history) = state.get_move_history() {
                // Not free because a move is happening
                // Is current move cancellable, if so, since when
                if let Some((stored, id, freedom)) = buffer
                    .get_situation_moves(
                        &character,
                        Situation {
                            inventory,
                            history: state.get_move_history().map(|history| history.to_owned()),
                            grounded: state.is_grounded(),
                            resources,
                            parser,
                            current_frame: clock.frame,
                        },
                    )
                    .into_iter()
                    .filter_map(|(frame, id, data)| {
                        if let Some(freedom) = history.cancellable_into_since(&data) {
                            Some((frame, id, freedom))
                        } else {
                            None
                        }
                    })
                    .min_by(|(_, id1, _), (_, id2, _)| id1.cmp(id2))
                {
                    let error = stored as i32 - freedom as i32;
                    if error.abs() < AUTOCORRECT as i32 {
                        buffer.activation = Some(MoveActivation {
                            id,
                            kind: ActivationType::Cancel(Timing {
                                error,
                                correction: freedom,
                            }),
                        });
                    }
                }
            }
        }
    }
}

pub(super) fn move_activator(
    clock: Res<Clock>,
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
    for (mut buffer, mut state, mut resources, player, character) in query.iter_mut() {
        if let Some(activation) = buffer.activation.take() {
            let started = if let ActivationType::Link(timing) | ActivationType::Cancel(timing) =
                activation.kind
            {
                let (message, meter_gain) = get_combo_notification(timing.error);

                notifications.add(*player, message);
                resources.meter.gain(meter_gain);

                timing.correction
            } else {
                clock.frame
            };

            state.start_move(MoveHistory {
                move_id: activation.id,
                move_data: character.get_move(activation.id),
                started,
                ..default()
            });
            buffer.buffer.retain(|(_, id)| *id != activation.id);
        }
    }
}

const MIDDLE_LEN: usize = 1 + 2 * PERFECT_TIMING_DELTA;
const EDGE_LEN: usize = GOOD_TIMING_DELTA - PERFECT_TIMING_DELTA;
const BUFFER_LEN: usize = AUTOCORRECT - EDGE_LEN;

const METER_GAIN_ON_PERFECT: i32 = 30;
const METER_GAIN_ON_GOOD: i32 = 10;

fn get_combo_notification(frame_diff: i32) -> (String, i32) {
    // TODO: This could use a cleanup
    let mut middle = "-".repeat(MIDDLE_LEN);
    let mut good_left_edge = "-".repeat(EDGE_LEN);
    let mut good_right_edge = good_left_edge.clone();
    let mut buffer_left_edge = "-".repeat(BUFFER_LEN);
    let mut buffer_right_edge = buffer_left_edge.clone();

    let abs_diff = frame_diff.unsigned_abs() as usize;
    let (word, gain) = if abs_diff <= PERFECT_TIMING_DELTA {
        // Perfect timing
        let bound = (frame_diff + 1) as usize;
        middle.replace_range(bound..bound + 1, "x");

        ("Perfect", METER_GAIN_ON_PERFECT)
    } else if abs_diff <= GOOD_TIMING_DELTA {
        // Good timing
        let index = (abs_diff - PERFECT_TIMING_DELTA).min(EDGE_LEN - 1);
        if frame_diff > 0 {
            good_left_edge.replace_range(EDGE_LEN - index..EDGE_LEN - index + 1, "x");
        } else {
            good_right_edge.replace_range(index..index + 1, "x");
        }

        ("Good", METER_GAIN_ON_GOOD)
    } else {
        let index = (abs_diff - GOOD_TIMING_DELTA).min(BUFFER_LEN - 1);
        if frame_diff > 0 {
            buffer_left_edge.replace_range(BUFFER_LEN - index..BUFFER_LEN - index + 1, "x");
            ("Early", 0)
        } else {
            buffer_right_edge.replace_range(index..index + 1, "x");
            ("Late", 0)
        }
    };

    (
        format!(
            "{}: {}[{}[{}]{}]{}",
            word, buffer_left_edge, good_left_edge, middle, good_right_edge, buffer_right_edge
        ),
        gain,
    )
}
