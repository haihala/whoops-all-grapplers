use bevy::{ecs::query::QueryData, prelude::*};
use strum::IntoEnumIterator;

use characters::{
    ActionEvent, Attack, AttackHeight, BlockType, Hitbox, Hurtbox, Movement, ResourceType,
    WAGResources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{
    ActionId, Area, Clock, Facing, Owner, Player, Players, SoundEffect, Stats, StatusFlag,
    StickPosition, VisualEffect, CLASH_PARRY_METER_GAIN, GI_PARRY_METER_GAIN,
};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
    physics::{PlayerVelocity, Pushbox},
    ui::Notifications,
};

use super::{hitboxes::ProjectileMarker, Combo, Defense, HitTracker, HitboxSpawner};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum ConnectionType {
    Strike,
    Block,
    Parry,
    Throw,
    Tech,
    Stunlock,
}

#[derive(Debug, PartialEq, Clone)]
pub(super) struct AttackConnection {
    attacker: Entity,
    defender: Entity,
    hitbox: Entity,
    overlap: Area,
    attack: Attack,
    contact_type: ConnectionType,
}

#[derive(QueryData)]
#[query_data(mutable)]
/// Used for querying all the components that are required when a player is hit.
pub struct HitPlayerQuery<'a> {
    defense: &'a mut Defense,
    tf: &'a mut Transform,
    properties: &'a mut WAGResources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a Facing,
    spawner: &'a mut HitboxSpawner,
    pushbox: &'a Pushbox,
    stats: &'a Stats,
}

