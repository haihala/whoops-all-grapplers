use bevy::{ecs::query::WorldQuery, prelude::*};
use characters::{Action, Grabable};
use input_parsing::InputParser;
use player_state::PlayerState;
use types::{Facing, Player, Players};

use crate::{physics::PlayerVelocity, ui::Notifications};

use super::{Health, HitboxSpawner};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(super) struct PlayerQuery<'a> {
    tf: &'a Transform,
    input_parser: &'a InputParser,
    state: &'a mut PlayerState,
    facing: &'a Facing,
    grabbable: &'a mut Grabable,
    player: &'a Player,
}

pub(super) fn spawn_grabs(
    mut notifications: ResMut<Notifications>,
    mut query: Query<PlayerQuery>,
    players: Res<Players>,
) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        handle_grabs(&mut notifications, &mut p1, &mut p2);
        handle_grabs(&mut notifications, &mut p2, &mut p1);
    }
}
fn handle_grabs(
    notifications: &mut Notifications,
    actor: &mut PlayerQueryItem,
    target: &mut PlayerQueryItem,
) {
    for descriptor in actor.state.drain_matching_actions(|action| {
        if let Action::Grab(gd) = action {
            Some(*gd)
        } else {
            None
        }
    }) {
        let grab_origin = actor.tf.translation + descriptor.offset.extend(0.0);
        let distance = (grab_origin - target.tf.translation).length();
        let max_distance = target.grabbable.size + descriptor.range;
        let in_range = distance <= max_distance;

        let teched =
            target.state.get_move_history().is_none() && target.input_parser.head_is_clear();

        if teched {
            notifications.add(target.player.to_owned(), "Teched!".into());
        } else if in_range {
            target.grabbable.queue.push(descriptor);
        }
    }
}

pub(super) fn register_grabs(
    mut commands: Commands,
    mut query: Query<(
        &mut Grabable,
        &mut PlayerState,
        &mut HitboxSpawner,
        &mut PlayerVelocity,
        &mut Health,
        &Facing,
    )>,
) {
    for (mut grab_target, mut state, mut spawner, mut velocity, mut health, &facing) in &mut query {
        for descriptor in grab_target.queue.drain(..).collect::<Vec<_>>().into_iter() {
            state.throw();
            spawner.despawn_on_hit(&mut commands);
            // Facing is from the one being thrown, but we want to write the vector from the attacker's perspective
            velocity.add_impulse(facing.opposite().mirror_vec(descriptor.impulse));
            health.apply_damage(descriptor.damage);
        }
    }
}
