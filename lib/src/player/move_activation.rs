use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use characters::{Character, Move, MoveId, MoveSituation};
use time::Clock;
use types::{Players, SoundEffect};

use crate::{assets::Sounds, ui::Notifications};

use super::{move_advancement::activate_phase, PlayerQuery};

const AUTOCORRECT: usize = (0.2 * constants::FPS) as usize;

// +- frames. 0 is frame perfect, 1 means +-1 aka 3 frame window
const PERFECT_TIMING_DELTA: usize = 1;
const GOOD_TIMING_DELTA: usize = 5;

#[derive(Debug, Default, Component)]
pub struct MoveBuffer {
    buffer: Vec<(usize, MoveId)>,
    force_start: Option<(MoveId, Move, Option<i32>)>,
}
impl MoveBuffer {
    pub fn set_force_starter(&mut self, move_id: MoveId, move_data: Move) {
        self.force_start = Some((move_id, move_data, None));
    }

    fn add_events(&mut self, events: Vec<MoveId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn use_move(
        &mut self,
        character: &Character,
        situation: &MoveSituation,
    ) -> Option<(MoveId, Move, Option<i32>)> {
        if let Some((selected_id, move_data, frame)) = self
            .buffer
            .iter()
            .map(|(frame, id)| (*id, character.get_move(*id), *frame))
            .filter(|(_, move_data, _)| {
                situation.fulfills(&move_data.requirements, Some(move_data.move_type))
            })
            .min_by(|(id1, _, _), (id2, _, _)| id1.cmp(id2))
        {
            self.buffer.retain(|(_, id)| selected_id != *id);
            Some((selected_id, move_data, Some(frame as i32)))
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

#[allow(clippy::type_complexity)]
pub(super) fn move_activator(
    mut commands: Commands,
    mut sounds: ResMut<Sounds>,
    players: Res<Players>,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut query: Query<PlayerQuery>,
) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        activate_move(
            &mut commands,
            &mut sounds,
            &clock,
            &mut notifications,
            &mut p1,
            &mut p2,
        );
        activate_move(
            &mut commands,
            &mut sounds,
            &clock,
            &mut notifications,
            &mut p2,
            &mut p1,
        );
    }
}

fn activate_move(
    commands: &mut Commands,
    sounds: &mut ResMut<Sounds>,
    clock: &Res<Clock>,
    notifications: &mut ResMut<Notifications>,
    actor: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    target: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
    actor.buffer.clear_old(clock.frame);
    actor
        .buffer
        .add_events(actor.input_parser.drain_events(), clock.frame);

    if actor.state.stunned() {
        return;
    }

    let force_start = if actor.buffer.force_start.is_some() {
        actor.buffer.force_start.take()
    } else {
        None
    };

    let ongoing_move_situation = actor.state.get_move_state().map(|state| state.to_owned());

    // As a move is either happening or not happening, one of the 'or' options will always be Some if the user has a move they are trying to get out
    if let Some((move_id, move_data, stored_frame)) = force_start.or_else(|| {
        ongoing_move_situation
            .clone()
            .or_else(|| {
                Some(MoveSituation {
                    // Construct a pseudo-situation. This is one that represents the current state without a move.
                    // Some of the fields like start frame will be off, but those aren't relevant for move activation
                    resources: actor.resources.to_owned(),
                    inventory: actor.inventory.to_owned(),
                    buttons_held: actor.input_parser.get_pressed(),
                    grounded: actor.state.is_grounded(),
                    ..default()
                })
            })
            .and_then(|situation| actor.buffer.use_move(actor.character, &situation))
    }) {
        let start_frame = if let Some(earliest_activation_frame) = actor
            .state
            .free_since
            .filter(|free_since| *free_since as usize + AUTOCORRECT >= clock.frame)
            .or_else(|| ongoing_move_situation.and_then(|sit| sit.cancellable_since))
        {
            if let Some(frame) = stored_frame {
                // Not a forced start
                // Make a toast
                let frame_diff = earliest_activation_frame as i32 - frame;
                let (notification_content, meter_gain) = get_combo_notification(frame_diff);

                notifications.add(*actor.player, notification_content);
                actor.resources.meter.gain(meter_gain);
            }
            earliest_activation_frame
        } else {
            clock.frame
        } as i32;

        actor.resources.pay(move_data.requirements.cost);
        sounds.play(SoundEffect::Whoosh);
        actor.state.start_move(MoveSituation {
            move_id,
            start_frame,
            cost: move_data.requirements.cost,
            move_type: Some(move_data.move_type),
            resources: actor.resources.to_owned(),
            inventory: actor.inventory.to_owned(),
            ..default()
        });
        activate_phase(commands, 0, clock.frame, notifications, actor, target);
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
