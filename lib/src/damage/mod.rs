use bevy::prelude::*;

use input_parsing::InputParser;
use kits::{Grabable, Hurtbox, OnHitEffect, Resources};
use player_state::PlayerState;
use time::{Clock, GameState, WAGStage};
use types::{LRDirection, Owner, Player};

mod health;
pub use health::Health;

use crate::{
    physics::{rect_collision, PlayerVelocity},
    spawner::Spawner,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            WAGStage::HitReg,
            SystemSet::new()
                .with_system(register_hits)
                .with_system(handle_grabs.after(register_hits))
                .with_system(
                    health::check_dead
                        .after(handle_grabs)
                        .with_run_criteria(State::on_update(GameState::Combat)),
                ),
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut hitboxes: Query<(&Owner, &OnHitEffect, &GlobalTransform, &Sprite)>,
    mut hurtboxes: Query<(
        &Hurtbox,
        &Sprite,
        &Transform,
        &mut Health,
        &mut Resources,
        &Player,
        &InputParser,
        &mut PlayerState,
        &mut PlayerVelocity,
        &LRDirection,
        &mut Spawner,
    )>,
) {
    for (owner, effect, hitbox_tf, hitbox_sprite) in hitboxes.iter_mut() {
        let hitbox_position = hitbox_tf.translation;
        let hitbox_size = hitbox_sprite.custom_size.unwrap();

        let mut players = hurtboxes.iter_combinations_mut();
        if let Some([mut p1, mut p2]) = players.fetch_next() {
            handle_hit(
                &mut commands,
                clock.frame,
                effect,
                owner,
                hitbox_position,
                hitbox_size,
                &mut p1,
                &mut p2,
            );
            handle_hit(
                &mut commands,
                clock.frame,
                effect,
                owner,
                hitbox_position,
                hitbox_size,
                &mut p2,
                &mut p1,
            );
        }
    }
}

type ComponentList<'a> = (
    &'a Hurtbox,
    &'a Sprite,
    &'a Transform,
    Mut<'a, Health>,
    Mut<'a, Resources>,
    &'a Player,
    &'a InputParser,
    Mut<'a, PlayerState>,
    Mut<'a, PlayerVelocity>,
    &'a LRDirection,
    Mut<'a, Spawner>,
);

// Bevy 0.7 will fix
#[allow(clippy::too_many_arguments)]
fn handle_hit(
    commands: &mut Commands,
    frame: usize,
    effect: &OnHitEffect,
    owner: &Owner,
    hitbox_position: Vec3,
    hitbox_size: Vec2,
    attacker: &mut ComponentList,
    defender: &mut ComponentList,
) {
    let (
        _,
        _,
        _,
        _,
        attacker_resources,
        _,
        _,
        attacker_state,
        attacker_velocity,
        _,
        attacker_spawner,
    ) = attacker;
    let (
        hurtbox,
        hurtbox_sprite,
        defender_tf,
        health,
        _,
        defending_player,
        parser,
        defender_state,
        defender_velocity,
        facing,
        defender_spawner,
    ) = defender;

    if owner.0 == **defending_player {
        // You can't hit yourself
        return;
    }

    if rect_collision(
        defender_tf.translation + hurtbox.offset,
        hurtbox_sprite.custom_size.unwrap(),
        hitbox_position,
        hitbox_size,
    ) {
        attacker_state.register_hit();
        // Hit has happened
        // Handle blocking and state transitions here

        let blocked = defender_state.blocked(
            effect.fixed_height,
            hitbox_position.y + hitbox_size.y / 2.0,
            parser.get_relative_stick_position(),
        );

        // Damage and meter gain
        if let Some(damage_prop) = effect.damage {
            let amount = damage_prop.get(blocked);
            health.apply_damage(amount);
            attacker_resources.meter.add_combo_meter(amount);
        }

        // Knockback
        let knockback_impulse = effect
            .knockback
            // Knockback is positive aka away from attacker, so defender must flip it the other way
            .map(|knockback_prop| facing.opposite().mirror_vec(knockback_prop.get(blocked)))
            .unwrap_or_default();
        defender_velocity.add_impulse(knockback_impulse);

        // Pushback
        if let Some(pushback_prop) = effect.pushback {
            attacker_velocity.add_impulse(facing.mirror_vec(pushback_prop.get(blocked)));
        }

        // Stun
        if let Some(stun_prop) = effect.stun {
            if knockback_impulse.y > 0.0 {
                defender_state.launch();
            } else {
                defender_state.stun(stun_prop.get(blocked) + frame);
            }
        }

        // Despawns
        defender_spawner.despawn_on_hit(commands);
        attacker_spawner.despawn(commands, vec![effect.id]);
    }
}

fn handle_grabs(
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
