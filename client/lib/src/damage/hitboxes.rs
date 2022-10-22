use bevy::prelude::*;

use characters::{Action, HitTracker, Hitbox, Lifetime, OnHitEffect, ToHit};
use core::{Area, Facing, Owner, Player};
use player_state::PlayerState;
use time::Clock;

use crate::{assets::Models, physics::ConstantVelocity};

#[derive(Debug)]
struct DespawnRequest {
    entity: Entity,
    lifetime: Lifetime,
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
        models: &Models,
        to_hit: ToHit,
        on_hit: OnHitEffect,
        frame: usize,
        parent: Entity,
        facing: &Facing,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.mirror_vec3(to_hit.hitbox.center().extend(0.0));
        let absolute_position = parent_position + offset;
        let transform = Transform::from_translation(if to_hit.projectile.is_none() {
            offset
        } else {
            absolute_position
        });

        let mut builder = commands.spawn_bundle(SpatialBundle {
            transform,
            global_transform: Transform::from_translation(absolute_position).into(),
            ..default()
        });

        // Housekeeping
        let new_hitbox = builder.id();

        // Components used when collision happens
        builder
            .insert(on_hit)
            .insert(HitTracker::new(to_hit.hits))
            .insert(Owner(player))
            .insert(Hitbox(Area::from_center_size(
                Vec2::ZERO, // Position is set into the object directly
                to_hit.hitbox.size(),
            )))
            .insert(to_hit.block_type);

        if let Some(velocity) = to_hit.velocity {
            builder.insert(ConstantVelocity::new(
                facing.mirror_vec3(velocity.extend(0.0)),
            ));
        }

        if let Some(model) = to_hit.projectile.map(|p| p.model) {
            builder.with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: models[&model].clone(),
                    ..default()
                });
            });
        }

        if to_hit.projectile.is_none() {
            commands.entity(parent).push_children(&[new_hitbox]);
        }
        let mut lifetime = to_hit.lifetime;
        lifetime.frames = lifetime.frames.map(|lifetime| lifetime + frame);

        self.despawn_requests.push(DespawnRequest {
            entity: new_hitbox,
            lifetime,
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
        self.despawn_matching(commands, |event| event.lifetime.despawn_on_hit);
    }

    pub fn despawn_on_landing(&mut self, commands: &mut Commands) {
        self.despawn_matching(commands, |event| event.lifetime.despawn_on_landing);
    }
}

pub(super) fn spawn_new(
    mut commands: Commands,
    clock: Res<Clock>,
    models: Res<Models>,
    mut query: Query<(
        &mut HitboxSpawner,
        &mut PlayerState,
        Entity,
        &Facing,
        &Player,
        &Transform,
    )>,
) {
    for (mut spawner, mut state, parent, facing, player, transform) in &mut query {
        for (to_hit, on_hit) in state
            .drain_matching_actions(|action| {
                if let Action::Attack(to_hit, on_hit) = action {
                    Some((*to_hit, *on_hit))
                } else {
                    None
                }
            })
            .into_iter()
        {
            spawner.spawn_attack(
                &mut commands,
                &models,
                to_hit,
                on_hit,
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
    for mut spawner in &mut spawners {
        spawner.despawn_matching(&mut commands, |event| {
            if let Some(last_frame_alive) = event.lifetime.frames {
                last_frame_alive <= clock.frame
            } else {
                false
            }
        });
    }
}

pub(super) fn despawn_everything(mut commands: Commands, mut spawners: Query<&mut HitboxSpawner>) {
    for mut spawner in &mut spawners {
        spawner.despawn_matching(&mut commands, |_| true);
    }
}