#[allow(clippy::type_complexity)]
pub(super) fn clash_parry(
    mut hitboxes: Query<(
        &Owner,
        &GlobalTransform,
        &Hitbox,
        &mut HitTracker,
        &Attack,
        Option<&ProjectileMarker>,
    )>,
    clock: Res<Clock>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut owners: Query<&mut WAGResources>,
    players: Res<Players>,
) {
    let mut iter = hitboxes.iter_combinations_mut();
    while let Some(
        [(owner1, gtf1, hitbox1, tracker1, attack1, maybe_proj1), (owner2, gtf2, hitbox2, tracker2, attack2, maybe_proj2)],
    ) = iter.fetch_next()
    {
        if **owner1 == **owner2 {
            // Can't clash with your own boxes
            continue;
        }

        if !tracker1.active(clock.frame) || !tracker2.active(clock.frame) {
            continue;
        }

        if attack1.to_hit.block_type == BlockType::Grab
            || attack2.to_hit.block_type == BlockType::Grab
        {
            continue;
        }

        if let Some(overlap) = hitbox1
            .with_offset(gtf1.translation().truncate())
            .intersection(&hitbox2.with_offset(gtf2.translation().truncate()))
        {
            // Hitboxes collide
            sounds.play(SoundEffect::Clash);
            particles.spawn(ParticleRequest {
                effect: VisualEffect::Clash,
                position: overlap.center().extend(0.0),
            });

            for (mut tracker, owner, is_projectile) in [
                (tracker1, owner1, maybe_proj1.is_some()),
                (tracker2, owner2, maybe_proj2.is_some()),
            ] {
                let mut properties = owners.get_mut(players.get(**owner)).unwrap();

                // Pay up
                if !is_projectile {
                    properties
                        .get_mut(ResourceType::Meter)
                        .unwrap()
                        .gain(CLASH_PARRY_METER_GAIN);
                }

                // Despawn projectiles and consume hits
                if tracker.hits >= 1 {
                    tracker.register_hit(clock.frame);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn detect_hits(
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    combo: Option<Res<Combo>>,
    mut hitboxes: Query<(
        Entity,
        &Owner,
        &Attack,
        &GlobalTransform,
        &Hitbox,
        &mut HitTracker,
    )>,
    players: Res<Players>,
    hurtboxes: Query<(&Hurtbox, &Owner)>,
    defenders: Query<(&Transform, &PlayerState, &InputParser)>,
) -> Vec<AttackConnection> {
    hitboxes
        .iter_mut()
        .filter_map(
            |(hitbox_entity, hit_owner, attack, hitbox_tf, hitbox, mut hit_tracker)| {
                if !hit_tracker.active(clock.frame) {
                    return None;
                }

                let attacking_player = **hit_owner;
                let defending_player = hit_owner.other();

                let defender = players.get(defending_player);
                let attacker = players.get(**hit_owner);
                let (defender_tf, state, parser) = defenders.get(defender).unwrap();

                let offset_hitbox = hitbox.with_offset(hitbox_tf.translation().truncate());

                // This technically doesn't get the actual overlap, as it just gets some overlap with one of the hitboxes
                let overlap = hurtboxes.iter().find_map(|(hurtbox, hurt_owner)| {
                    if **hurt_owner == **hit_owner {
                        None
                    } else {
                        // Different owners, hit can register
                        hurtbox
                            .with_offset(defender_tf.translation.truncate())
                            .intersection(&offset_hitbox)
                    }
                })?;

                if state.is_intangible() {
                    if !hit_tracker.hit_intangible {
                        // Only send the notification once
                        hit_tracker.hit_intangible = true;
                        notifications.add(defending_player, "Intangible".to_owned());
                    }
                    return None;
                }

                if hit_tracker.hits >= 1 {
                    hit_tracker.register_hit(clock.frame)
                } else {
                    return None;
                }

                // Connection confirmed

                let (avoid_notification, contact_type) = match attack.to_hit.block_type {
                    BlockType::Strike(height) => {
                        let parrying = state.has_flag(StatusFlag::Parry) && state.is_grounded();
                        let (blocked, reason) =
                            handle_blocking(height, parser.get_relative_stick_position());

                        if parrying {
                            (Some("Parry!".into()), ConnectionType::Parry)
                        } else if blocked && state.can_block() {
                            (Some(reason), ConnectionType::Block)
                        } else {
                            (None, ConnectionType::Strike)
                        }
                    }
                    BlockType::Grab => {
                        if combo.is_some() {
                            (
                                Some("Can't grab from stun".into()),
                                ConnectionType::Stunlock,
                            )
                        } else if yomi_teched(parser)
                            && (state.can_block() && !state.action_in_progress())
                        {
                            (Some("Teched".into()), ConnectionType::Tech)
                        } else {
                            (None, ConnectionType::Throw)
                        }
                    }
                };

                if let Some(reason) = avoid_notification {
                    notifications.add(defending_player, format!("Avoid - {}", reason,));
                }

                if hit_tracker.hit_intangible {
                    // Just a nice notification for now.
                    notifications.add(attacking_player, "Meaty!".to_owned());
                }

                Some(AttackConnection {
                    defender,
                    attacker,
                    hitbox: hitbox_entity,
                    overlap,
                    attack: attack.to_owned(),
                    contact_type,
                })
            },
        )
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn apply_connections(
    In(mut hits): In<Vec<AttackConnection>>,
    mut commands: Commands,
    mut notifications: ResMut<Notifications>,
    combo: Option<Res<Combo>>,
    clock: Res<Clock>,
    mut players: Query<HitPlayerQuery>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
) {
    if hits.len() >= 2 {
        if hits
            .iter()
            .all(|hit| hit.contact_type == ConnectionType::Throw)
        {
            // Two grabs can't hit on the same frame
            for mut player in &mut players {
                player
                    .velocity
                    .add_impulse(player.facing.mirror_vec2(Vec2::X * -10.0));
                notifications.add(*player.player, "Throw clash".to_owned());
            }

            particles.spawn(ParticleRequest {
                effect: VisualEffect::Clash,
                position: hits
                    .iter()
                    .map(|hit| hit.overlap.center())
                    .reduce(|a, b| a + b)
                    .unwrap()
                    .extend(0.0)
                    * 0.5,
            });

            sounds.play(SoundEffect::Whoosh); // TODO change sound effect
            return;
        } else if hits
            .iter()
            .any(|hit| hit.contact_type == ConnectionType::Throw)
        {
            // On a same frame connect, grab beats strike
            hits.retain(|hit| hit.contact_type == ConnectionType::Throw);
        }
    }

    for hit in hits {
        let [mut attacker, mut defender] =
            players.get_many_mut([hit.attacker, hit.defender]).unwrap();

        let (mut attacker_actions, mut defender_actions, sound, particle, avoided) = match hit
            .contact_type
        {
            ConnectionType::Strike | ConnectionType::Throw => {
                // Handle blocking and state transitions here
                attacker.state.register_hit();
                defender.defense.reset();
                (
                    hit.attack.self_on_hit,
                    hit.attack.target_on_hit,
                    SoundEffect::Hit,
                    VisualEffect::Hit,
                    false,
                )
            }
            ConnectionType::Block => {
                attacker.state.register_hit();
                defender.defense.bump_streak(clock.frame);
                defender
                    .properties
                    .get_mut(ResourceType::Meter)
                    .unwrap()
                    .gain(defender.defense.get_reward());

                (
                    hit.attack.self_on_block,
                    if !defender.stats.chip_damage {
                        hit.attack
                            .target_on_block
                            .into_iter()
                            .filter(|ev| {
                                !matches!(ev, ActionEvent::ModifyResource(ResourceType::Health, _))
                            })
                            .collect()
                    } else {
                        hit.attack.target_on_block
                    },
                    SoundEffect::Block,
                    VisualEffect::Block,
                    true,
                )
            }
            ConnectionType::Parry => (
                vec![],
                vec![
                    ActionEvent::ModifyResource(ResourceType::Meter, GI_PARRY_METER_GAIN),
                    ActionEvent::StartAction(ActionId::ParryFlash),
                ],
                SoundEffect::Clash,
                VisualEffect::Clash,
                true,
            ),
            ConnectionType::Tech | ConnectionType::Stunlock => (
                vec![Movement::impulse(Vec2::X * -4.0).into()],
                vec![],
                SoundEffect::Clash,
                VisualEffect::Clash,
                true,
            ),
        };

        if !avoided {
            if combo.is_none() {
                commands.insert_resource(Combo);
                sounds.play(SoundEffect::Whoosh); // TODO change sound effect
                if attacker.stats.opener_damage_multiplier > 1.0 {
                    attacker_actions = handle_opener(attacker_actions, attacker.stats);
                    attacker_actions.push(ActionEvent::ModifyResource(
                        ResourceType::Meter,
                        attacker.stats.opener_meter_gain,
                    ));
                    defender_actions = handle_opener(defender_actions, attacker.stats);
                }
                notifications.add(*attacker.player, "Opener!".to_owned());
            } else if defender.stats.direct_influence > 0.0 {
                defender.velocity.add_impulse(defender.facing.mirror_vec2(
                    defender.parser.get_relative_stick_position().as_vec2()
                        * defender.stats.direct_influence,
                ));
            }
        }

        defender_actions =
            apply_damage_multiplier(defender_actions, attacker.stats.damage_multiplier);
        attacker.state.add_actions(attacker_actions);
        defender.state.add_actions(defender_actions);
        sounds.play(sound);
        particles.spawn(ParticleRequest {
            effect: particle,
            // TODO: This can be refined more
            position: hit.overlap.center().extend(0.0),
        });

        defender.spawner.despawn_on_hit();
    }
}

fn handle_blocking(height: AttackHeight, stick: StickPosition) -> (bool, String) {
    let blocking_high = stick == StickPosition::W;
    let blocking_low = stick == StickPosition::SW;

    if !(blocking_high || blocking_low) {
        (false, "Not blocking".into())
    } else if match height {
        AttackHeight::Low => blocking_low,
        AttackHeight::Mid => blocking_low || blocking_high,
        AttackHeight::High => blocking_high,
    } {
        (true, "Blocked!".into())
    } else {
        (false, format!("Hit {:?}", height))
    }
}

fn yomi_teched(parser: &InputParser) -> bool {
    parser.head_is_clear()
}

fn handle_opener(actions: Vec<ActionEvent>, status_effect: &Stats) -> Vec<ActionEvent> {
    actions
        .into_iter()
        .map(|action| match action {
            ActionEvent::ModifyResource(ResourceType::Health, amount) => {
                ActionEvent::ModifyResource(
                    ResourceType::Health,
                    (amount as f32 * status_effect.opener_damage_multiplier) as i32,
                )
            }
            ActionEvent::HitStun(amount) => {
                ActionEvent::HitStun((amount as i32 + status_effect.opener_stun_frames) as usize)
            }
            other => other,
        })
        .collect()
}
fn apply_damage_multiplier(actions: Vec<ActionEvent>, multiplier: f32) -> Vec<ActionEvent> {
    actions
        .into_iter()
        .map(|action| match action {
            ActionEvent::ModifyResource(ResourceType::Health, amount) => {
                ActionEvent::ModifyResource(
                    ResourceType::Health,
                    (amount as f32 * multiplier) as i32,
                )
            }
            other => other,
        })
        .collect()
}

pub(super) fn snap_and_switch(
    mut query: Query<(
        &mut PlayerState,
        &mut Transform,
        &mut Facing,
        &Pushbox,
        &mut PlayerVelocity,
    )>,
    players: Res<Players>,
) {
    for player in Player::iter() {
        let [(mut state, mut self_tf, mut self_facing, self_pushbox, mut self_velocity), (_, other_tf, mut other_facing, other_pushbox, other_velocity)] =
            query
                .get_many_mut([players.get(player), players.get(player.other())])
                .unwrap();
        let actions = state.drain_matching_actions(|action| {
            if matches!(
                *action,
                ActionEvent::SnapToOpponent | ActionEvent::SideSwitch
            ) {
                Some(action.to_owned())
            } else {
                None
            }
        });

        if actions.contains(&ActionEvent::SnapToOpponent) {
            let side_switch = actions.contains(&ActionEvent::SideSwitch);

            let raw_diff = self_tf.translation.x - other_tf.translation.x; // This ought to be positive when attacker is on the left
            let width_between = (self_pushbox.width() + other_pushbox.width()) / 2.0;

            self_tf.translation = other_tf.translation
                + Vec3::X
                    * raw_diff.signum()
                    * width_between
                    * (if side_switch { -1.0 } else { 1.0 });
            self_velocity.sync_with(&other_velocity);

            if side_switch {
                // Automatic side switcher won't run in time, animations will be fucked
                // Easier to just do this instead of fixing the system execution order
                (*self_facing, *other_facing) = (*other_facing, *self_facing);
            }
        }
    }
}

pub(super) fn stun_actions(
    mut query: Query<(&mut PlayerState, &mut PlayerVelocity, &Facing)>,
    clock: Res<Clock>,
) {
    for (mut state, mut velocity, facing) in &mut query {
        if state.active_cinematic().is_some() {
            continue;
        }

        for action in state.drain_matching_actions(|action| {
            if matches!(
                *action,
                ActionEvent::Launch { impulse: _ }
                    | ActionEvent::HitStun(_)
                    | ActionEvent::BlockStun(_)
            ) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            match action {
                ActionEvent::HitStun(frames) => state.stun(clock.frame + frames),
                ActionEvent::BlockStun(frames) => state.block(clock.frame + frames),
                ActionEvent::Launch { impulse } => {
                    state.launch();
                    velocity.add_impulse(facing.mirror_vec2(impulse));
                }
                _ => panic!("Leaking"),
            }
        }
    }
}
