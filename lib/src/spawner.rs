use bevy::prelude::*;
use bevy::utils::HashMap;

use player_state::{PlayerState, StateEvent};
use types::{LRDirection, Lifetime, MoveId, Player, PlayerCollisionEffect, SpawnDescriptor};

use crate::assets::Colors;
use crate::clock::Clock;
use crate::game_flow::GameState;
use crate::physics::ConstantVelocity;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_hitbox_events.system())
            .add_system(despawn_expired.system())
            .add_system(despawn_on_phase_change.system())
            .add_system_set(
                SystemSet::on_exit(GameState::Combat).with_system(despawn_everything.system()),
            );
    }
}

#[derive(Debug, Clone, Copy)]
enum DespawnTime {
    Frame(usize),
    StateChange,
    OnHit,
    EndOfRound,
}

struct DespawnRequest {
    id: MoveId,
    time: DespawnTime,
}

#[derive(Default)]
pub struct Spawner {
    spawned: HashMap<MoveId, Entity>,
    despawn_requests: Vec<DespawnRequest>,
}
impl Spawner {
    #[allow(clippy::too_many_arguments)]
    fn spawn_attack(
        &mut self,
        id: MoveId,
        descriptor: SpawnDescriptor,
        commands: &mut Commands,
        colors: &Res<Colors>,
        frame: usize,
        parent: Entity,
        facing: &LRDirection,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.mirror_vec(descriptor.hitbox.offset);
        let translation = if descriptor.attached_to_player {
            offset
        } else {
            parent_position + offset
        };

        let mut builder = commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation,
                ..Default::default()
            },
            material: colors.hurtbox.clone(),
            sprite: Sprite::new(descriptor.hitbox.size),
            ..Default::default()
        });

        // Components used when collision happens
        builder.insert(PlayerCollisionEffect {
            owner: player,
            fixed_height: descriptor.fixed_height,
            damage: descriptor.damage,
            stun: descriptor.stun,
            knockback: descriptor.knockback,
            pushback: descriptor.pushback,
        });

        if let Some(speed) = descriptor.speed {
            builder.insert(ConstantVelocity::new(facing.to_vec3() * speed));
        }

        // Housekeeping
        let new_hitbox = builder.id();
        if descriptor.attached_to_player {
            commands.entity(parent).push_children(&[new_hitbox]);
        }
        self.spawned.insert(id, new_hitbox);
        self.despawn_requests.push(DespawnRequest {
            id,
            time: match descriptor.lifetime {
                Lifetime::Phase => DespawnTime::StateChange,
                Lifetime::UntilHit => DespawnTime::OnHit,
                Lifetime::Frames(time_to_live) => DespawnTime::Frame(frame + time_to_live),
                Lifetime::Forever => DespawnTime::EndOfRound,
            },
        });
    }

    fn despawn(&mut self, commands: &mut Commands, ids: Vec<MoveId>) {
        for id in ids.into_iter() {
            if let Some(spawned) = self.spawned.get(&id) {
                commands.entity(*spawned).despawn();
                self.spawned.remove(&id);
            }
        }
    }

    fn drain(&mut self, predicate: impl Fn(&mut DespawnRequest) -> bool) -> Vec<MoveId> {
        self.despawn_requests
            .drain_filter(predicate)
            .map(|event| event.id)
            .collect()
    }

    fn drain_old(&mut self, frame: usize) -> Vec<MoveId> {
        self.drain(|event| {
            if let DespawnTime::Frame(despawn_frame) = event.time {
                despawn_frame <= frame
            } else {
                false
            }
        })
    }

    pub fn despawn_on_hit(&mut self, commands: &mut Commands) {
        let ids = self.drain(|event| {
            matches!(event.time, DespawnTime::OnHit)
            // Getting hit changes the state
                || matches!(event.time, DespawnTime::StateChange)
        });

        self.despawn(commands, ids);
    }
}

pub fn handle_hitbox_events(
    mut commands: Commands,
    clock: Res<Clock>,
    colors: Res<Colors>,
    mut query: Query<(
        &mut Spawner,
        &mut PlayerState,
        Entity,
        &LRDirection,
        &Player,
        &Transform,
    )>,
) {
    for (mut spawner, mut state, parent, facing, player, parent_tf) in query.iter_mut() {
        for event in state.get_events() {
            if let StateEvent::Attack(id, descriptor) = event {
                spawner.spawn_attack(
                    id,
                    descriptor,
                    &mut commands,
                    &colors,
                    clock.frame,
                    parent,
                    facing,
                    *player,
                    parent_tf.translation,
                );

                state.consume_event(event);
            }
        }
    }
}

pub fn despawn_expired(
    mut commands: Commands,
    clock: Res<Clock>,
    mut spawners: Query<&mut Spawner>,
) {
    for mut spawner in spawners.iter_mut() {
        let ids = spawner.drain_old(clock.frame);

        spawner.despawn(&mut commands, ids);
    }
}

pub fn despawn_on_phase_change(
    mut commands: Commands,
    mut spawners: Query<(&mut Spawner, &mut PlayerState)>,
) {
    for (mut spawner, mut state) in spawners.iter_mut() {
        for event in state.get_events() {
            if matches!(event, StateEvent::PhaseChange) {
                let ids = spawner.drain(|event| matches!(event.time, DespawnTime::StateChange));
                spawner.despawn(&mut commands, ids);
                state.consume_event(event);
            }
        }
    }
}

pub fn despawn_everything(mut commands: Commands, mut spawners: Query<&mut Spawner>) {
    for mut spawner in spawners.iter_mut() {
        let ids = spawner.spawned.drain().map(|(id, _)| id).collect();
        spawner.despawn(&mut commands, ids);
    }
}
