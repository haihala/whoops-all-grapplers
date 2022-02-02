use bevy::prelude::*;
use input_parsing::InputParser;
use moves::{MoveBank, PhaseKind};
use player_state::{MoveState, PlayerState};
use time::Clock;
use types::{Grabable, LRDirection, Player};

use crate::{assets::Colors, damage::Health, physics::PlayerVelocity, spawner::Spawner};

#[allow(clippy::type_complexity)]
pub fn move_advancement(
    mut commands: Commands,
    colors: Res<Colors>,
    clock: Res<Clock>,
    mut players: Query<(
        &mut PlayerState,
        &mut Spawner,
        &MoveBank,
        Entity,
        &LRDirection,
        &Player,
        &Transform,
        &Grabable,
        &InputParser,
        &mut PlayerVelocity,
        &mut Health,
    )>,
) {
    let mut iter = players.iter_combinations_mut();
    if let Some(
        [(
            mut state1,
            mut spawner1,
            bank1,
            parent1,
            facing1,
            player1,
            tf1,
            grab_target1,
            parser1,
            mut velocity1,
            mut health1,
        ), (
            mut state2,
            mut spawner2,
            bank2,
            parent2,
            facing2,
            player2,
            tf2,
            grab_target2,
            parser2,
            mut velocity2,
            mut health2,
        )],
    ) = iter.fetch_next()
    {
        do_the_thing(
            &mut commands,
            &clock,
            &colors,
            &mut state1,
            &mut spawner1,
            bank1,
            parent1,
            facing1,
            *player1,
            tf1,
            &mut state2,
            &mut spawner2,
            tf2,
            grab_target2,
            parser2,
            &mut velocity2,
            &mut health2,
        );

        do_the_thing(
            &mut commands,
            &clock,
            &colors,
            &mut state2,
            &mut spawner2,
            bank2,
            parent2,
            facing2,
            *player2,
            tf2,
            &mut state1,
            &mut spawner1,
            tf1,
            grab_target1,
            parser1,
            &mut velocity1,
            &mut health1,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn do_the_thing(
    commands: &mut Commands,
    clock: &Clock,
    colors: &Res<Colors>,
    state1: &mut PlayerState,
    spawner1: &mut Spawner,
    bank: &MoveBank,
    parent: Entity,
    facing: &LRDirection,
    player: Player,
    tf1: &Transform,
    state2: &mut PlayerState,
    spawner2: &mut Spawner,
    tf2: &Transform,
    grab_target: &Grabable,
    parser: &InputParser,
    velocity: &mut PlayerVelocity,
    health: &mut Health,
) {
    if let Some(move_state) = state1.get_move_state() {
        let move_data = bank.get(move_state.move_id);
        if let Some(phase_index) = move_data.get_phase_index(move_state.start_frame, clock.frame) {
            if move_state.phase_index != phase_index {
                // Despawn old things
                spawner1.despawn_on_phase_change(commands);

                match move_data.get_phase(phase_index).kind {
                    PhaseKind::Attack(descriptor) => spawner1.spawn_attack(
                        move_state.move_id,
                        descriptor,
                        commands,
                        colors,
                        clock.frame,
                        parent,
                        facing,
                        player,
                        tf1.translation,
                    ),
                    PhaseKind::Grab(descriptor) => {
                        let grab_origin = tf1.translation + descriptor.offset.extend(0.0);
                        let distance = (grab_origin - tf2.translation).length();
                        let max_distance = grab_target.size + descriptor.range;
                        let in_range = distance <= max_distance;

                        let teched = state2.get_move_state().is_none() && parser.clear_head();

                        if in_range && !teched {
                            state2.throw();
                            spawner2.despawn_on_hit(commands);
                            velocity.add_impulse(descriptor.impulse);
                            health.apply_damage(descriptor.damage);
                        }
                    }
                    PhaseKind::Animation => {}
                };
                // Start next phase
                state1.set_move_state(MoveState {
                    phase_index,
                    ..move_state
                });
            }
        } else {
            // Move has ended
            spawner1.despawn_on_phase_change(commands);
            state1.recover();
        }
    }
}
