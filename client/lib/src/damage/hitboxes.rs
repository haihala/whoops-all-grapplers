use bevy::prelude::*;

use bevy_ggrs::AddRollbackCommandExtension;
use characters::{Attack, Hitbox, Lifetime};
use player_state::PlayerState;
use wag_core::{Area, Clock, Facing, MatchState, Owner, Player};

use crate::{
    assets::Models,
    entity_management::DespawnMarker,
    event_spreading::SpawnHitbox,
    movement::{ConstantVelocity, Follow},
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

#[derive(Debug, Component)]
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
        facing: &Facing,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.mirror_vec2(attack.to_hit.hitbox.center());
        let absolute_position = parent_position + offset.extend(0.0);
        let transform = Transform::from_translation(absolute_position);

        let hitbox = Hitbox(Area::from_center_size(
            Vec2::ZERO, // Position is set into the object directly
            attack.to_hit.hitbox.size(),
        ));

        let mut builder = commands.spawn((
            SpatialBundle {
                transform,
                global_transform: Transform::from_translation(absolute_position).into(),
                ..default()
            },
            HitTracker::new(attack.to_hit.hits),
            Owner(player),
            hitbox,
            attack.clone(),
            StateScoped(MatchState::Combat),
        ));

        if attack.to_hit.velocity != Vec2::ZERO {
            builder.insert(ConstantVelocity::new(
                facing.mirror_vec3(attack.to_hit.velocity.extend(0.0)),
            ));
        }

        if let Some(projectile) = attack.to_hit.projectile {
            builder.with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: models[&projectile.model].clone(),
                    transform: Transform {
                        scale: Vec3::new(facing.to_signum(), 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                });
            });
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
        &Facing,
        &Player,
        &PlayerState,
    )>,
) {
    let (mut spawner, tf, parent, facing, player, state) = query.get_mut(trigger.entity()).unwrap();
    let SpawnHitbox(to_hit, on_hit) = trigger.event();
    let attack = Attack {
        to_hit: *to_hit,
        on_hit: *on_hit,
        action_id: state.get_action_tracker().unwrap().action_id,
    };

    spawner.spawn_attack(
        &mut commands,
        &models,
        attack,
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
