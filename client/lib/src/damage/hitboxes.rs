use bevy::prelude::*;

use characters::{Action, HitTracker, Hitbox, Lifetime, OnHitEffect, SpawnDescriptor};
use player_state::PlayerState;
use time::Clock;
use types::{Area, Facing, Owner, Player};

use crate::physics::ConstantVelocity;

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
pub struct HitboxSpawner {
    despawn_requests: Vec<DespawnRequest>,
}
impl HitboxSpawner {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn_attack(
        &mut self,
        commands: &mut Commands,
        descriptor: SpawnDescriptor,
        frame: usize,
        parent: Entity,
        facing: &Facing,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.mirror_vec(descriptor.hitbox.center().extend(0.0));
        let absolute_position = parent_position + offset;
        let transform = Transform::from_translation(if descriptor.attached_to_player {
            offset
        } else {
            absolute_position
        });

        let mut builder = commands.spawn_bundle(TransformBundle {
            local: transform,
            global: GlobalTransform {
                translation: absolute_position,
                ..default()
            },
        });

        // Housekeeping
        let new_hitbox = builder.id();

        // Components used when collision happens
        builder
            .insert(OnHitEffect {
                fixed_height: descriptor.fixed_height,
                damage: descriptor.damage,
                stun: descriptor.stun,
                knockback: descriptor.knockback,
                pushback: descriptor.pushback,
            })
            .insert(HitTracker::new(descriptor.hits))
            .insert(Owner(player))
            .insert(Hitbox(Area::from_center_size(
                Vec2::ZERO, // Position is set into the object directly
                descriptor.hitbox.size(),
            )))
            .insert(ConstantVelocity::new(facing.mirror_vec(descriptor.speed)));

        if descriptor.attached_to_player {
            commands.entity(parent).push_children(&[new_hitbox]);
        }
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

    fn despawn_matching(
        &mut self,
        commands: &mut Commands,
        predicate: impl Fn(&mut DespawnRequest) -> bool,
    ) {
        for id in self
            .despawn_requests
            .drain_filter(predicate)
            .map(|event| event.entity)
        {
            commands.entity(id).despawn_recursive();
        }
    }

    pub fn despawn(&mut self, commands: &mut Commands, entity: Entity) {
        self.despawn_matching(commands, |request| request.entity == entity);
    }

    pub fn despawn_on_hit(&mut self, commands: &mut Commands) {
        self.despawn_matching(commands, |event| {
            matches!(event.time, DespawnTime::OnHit)
            // Getting hit changes the state
                || matches!(event.time, DespawnTime::StateChange)
        });
    }

    pub fn despawn_on_phase_change(&mut self, commands: &mut Commands) {
        self.despawn_matching(commands, |event| {
            matches!(event.time, DespawnTime::StateChange)
        });
    }
}

pub(super) fn spawn_new(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        &mut HitboxSpawner,
        &mut PlayerState,
        Entity,
        &Facing,
        &Player,
        &Transform,
    )>,
) {
    for (mut spawner, mut state, parent, facing, player, transform) in query.iter_mut() {
        for spawn_descriptor in state
            .drain_matching_actions(|action| {
                if let Action::Hitbox(descriptor) = action {
                    Some(*descriptor)
                } else {
                    None
                }
            })
            .into_iter()
        {
            spawner.spawn_attack(
                &mut commands,
                spawn_descriptor,
                clock.frame,
                parent,
                facing,
                *player,
                transform.translation,
            );
        }
    }
}

pub(super) fn despawn_expired(
    mut commands: Commands,
    clock: Res<Clock>,
    mut spawners: Query<&mut HitboxSpawner>,
) {
    for mut spawner in spawners.iter_mut() {
        spawner.despawn_matching(&mut commands, |event| {
            if let DespawnTime::Frame(despawn_frame) = event.time {
                despawn_frame <= clock.frame
            } else {
                false
            }
        });
    }
}

pub(super) fn despawn_everything(mut commands: Commands, mut spawners: Query<&mut HitboxSpawner>) {
    for mut spawner in spawners.iter_mut() {
        spawner.despawn_matching(&mut commands, |_| true);
    }
}
