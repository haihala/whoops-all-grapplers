use bevy::prelude::*;

use characters::{Action, Character, MoveHistory, Situation};
use time::Clock;
use types::{MoveId, SoundEffect};

use crate::{assets::Sounds, ui::Notifications};

use super::PlayerQuery;

const AUTOCORRECT: usize = (0.2 * constants::FPS) as usize;

// +- frames. 0 is frame perfect, 1 means +-1 aka 3 frame window
const PERFECT_TIMING_DELTA: usize = 1;
const GOOD_TIMING_DELTA: usize = 5;

#[derive(Debug, Default, Component)]
pub struct MoveBuffer {
    buffer: Vec<(usize, MoveId)>,
}
impl MoveBuffer {
    fn add_events(&mut self, events: Vec<MoveId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn use_move(
        &mut self,
        character: &Character,
        situation: &Situation,
    ) -> Option<(MoveId, Option<i32>)> {
        if let Some((selected_id, _, frame)) = self
            .buffer
            .iter()
            .map(|(frame, id)| (*id, character.get_move(*id), *frame))
            .filter(|(_, move_data, _)| (move_data.requirement)(situation.clone()))
            // TODO: Special/normal considerations. Maybe add those dynamically to can_start somehow?
            // Do that when splitting move starting.
            .min_by(|(id1, _, _), (id2, _, _)| id1.cmp(id2))
        {
            self.buffer.retain(|(_, id)| selected_id != *id);
            Some((selected_id, Some(frame as i32)))
        } else {
            None
        }
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

    pub fn clear(&mut self) {
        *self = MoveBuffer::default();
    }
}

pub(super) fn move_activator(
    mut sounds: ResMut<Sounds>,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut query: Query<PlayerQuery>,
) {
    for mut player in query.iter_mut() {
        player.buffer.clear_old(clock.frame);
        player
            .buffer
            .add_events(player.input_parser.drain_events(), clock.frame);

        if player.state.stunned() {
            return;
        }

        if let Some(move_id) = player
            .state
            .drain_matching_actions(|action| {
                if let Action::Move(move_id) = action {
                    Some(*move_id)
                } else {
                    None
                }
            })
            .last()
        {
            player.state.start_move(MoveHistory {
                move_id: move_id.to_owned(),
                started: clock.frame,
                ..default()
            });
        }

        let situation = Situation {
            inventory: &player.inventory,
            history: player
                .state
                .get_move_history()
                .map(|history| history.to_owned()),
            grounded: player.state.is_grounded(),
            resources: &player.resources,
            parser: &player.input_parser,
            current_frame: clock.frame,
        };

        if let Some((move_id, stored_frame)) = player.buffer.use_move(player.character, &situation)
        {
            let started = if let Some(earliest_activation_frame) = player
                .state
                .free_since
                .filter(|free_since| *free_since as usize + AUTOCORRECT >= clock.frame)
                .or_else(|| {
                    situation
                        .history
                        .and_then(|history| history.cancellable_since)
                }) {
                if let Some(frame) = stored_frame {
                    // Not a forced start
                    // Make a toast
                    let frame_diff = earliest_activation_frame as i32 - frame;
                    let (notification_content, meter_gain) = get_combo_notification(frame_diff);

                    notifications.add(*player.player, notification_content);
                    player.resources.meter.gain(meter_gain);
                }
                earliest_activation_frame
            } else {
                clock.frame
            };

            sounds.play(SoundEffect::Whoosh); // TODO, make this an action
            player.state.start_move(MoveHistory {
                move_id,
                started,
                ..default()
            });
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
