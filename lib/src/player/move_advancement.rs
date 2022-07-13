use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};
use characters::{MoveAction, PhaseKind};
use time::Clock;
use types::Players;

use crate::ui::Notifications;

use super::PlayerQuery;

pub(super) fn move_advancement(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<PlayerQuery>,
    players: Res<Players>,
    mut notifications: ResMut<Notifications>,
) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        advance_move(&mut commands, &clock, &mut notifications, &mut p1, &mut p2);
        advance_move(&mut commands, &clock, &mut notifications, &mut p2, &mut p1);
    }
}

fn advance_move(
    commands: &mut Commands,
    clock: &Clock,
    notifications: &mut ResMut<Notifications>,
    actor: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    target: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
    let mut index_to_activate = None;
    if let Some(move_state) = actor.state.get_move_state_mut() {
        move_state.buttons_held = actor.input_parser.get_pressed();

        let move_data = actor.character.get_move(move_state.move_id);
        if let Some(phase_index) = move_data.get_action_index(move_state, clock.frame as i32) {
            if move_state.phase_index != phase_index {
                index_to_activate = Some(phase_index);
            }
        } else {
            // Move has ended
            actor.spawner.despawn_on_phase_change(commands);
            actor.state.recover();
        }
    }
    if let Some(phase_index) = index_to_activate {
        // Avoid simultaneous burrows and to make interface manageable
        activate_phase(commands, phase_index, notifications, actor, target);
    }
}

pub(super) fn activate_phase(
    commands: &mut Commands,
    phase_index: usize,
    notifications: &mut ResMut<Notifications>,
    actor: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    target: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
    if let Some(move_state) = actor.state.get_move_state_mut() {
        let move_data = actor.character.get_move(move_state.move_id);

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
                        .set_force_starter(move_id, actor.character.get_move(move_id));
                    // TODO: Some buffer clearing here?
                }
                MoveAction::Phase(phase_data) => {
                    move_state.cancellable = phase_data.cancellable;

                    match phase_data.kind {
                        PhaseKind::Attack(descriptor) => actor.spawner.add_to_queue(descriptor),
                        PhaseKind::Grab(descriptor) => {
                            let grab_origin = actor.tf.translation + descriptor.offset.extend(0.0);
                            let distance = (grab_origin - target.tf.translation).length();
                            let max_distance = target.grabbable.size + descriptor.range;
                            let in_range = distance <= max_distance;

                            let teched = target.state.get_move_state().is_none()
                                && target.input_parser.head_is_clear();

                            if teched {
                                notifications.add(target.player.to_owned(), "Teched!".into());
                            } else if in_range {
                                target.grabbable.queue.push(descriptor);
                            }
                        }
                        PhaseKind::Animation => {}
                    };
                }
            }
        }
    }
}
