use bevy::prelude::*;

use input_parsing::InputParser;
use kits::{Inventory, Kit, Move, MoveId, MoveSituation, Resources};
use player_state::PlayerState;
use time::Clock;

use crate::spawner::Spawner;
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
pub fn move_activator(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        &mut InputParser,
        &mut PlayerState,
        &mut MoveBuffer,
        &Kit,
        &mut Resources,
        &Inventory,
        &mut Spawner,
    )>,
) {
    for (mut reader, mut state, mut buffer, kit, mut resources, inventory, mut spawner) in
        query.iter_mut()
    {
        buffer.clear_old(clock.frame);
        buffer.add_events(reader.drain_events(), clock.frame);

        if state.stunned() {
            continue;
        }

        let mut situation = state
            .get_move_state()
            .map(|state| state.to_owned())
            .unwrap_or(MoveSituation {
                resources: *resources,
                inventory: inventory.clone(),
                buttons_held: reader.get_pressed().clone(),
                grounded: state.is_grounded(),
                ..default()
            });

        if let Some((move_id, move_data)) = buffer.use_move(kit, &situation) {
            situation.move_id = move_id;
            situation.start_frame = clock.frame as i32;
            situation.resources.pay(move_data.requirements.cost.clone());
            situation.hit_registered = false;
            situation.cancel_level = move_data.requirements.cancel_level.unwrap();

            resources.pay(move_data.requirements.cost);
            spawner.despawn_on_phase_change(&mut commands);
            state.start_move(situation);
        }
    }
}
