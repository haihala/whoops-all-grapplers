use bevy::prelude::*;

use bevy::utils::HashMap;
use kits::{Lifetime, MoveId, OnHitEffect, SpawnDescriptor};
use time::{Clock, GameState};
use types::{LRDirection, Owner, Player};

use crate::assets::Colors;
use crate::physics::ConstantVelocity;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(spawn_new)
                .with_system(despawn_expired.after(spawn_new))
                .with_system(
                    despawn_everything
                        .with_run_criteria(State::on_exit(GameState::Combat))
                        // Technically despawning everything after expired is stupid,
                        // but as of resolving ordering conflicts for a few hours I can't be bothered to do it properly.
                        .after(despawn_expired),
                ),
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

#[derive(Debug)]
struct DespawnRequest {
    entity: Entity,
    time: DespawnTime,
}

#[derive(Default, Component)]
pub struct Spawner {
    queue: Vec<(MoveId, SpawnDescriptor)>,
    spawned: HashMap<Entity, MoveId>,
    despawn_requests: Vec<DespawnRequest>,
}
impl Spawner {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn_attack(
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
        let absolute_position = parent_position + offset;
        let transform = Transform::from_translation(if descriptor.attached_to_player {
            offset
        } else {
            absolute_position
        });

        let mut builder = commands.spawn_bundle(SpriteBundle {
            transform,
            global_transform: GlobalTransform {
                translation: absolute_position,
                ..default()
            },
            sprite: Sprite {
                color: colors.hurtbox,
                custom_size: Some(descriptor.hitbox.size),
                ..default()
            },
            ..default()
        });

        // Housekeeping
        let new_hitbox = builder.id();

        // Components used when collision happens
        builder.insert(OnHitEffect {
            id,
            fixed_height: descriptor.fixed_height,
            damage: descriptor.damage,
            stun: descriptor.stun,
            knockback: descriptor.knockback,
            pushback: descriptor.pushback,
        });
        builder.insert(Owner(player));

        if let Some(speed) = descriptor.speed {
            builder.insert(ConstantVelocity::new(facing.to_vec3() * speed));
        }

        if descriptor.attached_to_player {
            commands.entity(parent).push_children(&[new_hitbox]);
        }
        self.spawned.insert(new_hitbox, id);
        self.despawn_requests.push(DespawnRequest {
            entity: new_hitbox,
            time: match descriptor.lifetime {
                Lifetime::Phase => DespawnTime::StateChange,
                Lifetime::UntilHit => DespawnTime::OnHit,
                Lifetime::Frames(time_to_live) => DespawnTime::Frame(frame + time_to_live),
                Lifetime::Forever => DespawnTime::EndOfRound,
            },
        });
    }

    pub fn despawn(&mut self, commands: &mut Commands, ids: Vec<Entity>) {
        for id in ids.into_iter() {
            if self.spawned.remove(&id).is_some() {
                commands.entity(id).despawn_recursive();
            }
            self.despawn_requests.retain(|request| request.entity != id);
        }
    }

    fn drain(&mut self, predicate: impl Fn(&mut DespawnRequest) -> bool) -> Vec<Entity> {
        self.despawn_requests
            .drain_filter(predicate)
            .map(|event| event.entity)
            .collect()
    }

    fn drain_old(&mut self, frame: usize) -> Vec<Entity> {
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

    pub fn despawn_on_phase_change(&mut self, commands: &mut Commands) {
        let ids = self.drain(|event| matches!(event.time, DespawnTime::StateChange));
        self.despawn(commands, ids);
    }

    pub fn despawn_for_move(&mut self, commands: &mut Commands, move_id: MoveId) {
        let ids = self
            .spawned
            .iter()
            .filter_map(|(entity, id)| {
                if *id == move_id {
                    Some(entity.to_owned())
                } else {
                    None
                }
            })
            .collect();

        self.despawn(commands, ids);
    }

    pub fn add_to_queue(&mut self, id: MoveId, object: SpawnDescriptor) {
        self.queue.push((id, object));
    }
}

pub fn spawn_new(
    mut commands: Commands,
    clock: Res<Clock>,
    colors: Res<Colors>,
    mut query: Query<(&mut Spawner, Entity, &LRDirection, &Player, &Transform)>,
) {
    for (mut spawner, parent, facing, player, transform) in query.iter_mut() {
        for (move_id, spawn_descriptor) in spawner.queue.drain(..).collect::<Vec<_>>().into_iter() {
            spawner.spawn_attack(
                move_id,
                spawn_descriptor,
                &mut commands,
                &colors,
                clock.frame,
                parent,
                facing,
                *player,
                transform.translation,
            );
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

pub fn despawn_everything(mut commands: Commands, mut spawners: Query<&mut Spawner>) {
    for mut spawner in spawners.iter_mut() {
        let ids = spawner
            .spawned
            .drain()
            .map(|(entity, _move_id)| entity)
            .collect();
        spawner.despawn(&mut commands, ids);
    }
}
