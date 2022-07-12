use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use characters::{Character, Move, MoveId, MoveSituation};
use time::Clock;
use types::{Players, SoundEffect};

use crate::{assets::Sounds, ui::Notifications};

use super::{move_advancement::activate_phase, PlayerQuery};
const EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
const FRAMES_TO_LIVE_IN_BUFFER: usize = (EVENT_REPEAT_PERIOD * constants::FPS) as usize;

#[derive(Debug, Default, Component)]
pub struct MoveBuffer {
    buffer: Vec<(usize, MoveId)>,
    force_start: Option<(MoveId, Move)>,
}
impl MoveBuffer {
    pub fn set_force_starter(&mut self, move_id: MoveId, move_data: Move) {
        self.force_start = Some((move_id, move_data));
    }

    fn add_events(&mut self, events: Vec<MoveId>, frame: usize) {
        self.buffer.extend(events.into_iter().map(|id| (frame, id)));
    }

    fn use_move(
        &mut self,
        character: &Character,
        situation: &MoveSituation,
    ) -> Option<(MoveId, Move)> {
        if let Some((selected_id, move_data)) = self
            .buffer
            .iter()
            .map(|(_, id)| (*id, character.get_move(*id)))
            .filter(|(_, move_data)| {
                situation.fulfills(&move_data.requirements, Some(move_data.move_type))
            })
            .min_by(|(id1, _), (id2, _)| id1.cmp(id2))
        {
            self.buffer.retain(|(_, id)| selected_id != *id);
            Some((selected_id, move_data))
        } else {
            None
        }
    }

    fn clear_old(&mut self, current_frame: usize) {
        self.buffer.retain(|(frame, _)| {
            if current_frame > *frame {
                // Default case, retain those who are fresh
                current_frame - frame < FRAMES_TO_LIVE_IN_BUFFER
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

    // As a move is either happening or not happening, one of the 'or' options will always be Some if the user has a move they are trying to get out
    if let Some((move_id, move_data)) = force_start.or_else(|| {
        actor
            .state
            .get_move_state()
            .map(|state| state.to_owned())
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
        dbg!(move_id);
        actor.resources.pay(move_data.requirements.cost);
        sounds.play(SoundEffect::Whoosh);
        actor.state.start_move(MoveSituation {
            move_id,
            move_type: Some(move_data.move_type),
            start_frame: clock.frame as i32,
            resources: actor.resources.to_owned(),
            inventory: actor.inventory.to_owned(),
            ..default()
        });
        activate_phase(commands, 0, notifications, actor, target);
    }
}
