use bevy::prelude::*;

use bevy_ggrs::AddRollbackCommandExtension;
use characters::{Attack, Hitbox, Lifetime};
use foundation::{Area, CharacterFacing, Clock, MatchState, Owner, Player};

use crate::{
    assets::Models,
    entity_management::DespawnMarker,
    event_spreading::SpawnHitbox,
    movement::{Follow, ObjectVelocity},
};

use super::HitTracker;

#[derive(Component, Clone, Copy)]
pub struct LifetimeFlags {
    on_landing: bool,
    on_hit: bool,
}
impl From<Lifetime> for LifetimeFlags {
    fn from(value: Lifetime) -> Self {
        Self {
            on_landing: value.despawn_on_landing,
            on_hit: value.despawn_on_hit,
        }
    }
}

#[derive(Default, Component, Clone, Copy)]
pub struct HitboxSpawner {
    mark_landers: bool,
    mark_hitters: bool,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct ProjectileMarker;

impl HitboxSpawner {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn_attack(
        &mut self,
        commands: &mut Commands,
        models: &Models,
        attack: Attack,
        frame: usize,
        parent: Entity,
        facing: &CharacterFacing,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.visual.mirror_vec2(attack.to_hit.hitbox.center());
        let absolute_position = parent_position + offset.extend(0.0);
        let transform = Transform::from_translation(absolute_position);

        let hitbox = Hitbox(Area::from_center_size(
            Vec2::ZERO, // Position is set into the object directly
            attack.to_hit.hitbox.size(),
        ));

        let mut builder = commands.spawn((
            transform,
            GlobalTransform::from_translation(absolute_position),
            HitTracker::new(attack.to_hit.hits),
            Owner(player),
            hitbox,
            attack.clone(),
            StateScoped(MatchState::Combat),
        ));

        if attack.to_hit.velocity != Vec2::ZERO || attack.to_hit.gravity != 0.0 {
            builder.insert(ObjectVelocity::new(
                facing
                    .visual
                    .mirror_vec3(attack.to_hit.velocity.extend(0.0)),
                attack.to_hit.gravity,
            ));
        }

        if let Some(model) = attack.to_hit.model {
            builder.insert(SceneRoot(models[&model].clone()));
        }

        if attack.to_hit.projectile {
            builder.insert(ProjectileMarker);
        } else {
            builder.insert(Follow {
                target: parent,
                offset: offset.extend(0.0),
            });
        }

        if let Some(lifetime) = attack.to_hit.lifetime.frames {
            builder.insert(DespawnMarker(lifetime + frame));
        }
        builder.insert(LifetimeFlags::from(attack.to_hit.lifetime));
        builder.add_rollback();
    }

    pub fn despawn_on_hit(&mut self) {
        self.mark_hitters = true;
    }

    pub fn despawn_on_landing(&mut self) {
        self.mark_landers = true;
    }
}

pub fn spawn_hitbox(
    trigger: Trigger<SpawnHitbox>,
    mut commands: Commands,
    clock: Res<Clock>,
    models: Res<Models>,
    mut query: Query<(
        &mut HitboxSpawner,
        &Transform,
        Entity,
        &CharacterFacing,
        &Player,
    )>,
) {
    let (mut spawner, tf, parent, facing, player) = query.get_mut(trigger.entity()).unwrap();
    let SpawnHitbox(attack) = trigger.event();

    spawner.spawn_attack(
        &mut commands,
        &models,
        attack.clone(),
        clock.frame,
        parent,
        facing,
        *player,
        tf.translation,
    );
}

pub fn handle_despawn_flags(
    mut commands: Commands,
    boxes: Query<(Entity, &Owner, &LifetimeFlags, &HitTracker)>,
    mut player_query: Query<(&mut HitboxSpawner, &Player)>,
) {
    for (mut spawner, player) in &mut player_query {
        for (hitbox, owner, flags, tracker) in &boxes {
            if *player != owner.0 {
                continue;
            }

            if (flags.on_hit && spawner.mark_hitters)
                || (flags.on_landing && spawner.mark_landers)
                || (tracker.hits == 0)
            {
                commands.entity(hitbox).insert(DespawnMarker(0));
            }
        }

        spawner.mark_hitters = false;
        spawner.mark_landers = false;
    }
}
