use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};
use input_parsing::InputParser;
use kits::{Grabable, Kit, MoveAction, PhaseKind, Resources};
use player_state::PlayerState;
use time::Clock;
use types::Players;

use crate::spawner::Spawner;

use super::move_activation::MoveBuffer;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    state: &'a mut PlayerState,
    spawner: &'a mut Spawner,
    kit: &'a Kit,
    tf: &'a Transform,
    grabbable: &'a mut Grabable,
    parser: &'a InputParser,
    buffer: &'a mut MoveBuffer,
    resources: &'a mut Resources,
}

pub fn move_advancement(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<PlayerQuery>,
    players: Res<Players>,
) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        advance_move(&mut commands, &clock, &mut p1, &mut p2);
        advance_move(&mut commands, &clock, &mut p2, &mut p1);
    }
}

fn advance_move(
    commands: &mut Commands,
    clock: &Clock,
    actor: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    target: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
    if let Some(ref mut move_state) = actor.state.get_move_state_mut() {
        move_state.buttons_held = actor.parser.get_pressed();

        let move_data = actor.kit.get_move(move_state.move_id);
        if let Some(phase_index) = move_data.get_action_index(move_state, clock.frame as i32) {
            if move_state.phase_index != phase_index {
                move_state.phase_index = phase_index;

                // Despawn old things
                actor.spawner.despawn_on_phase_change(commands);

                // Start next phase
                move_state.phase_index = phase_index;

                if let Some((action, requirements)) = move_data.get_action(move_state) {
                    if let Some(req) = requirements {
                        move_state.resources.pay(req.cost.to_owned());
                        actor.resources.pay(req.cost);
                    };

                    match action {
                        MoveAction::Move(move_id) => {
                            // The move has branched or recursed
                            actor
                                .buffer
                                .set_force_starter(move_id, actor.kit.get_move(move_id));
                            // TODO: Some buffer clearing here?
                        }
                        MoveAction::Phase(phase_data) => {
                            match phase_data.kind {
                                PhaseKind::Attack(descriptor) => {
                                    actor.spawner.add_to_queue(move_state.move_id, descriptor)
                                }
                                PhaseKind::Grab(descriptor) => {
                                    let grab_origin =
                                        actor.tf.translation + descriptor.offset.extend(0.0);
                                    let distance = (grab_origin - target.tf.translation).length();
                                    let max_distance = target.grabbable.size + descriptor.range;
                                    let in_range = distance <= max_distance;

                                    let teched = target.state.get_move_state().is_none()
                                        && target.parser.head_is_clear();

                                    if in_range && !teched {
                                        target.grabbable.queue.push(descriptor);
                                    }
                                }
                                PhaseKind::Animation => {}
                            };
                        }
                    }
                };
            }
        } else {
            // Move has ended
            actor.spawner.despawn_on_phase_change(commands);
            actor.state.recover();
        }
    }
}
