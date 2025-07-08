use bevy::{ecs::query::QueryData, prelude::*};

use characters::{
    ActionEvent, Attack, AttackHeight, BlockType, Character, GaugeType, Gauges, HitEffect, HitInfo,
    Hitbox, Hurtboxes, Inventory,
};
use foundation::{
    Area, CharacterClock, CharacterFacing, Clock, Combo, Owner, Player, Players, Sound,
    SoundRequest, Stats, StatusFlag, StickPosition, VfxRequest, VisualEffect,
    CLASH_PARRY_METER_GAIN, GI_PARRY_METER_GAIN,
};
use input_parsing::InputParser;
use player_state::PlayerState;

use crate::{
    event_spreading::{
        LaunchImpulse, SnapToOpponent, SpawnVfx, UpdateBlockstun, UpdateHitstun, ZoomCamera,
    },
    movement::{PlayerVelocity, Pushbox},
    ui::Notifications,
};

use super::{hitboxes::ProjectileMarker, HitTracker, HitboxSpawner};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum ConnectionType {
    Strike,
    Block,
    Parry,
    Throw,
    Tech,
    Stunlock,
}

#[derive(Clone)]
pub(super) struct AttackConnection {
    attacker: Entity,
    defender: Entity,
    overlap: Area,
    attack: Attack,
    contact_type: ConnectionType,
}

#[derive(QueryData)]
#[query_data(mutable)]
/// Used for querying all the components that are required when a player is hit.
pub struct HitPlayerQuery<'a> {
    tf: &'a mut Transform,
    properties: &'a mut Gauges,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a CharacterFacing,
    spawner: &'a mut HitboxSpawner,
    stats: &'a Stats,
    combo: &'a mut Combo,
    character: &'a Character,
    inventory: &'a Inventory,
    clock: &'a CharacterClock,
}

