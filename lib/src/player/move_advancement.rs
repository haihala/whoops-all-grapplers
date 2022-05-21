use bevy::prelude::*;
use input_parsing::InputParser;
use kits::{Grabable, Kit, Move, MoveAction, MoveSituation, PhaseKind, Resources};
use player_state::PlayerState;
use time::Clock;

use crate::spawner::Spawner;

use super::move_activation::MoveBuffer;

#[allow(clippy::type_complexity)]
pub fn move_advancement(
    mut commands: Commands,
    clock: Res<Clock>,
    mut players: Query<(
        &mut PlayerState,
        &mut Spawner,
        &Kit,
        &Transform,
        &mut Grabable,
        &InputParser,
        &mut MoveBuffer,
        &mut Resources,
    )>,
) {
    let mut iter = players.iter_combinations_mut();
    if let Some([mut p1, mut p2]) = iter.fetch_next() {
        advance_move(&mut commands, &clock, &mut p1, &mut p2);
        advance_move(&mut commands, &clock, &mut p2, &mut p1);
    }
}

// Bevy 0.7 has a better solution for these
type ComponentList<'a> = (
    Mut<'a, PlayerState>,
    Mut<'a, Spawner>,
    &'a Kit,
    &'a Transform,
    Mut<'a, Grabable>,
    &'a InputParser,
    Mut<'a, MoveBuffer>,
    Mut<'a, Resources>,
);

fn advance_move(
    commands: &mut Commands,
    clock: &Clock,
    actor: &mut ComponentList,
    target: &mut ComponentList,
) {
    let (state1, spawner1, kit, tf1, _, attacker_parser, buffer, attacker_resources) = actor;
    let (state2, _, _, tf2, grab_target, defender_parser, _, _) = target;

    if let Some(move_state) = state1.get_move_state_mut() {
        move_state.buttons_held = attacker_parser.get_pressed();

        let move_data = kit.get_move(move_state.move_id);
        if let Some(phase_index) = move_data.get_action_index(move_state, clock.frame as i32) {
            if move_state.phase_index != phase_index {
                move_state.phase_index = phase_index;

                // Despawn old things
                spawner1.despawn_on_phase_change(commands);

                handle_new_phase(
                    move_data,
                    move_state,
                    attacker_resources,
                    buffer,
                    kit,
                    spawner1,
                    tf1.translation.to_owned(),
                    tf2.translation.to_owned(),
                    grab_target,
                    defender_parser,
                    state2,
                );

                // Start next phase
                state1.set_move_phase_index(phase_index);
            }
        } else {
            // Move has ended
            spawner1.despawn_on_phase_change(commands);
            state1.recover();
        }
    }
}

// Fix this in Bevy 0.7
#[allow(clippy::too_many_arguments)]
fn handle_new_phase(
    move_data: Move,
    move_state: &mut MoveSituation,
    attacker_resources: &mut Resources,
    attacker_buffer: &mut MoveBuffer,
    kit: &Kit,
    attacker_spawner: &mut Spawner,
    attacker_position: Vec3,
    defender_position: Vec3,
    grab_target: &mut Grabable,
    defender_parser: &InputParser,
    defender_spawner: &mut PlayerState,
) {
    if let Some((action, requirements)) = move_data.get_action(move_state) {
        if let Some(req) = requirements {
            move_state.resources.pay(req.cost.to_owned());
            attacker_resources.pay(req.cost);
        };

        match action {
            MoveAction::Move(move_id) => {
                // The move has branched or recursed
                attacker_buffer.set_force_starter(move_id, kit.get_move(move_id));
                // TODO: Some buffer clearing here?
            }
            MoveAction::Phase(phase_data) => {
                match phase_data.kind {
                    PhaseKind::Attack(descriptor) => {
                        attacker_spawner.add_to_queue(move_state.move_id, descriptor)
                    }
                    PhaseKind::Grab(descriptor) => {
                        let grab_origin = attacker_position + descriptor.offset.extend(0.0);
                        let distance = (grab_origin - defender_position).length();
                        let max_distance = grab_target.size + descriptor.range;
                        let in_range = distance <= max_distance;

                        let teched = defender_spawner.get_move_state().is_none()
                            && defender_parser.head_is_clear();

                        if in_range && !teched {
                            grab_target.queue.push(descriptor);
                        }
                    }
                    PhaseKind::Animation => {}
                };
            }
        }
    }
}
