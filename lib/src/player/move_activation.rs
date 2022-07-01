use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use kits::{Kit, Move, MoveId, MoveSituation};
use time::Clock;
use types::{Players, SoundEffect};

use crate::assets::Sounds;

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

    fn use_move(&mut self, kit: &Kit, situation: &MoveSituation) -> Option<(MoveId, Move)> {
        if self.force_start.is_some() {
            // Early return for the cases when a move has forked
            return self.force_start.take();
        }

        if let Some((selected_id, move_data)) = self
            .buffer
            .iter()
            .map(|(_, id)| (*id, kit.get_move(*id)))
            .filter(|(_, move_data)| situation.fulfills(&move_data.requirements))
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
    mut query: Query<PlayerQuery>,
) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        activate_move(&mut commands, &mut sounds, &clock, &mut p1, &mut p2);
        activate_move(&mut commands, &mut sounds, &clock, &mut p2, &mut p1);
    }
}

fn activate_move(
    commands: &mut Commands,
    sounds: &mut ResMut<Sounds>,
    clock: &Res<Clock>,
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

    let mut situation = actor
        .state
        .get_move_state()
        .map(|state| state.to_owned())
        .unwrap_or(MoveSituation {
            resources: *actor.resources,
            inventory: actor.inventory.clone(),
            buttons_held: actor.input_parser.get_pressed(),
            grounded: actor.state.is_grounded(),
            ..default()
        });

    if let Some((move_id, move_data)) = actor.buffer.use_move(actor.kit, &situation) {
        situation.move_id = move_id;
        situation.start_frame = clock.frame as i32;
        situation.resources.pay(move_data.requirements.cost.clone());
        situation.hit_registered = false;
        situation.cancel_level = move_data.requirements.cancel_level.unwrap();

        actor.resources.pay(move_data.requirements.cost);
        actor.state.start_move(situation);
        sounds.play(SoundEffect::Whoosh);
        activate_phase(commands, 0, actor, target);
    }
}
