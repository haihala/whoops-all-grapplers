use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use input_parsing::InputParser;
use kits::{Grabable, Hurtbox, OnHitEffect, Resources};
use player_state::PlayerState;
use time::Clock;
use types::{LRDirection, Owner, Player, Players, SoundEffect};

use crate::{
    assets::Sounds,
    physics::{hybrid_vec_rect_collision, PlayerVelocity},
    spawner::Spawner,
};

use super::Health;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    hurtbox: &'a Hurtbox,
    sprite: &'a Sprite,
    tf: &'a Transform,
    health: &'a mut Health,
    resources: &'a mut Resources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a LRDirection,
    spawner: &'a mut Spawner,
}

pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    sounds: Res<Sounds>,
    audio: Res<Audio>,
    mut hitboxes: Query<(&Owner, &OnHitEffect, &GlobalTransform, &Sprite)>,
    mut hurtboxes: Query<PlayerQuery>,
    players: Res<Players>,
) {
    for (owner, effect, hitbox_tf, hitbox_sprite) in hitboxes.iter_mut() {
        let hitbox_position = hitbox_tf.translation;
        let hitbox_size = hitbox_sprite.custom_size.unwrap();
        let hitbox = bevy::sprite::Rect {
            min: hitbox_position.truncate() - hitbox_size / 2.0,
            max: hitbox_position.truncate() + hitbox_size / 2.0,
        };

        if let Ok([mut p1, mut p2]) = hurtboxes.get_many_mut([players.one, players.two]) {
            let (attacker, defender) = if owner.0 == Player::One {
                (&mut p1, &mut p2)
            } else {
                (&mut p2, &mut p1)
            };

            handle_hit(
                &mut commands,
                clock.frame,
                &sounds,
                &audio,
                effect,
                hitbox,
                attacker,
                defender,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_hit(
    commands: &mut Commands,
    frame: usize,
    sounds: &Res<Sounds>,
    audio: &Res<Audio>,
    effect: &OnHitEffect,
    hitbox: bevy::sprite::Rect,
    attacker: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    defender: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
    if hybrid_vec_rect_collision(
        defender.tf.translation + defender.hurtbox.offset,
        defender.sprite.custom_size.unwrap(),
        hitbox,
    ) {
        attacker.state.register_hit();
        // Hit has happened
        // Handle blocking and state transitions here

        let blocked = defender.state.blocked(
            effect.fixed_height,
            hitbox.min.y,
            defender.parser.get_relative_stick_position(),
        );

        // Damage and meter gain
        if let Some(damage_prop) = effect.damage {
            let amount = damage_prop.get(blocked);
            defender.health.apply_damage(amount);
            attacker.resources.meter.add_combo_meter(amount);
        }

        // Knockback
        let knockback_impulse = effect
            .knockback
            // Knockback is positive aka away from attacker, so defender must flip it the other way
            .map(|knockback_prop| {
                defender
                    .facing
                    .opposite()
                    .mirror_vec(knockback_prop.get(blocked))
            })
            .unwrap_or_default();
        defender.velocity.add_impulse(knockback_impulse);

        // Pushback
        if let Some(pushback_prop) = effect.pushback {
            attacker
                .velocity
                // More intuitive to think of it from the defenders perspective
                .add_impulse(defender.facing.mirror_vec(pushback_prop.get(blocked)));
        }

        // Stun
        if let Some(stun_prop) = effect.stun {
            if knockback_impulse.y > 0.0 {
                defender.state.launch();
            } else {
                defender.state.stun(stun_prop.get(blocked) + frame);
            }
        }

        // Sound effect
        audio.play(sounds.get(if blocked {
            SoundEffect::Block
        } else {
            SoundEffect::Hit
        }));

        // Despawns
        defender.spawner.despawn_on_hit(commands);
        attacker.spawner.despawn_for_move(commands, effect.id);
    }
}

pub fn handle_grabs(
    mut commands: Commands,
    mut query: Query<(
        &mut Grabable,
        &mut PlayerState,
        &mut Spawner,
        &mut PlayerVelocity,
        &mut Health,
    )>,
) {
    for (mut grab_target, mut state, mut spawner, mut velocity, mut health) in query.iter_mut() {
        for descriptor in grab_target.queue.drain(..).collect::<Vec<_>>().into_iter() {
            state.throw();
            spawner.despawn_on_hit(&mut commands);
            velocity.add_impulse(descriptor.impulse);
            health.apply_damage(descriptor.damage);
        }
    }
}
