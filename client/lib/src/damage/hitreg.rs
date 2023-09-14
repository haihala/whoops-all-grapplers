use bevy::{ecs::query::WorldQuery, prelude::*};
use strum::IntoEnumIterator;

use characters::{
    ActionEvent, Attack, AttackHeight, BlockType, Character, Hitbox, Hurtbox, ResourceType,
    WAGResources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{
    Area, Clock, Facing, Owner, Player, Players, SoundEffect, Stats, StatusFlag, StickPosition,
    VisualEffect, CLASH_PARRY_METER_GAIN, GI_PARRY_METER_GAIN,
};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
    physics::{PlayerVelocity, Pushbox},
    ui::Notifications,
};

use super::{Combo, Defense, HitTracker, HitboxSpawner};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum HitType {
    Strike,
    Block,
    Parry,
    Throw,
}

#[derive(Debug, PartialEq, Clone)]
pub(super) struct Hit {
    attacker: Entity,
    defender: Entity,
    hitbox: Entity,
    overlap: Area,
    hit_type: HitType,
    attack: Attack,
    is_opener: bool,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
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
    status_effect: &'a Stats,
}

pub(super) fn clash_parry(
    mut commands: Commands,
    clock: Res<Clock>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut hitboxes: Query<(Entity, &Owner, &GlobalTransform, &Hitbox, &mut HitTracker)>,
    mut owners: Query<(&mut HitboxSpawner, &mut WAGResources)>,
    players: Res<Players>,
) {
    let mut iter = hitboxes.iter_combinations_mut();
    while let Some(
        [(entity1, owner1, gtf1, hitbox1, tracker1), (entity2, owner2, gtf2, hitbox2, tracker2)],
    ) = iter.fetch_next()
    {
        if **owner1 == **owner2 {
            // Can't clash with your own boxes
            continue;
        }

        if !tracker1.active(clock.frame) || !tracker2.active(clock.frame) {
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

            for (mut tracker, entity, owner) in
                [(tracker1, entity1, owner1), (tracker2, entity2, owner2)]
            {
                let (mut spawner, mut properties) = owners.get_mut(players.get(**owner)).unwrap();

                // Pay up
                let is_projectile = spawner
                    .is_projectile(entity)
                    .expect("to only check projectiles that have been spawned by this spawner");

                if !is_projectile {
                    properties
                        .get_mut(&ResourceType::Meter)
                        .unwrap()
                        .gain(CLASH_PARRY_METER_GAIN);
                }

                // Despawn projectiles and consume hits
                if tracker.hits <= 1 {
                    spawner.despawn(&mut commands, entity);
                } else {
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
    mut commands: Commands,
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
    defenders: Query<(&Transform, &PlayerState, &Character, &InputParser)>,
    mut spawners: Query<&mut HitboxSpawner>,
) -> Vec<Hit> {
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
                let (defender_tf, state, character, parser) = defenders.get(defender).unwrap();

                let offset_hitbox = hitbox.with_offset(hitbox_tf.translation().truncate());

                // This technically doesn't get the actual overlap, as it just gets some overlap with one of the hitboxes
                let Some(overlap) = hurtboxes.iter().find_map(|(hurtbox, hurt_owner)| {
                    if **hurt_owner == **hit_owner {
                        None
                    } else {
                        // Different owners, hit can register
                        hurtbox
                            .with_offset(defender_tf.translation.truncate())
                            .intersection(&offset_hitbox)
                    }
                }) else {
                    return None;
                };

                if state.is_intangible() {
                    if !hit_tracker.hit_intangible {
                        // Only send the notification once
                        hit_tracker.hit_intangible = true;
                        notifications.add(defending_player, "Intangible".to_owned());
                    }
                    return None;
                } else if hit_tracker.hit_intangible {
                    // Just a nice notification for now.
                    notifications.add(attacking_player, "Meaty!".to_owned());
                }

                if hit_tracker.hits <= 1 {
                    spawners
                        .get_mut(attacker)
                        .unwrap()
                        .despawn(&mut commands, hitbox_entity);
                } else {
                    hit_tracker.register_hit(clock.frame)
                }

                let (hit_type, notification) = if state.action_in_progress() {
                    (
                        match attack.to_hit.block_type {
                            BlockType::Constant(_) | BlockType::Dynamic => HitType::Strike,
                            BlockType::Grab => HitType::Throw,
                        },
                        "Busy".into(),
                    )
                } else if state.has_flag(StatusFlag::Parry)
                    && attack.to_hit.block_type != BlockType::Grab
                {
                    (HitType::Parry, "Parry!".into())
                } else {
                    match attack.to_hit.block_type {
                        BlockType::Constant(height) => {
                            handle_blocking(height, parser.get_relative_stick_position())
                        }
                        BlockType::Grab => {
                            if teched(parser) {
                                notifications.add(defending_player, "Teched".into());

                                return None;
                            }

                            (HitType::Throw, "Grappled".into())
                        }
                        BlockType::Dynamic => handle_blocking(
                            if overlap.bottom() > character.high_block_height {
                                AttackHeight::High
                            } else if overlap.top() > character.low_block_height {
                                AttackHeight::Mid
                            } else {
                                AttackHeight::Low
                            },
                            parser.get_relative_stick_position(),
                        ),
                    }
                };

                // TODO: This could be moved into hit processing, as it's not really relevant to hit recognition
                let is_opener = combo.is_none() && hit_type == HitType::Strike;
                if combo.is_none() {
                    notifications.add(
                        defending_player,
                        format!(
                            "{} - {}",
                            if is_opener { "Opener!" } else { "Avoid" },
                            notification,
                        ),
                    );
                }

                Some(Hit {
                    defender,
                    attacker,
                    hitbox: hitbox_entity,
                    overlap,
                    hit_type,
                    is_opener,
                    attack: attack.to_owned(),
                })
            },
        )
        .collect()
}

pub(super) fn apply_hits(
    In(hits): In<Vec<Hit>>,
    mut commands: Commands,
    combo: Option<Res<Combo>>,
    clock: Res<Clock>,
    mut players: Query<HitPlayerQuery>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
) {
    for hit in hits {
        let [mut attacker, mut defender] =
            players.get_many_mut([hit.attacker, hit.defender]).unwrap();

        // Hit has happened
        if combo.is_none() {
            commands.insert_resource(Combo);
        }

        let (mut attacker_actions, mut defender_actions, sound, particle) = match hit.hit_type {
            HitType::Strike | HitType::Throw => {
                // Handle blocking and state transitions here
                attacker.state.register_hit();
                defender.defense.reset();

                (
                    hit.attack.self_on_hit,
                    hit.attack.target_on_hit,
                    SoundEffect::Hit,
                    VisualEffect::Hit,
                )
            }
            HitType::Block => {
                attacker.state.register_hit(); // TODO: Specify it was blocked
                defender.defense.bump_streak(clock.frame);
                defender
                    .properties
                    .get_mut(&ResourceType::Meter)
                    .unwrap()
                    .gain(defender.defense.get_reward());

                (
                    hit.attack.self_on_block,
                    hit.attack.target_on_block,
                    SoundEffect::Block,
                    VisualEffect::Block,
                )
            }
            HitType::Parry => (
                vec![],
                vec![ActionEvent::ModifyProperty(
                    ResourceType::Meter,
                    GI_PARRY_METER_GAIN,
                )],
                SoundEffect::Clash,
                VisualEffect::Clash,
            ),
        };

        if hit.is_opener {
            sounds.play(SoundEffect::Whoosh); // TODO change sound effect
            attacker_actions = handle_opener(attacker_actions, attacker.status_effect);
            attacker_actions.push(ActionEvent::ModifyProperty(
                ResourceType::Meter,
                attacker.status_effect.opener_meter_gain,
            ));
            defender_actions = handle_opener(defender_actions, attacker.status_effect);
        }

        attacker.state.add_actions(attacker_actions);
        defender.state.add_actions(defender_actions);
        sounds.play(sound);
        particles.spawn(ParticleRequest {
            effect: particle,
            // TODO: This can be refined more
            position: hit.overlap.center().extend(0.0),
        });

        defender.spawner.despawn_on_hit(&mut commands);
    }
}

fn handle_blocking(height: AttackHeight, stick: StickPosition) -> (HitType, String) {
    let blocking_high = stick == StickPosition::W;
    let blocking_low = stick == StickPosition::SW;

    if !(blocking_high || blocking_low) {
        (HitType::Strike, "Not blocking".into())
    } else if match height {
        AttackHeight::Low => blocking_low,
        AttackHeight::Mid => blocking_low || blocking_high,
        AttackHeight::High => blocking_high,
    } {
        (HitType::Block, "Blocked!".into())
    } else {
        (HitType::Strike, format!("Hit {:?}", height))
    }
}

fn teched(parser: &InputParser) -> bool {
    parser.head_is_clear()
}

fn handle_opener(actions: Vec<ActionEvent>, status_effect: &Stats) -> Vec<ActionEvent> {
    actions
        .into_iter()
        .map(|action| match action {
            ActionEvent::ModifyProperty(ResourceType::Health, amount) => {
                ActionEvent::ModifyProperty(
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

pub(super) fn snap_and_switch(
    mut query: Query<(
        &mut PlayerState,
        &mut Transform,
        &Pushbox,
        &mut PlayerVelocity,
    )>,
    players: Res<Players>,
) {
    for player in Player::iter() {
        let [(mut state, mut self_tf, self_pushbox, mut self_velocity), (_, other_tf, other_pushbox, other_velocity)] =
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
            let switch = actions.contains(&ActionEvent::SideSwitch) as i32 as f32;

            let raw_diff = self_tf.translation.x - other_tf.translation.x; // This ought to be positive when attacker is on the left
            let width_between = (self_pushbox.width() + other_pushbox.width()) / 2.0;

            let new_position = other_tf.translation
                + Vec3::X * raw_diff.signum() * width_between * (1.0 - (2.0 * switch));

            self_tf.translation = new_position;
            self_velocity.sync_with(&other_velocity);
        }
    }
}

pub(super) fn stun_actions(mut query: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in &mut query {
        for action in state.drain_matching_actions(|action| {
            if matches!(
                *action,
                ActionEvent::Launch | ActionEvent::HitStun(_) | ActionEvent::BlockStun(_)
            ) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            match action {
                ActionEvent::HitStun(frames) => state.stun(clock.frame + frames),
                ActionEvent::BlockStun(frames) => state.block(clock.frame + frames),
                ActionEvent::Launch => state.launch(),
                _ => panic!("Leaking"),
            }
        }
    }
}