#[allow(clippy::type_complexity)]
pub(super) fn clash_parry(
    mut commands: Commands,
    mut hitboxes: Query<(
        &Owner,
        &Transform,
        &Hitbox,
        &mut HitTracker,
        &Attack,
        Option<&ProjectileMarker>,
    )>,
    clock: Res<Clock>,
    mut owners: Query<&mut Gauges>,
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
            .with_offset(gtf1.translation.truncate())
            .intersection(&hitbox2.with_offset(gtf2.translation.truncate()))
        {
            // Hitboxes collide
            commands.trigger(SoundRequest::from(Sound::GlassClink));
            commands.trigger(SpawnVfx(
                VfxRequest {
                    effect: VisualEffect::Clash,
                    tf: Transform::from_translation(overlap.center().extend(0.0)),
                    ..default()
                },
                None,
            ));

            for (mut tracker, owner, is_projectile) in [
                (tracker1, owner1, maybe_proj1.is_some()),
                (tracker2, owner2, maybe_proj2.is_some()),
            ] {
                let mut properties = owners.get_mut(players.get(**owner)).unwrap();

                // Pay up
                if !is_projectile {
                    properties
                        .get_mut(GaugeType::Meter)
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

#[allow(clippy::type_complexity)]
pub(super) fn detect_hits(
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut hitboxes: Query<(&Owner, &Attack, &Transform, &Hitbox, &mut HitTracker)>,
    players: Res<Players>,
    defenders: Query<(
        &Transform,
        &CharacterFacing,
        &Hurtboxes,
        &PlayerState,
        &InputParser,
    )>,
    attackers: Query<Option<&Combo>>,
) -> Vec<AttackConnection> {
    hitboxes
        .iter_mut()
        .filter_map(|(hit_owner, attack, hitbox_tf, hitbox, mut hit_tracker)| {
            if !hit_tracker.active(clock.frame) {
                return None;
            }

            let attacking_player = **hit_owner;
            let defending_player = hit_owner.other();

            let defender = players.get(defending_player);
            let attacker = players.get(**hit_owner);
            let (defender_tf, facing, hurtboxes, state, parser) = defenders.get(defender).unwrap();
            let combo = attackers.get(attacker).unwrap();

            let offset_hitbox = hitbox.with_offset(hitbox_tf.translation.truncate());

            // This technically doesn't get the actual overlap, as it just gets some overlap with one of the hitboxes
            let overlap = hurtboxes.as_vec().iter().find_map(|hurtbox| {
                // Different owners, hit can register
                hurtbox
                    .with_center(
                        facing.visual.mirror_vec2(hurtbox.center())
                            + defender_tf.translation.truncate(),
                    )
                    .intersection(&offset_hitbox)
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
                    let (blocked, reason) = handle_blocking(
                        height,
                        facing.absolute.mirror_stick_pos(parser.get_stick_pos()),
                    );

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
                notifications.add(defending_player, format!("Avoid - {reason}"));
            }

            if hit_tracker.hit_intangible {
                // Just a nice notification for now.
                notifications.add(attacking_player, "Meaty!".to_owned());
            }

            Some(AttackConnection {
                defender,
                attacker,
                overlap,
                attack: attack.to_owned(),
                contact_type,
            })
        })
        .collect()
}

pub fn apply_connections(
    In(mut hits): In<Vec<AttackConnection>>,
    mut commands: Commands,
    mut notifications: ResMut<Notifications>,
    mut players: Query<HitPlayerQuery>,
    abs_clock: Res<Clock>,
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
                    .add_impulse(player.facing.absolute.mirror_vec2(Vec2::X * -10.0));
                notifications.add(*player.player, "Throw clash".to_owned());
            }

            commands.trigger(SpawnVfx(
                VfxRequest {
                    effect: VisualEffect::Clash,
                    tf: Transform::from_translation(
                        hits.iter()
                            .map(|hit| hit.overlap.center())
                            .reduce(|a, b| a + b)
                            .unwrap()
                            .extend(0.0)
                            * 0.5,
                    ),
                    ..default()
                },
                None,
            ));

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

        let avoided = match hit.contact_type {
            ConnectionType::Strike | ConnectionType::Throw => {
                attacker.state.register_hit();
                false
            }
            ConnectionType::Block => {
                attacker.state.register_hit();
                if defender.stats.defense_meter != 0 {
                    defender
                        .properties
                        .get_mut(GaugeType::Meter)
                        .unwrap()
                        .gain(defender.stats.defense_meter);
                }

                true
            }
            ConnectionType::Tech | ConnectionType::Stunlock => true,
            ConnectionType::Parry => {
                commands.trigger(ZoomCamera(0.3));
                commands.trigger(SoundRequest::from(Sound::Clash));
                commands.trigger_targets(
                    ActionEvent::ModifyResource(GaugeType::Meter, GI_PARRY_METER_GAIN),
                    hit.defender,
                );
                commands.trigger(SpawnVfx(
                    VfxRequest {
                        effect: VisualEffect::Clash,
                        tf: Transform::from_translation(hit.overlap.center().extend(0.0)),
                        ..default()
                    },
                    None,
                ));

                defender.state.register_hit();

                return;
            }
        };

        let situation = attacker.state.build_situation(
            attacker.inventory.to_owned(),
            attacker.properties.to_owned(),
            attacker.parser.to_owned(),
            attacker.stats.to_owned(),
            attacker.clock.frame,
            abs_clock.frame,
            attacker.tf.translation,
            attacker.facing.to_owned(),
            attacker.combo.to_owned(),
        );
        let HitEffect {
            attacker: mut attacker_actions,
            defender: mut defender_actions,
        } = (hit.attack.on_hit)(
            &situation,
            &HitInfo {
                avoided,
                airborne: !defender.state.is_grounded(),
                hitbox_pos: hit.overlap.center(),
                defender_stats: *defender.stats,
            },
        );

        if !avoided {
            if attacker.combo.ongoing() {
                attacker.combo.hits += 1;
            } else {
                // First hit of a combo
                attacker
                    .combo
                    .start_at(defender.properties.get(GaugeType::Health).unwrap().current);

                commands.trigger(SoundRequest::from(Sound::Matches));
                notifications.add(*attacker.player, "Opener!".to_owned());
                if attacker.stats.opener_damage_multiplier > 1.0 {
                    attacker_actions = handle_opener(attacker_actions, attacker.stats);
                    attacker_actions.push(ActionEvent::ModifyResource(
                        GaugeType::Meter,
                        attacker.stats.opener_meter_gain,
                    ));
                    defender_actions = handle_opener(defender_actions, attacker.stats);
                }
            }

            // This may break throws
            if defender.stats.direct_influence > 0.0 {
                defender.velocity.add_impulse(
                    defender.parser.get_stick_pos().as_vec2() * defender.stats.direct_influence,
                );
            }
        }

        defender_actions =
            apply_damage_multiplier(defender_actions, attacker.stats.damage_multiplier);

        for event in attacker_actions {
            commands.trigger_targets(event, hit.attacker);
        }
        for event in defender_actions {
            commands.trigger_targets(event, hit.defender);
        }

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
        (false, format!("Hit {height:?}"))
    }
}

fn yomi_teched(parser: &InputParser) -> bool {
    parser.head_is_clear()
}

fn handle_opener(actions: Vec<ActionEvent>, status_effect: &Stats) -> Vec<ActionEvent> {
    actions
        .into_iter()
        .map(|action| match action {
            ActionEvent::ModifyResource(GaugeType::Health, amount) => ActionEvent::ModifyResource(
                GaugeType::Health,
                (amount as f32 * status_effect.opener_damage_multiplier) as i32,
            ),
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
            ActionEvent::ModifyResource(GaugeType::Health, amount) => {
                ActionEvent::ModifyResource(GaugeType::Health, (amount as f32 * multiplier) as i32)
            }
            other => other,
        })
        .collect()
}

pub fn snap_and_switch(
    trigger: Trigger<SnapToOpponent>,
    mut query: Query<(
        &mut Transform,
        &mut CharacterFacing,
        &Pushbox,
        &mut PlayerVelocity,
    )>,
    players: Res<Players>,
) {
    let [(mut self_tf, mut self_facing, self_pushbox, mut self_velocity), (other_tf, mut other_facing, other_pushbox, other_velocity)] =
        query
            .get_many_mut([trigger.target(), players.get_other_entity(trigger.target())])
            .unwrap();

    let raw_diff = self_tf.translation.x - other_tf.translation.x; // This ought to be positive when attacker is on the left
    let width_between = (self_pushbox.width() + other_pushbox.width()) / 2.0;

    self_tf.translation = other_tf.translation
        + Vec3::X
            * raw_diff.signum()
            * width_between
            * (if trigger.event().sideswitch {
                -1.0
            } else {
                1.0
            });
    self_velocity.sync_with(&other_velocity);

    if trigger.event().sideswitch {
        // Automatic side switcher won't run in time, animations will be fucked
        // Easier to just do this instead of fixing the system execution order
        (*self_facing, *other_facing) = (*other_facing, *self_facing);
    }
}

pub fn hitstun_events(
    trigger: Trigger<UpdateHitstun>,
    mut query: Query<(&mut PlayerState, &CharacterClock)>,
) {
    let (mut state, clock) = query.get_mut(trigger.target()).unwrap();
    state.hit_stun(clock.frame + trigger.event().0);
}

pub fn blockstun_events(
    trigger: Trigger<UpdateBlockstun>,
    mut query: Query<(&mut PlayerState, &CharacterClock)>,
) {
    let (mut state, clock) = query.get_mut(trigger.target()).unwrap();
    state.block(clock.frame + trigger.event().0);
}

pub fn launch_events(
    trigger: Trigger<LaunchImpulse>,
    mut query: Query<(&mut PlayerState, &mut PlayerVelocity, &CharacterFacing)>,
) {
    let (mut state, mut velocity, facing) = query.get_mut(trigger.target()).unwrap();
    state.launch();
    velocity.add_impulse(facing.visual.mirror_vec2(trigger.event().0));
}
