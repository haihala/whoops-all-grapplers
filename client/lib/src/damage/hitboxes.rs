use bevy::prelude::*;

use characters::{ActionEvent, ActionEvents, Attack, Hitbox, Lifetime};
use wag_core::{Area, Clock, Facing, InCombat, Joints, Owner, Player};

use crate::{
    assets::Models,
    entity_management::DespawnMarker,
    movement::{ConstantVelocity, Follow},
};

use super::HitTracker;

#[derive(Component)]
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
            StateScoped(InCombat),
        ));

        if let Some(velocity) = attack.to_hit.velocity {
            builder.insert(ConstantVelocity::new(
                facing.mirror_vec3(velocity.extend(0.0)),
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

        if let Some(frames) = attack.to_hit.lifetime.frames {
            builder.insert(DespawnMarker(frames + frame));
        }
        builder.insert(LifetimeFlags::from(attack.to_hit.lifetime));
    }

    pub fn despawn_on_hit(&mut self) {
        self.mark_hitters = true;
    }

    pub fn despawn_on_landing(&mut self) {
        self.mark_landers = true;
    }
}

pub(super) fn spawn_new_hitboxes(
    mut commands: Commands,
    clock: Res<Clock>,
    models: Res<Models>,
    tfs: Query<&GlobalTransform>,
    mut query: Query<(
        &mut HitboxSpawner,
        &ActionEvents,
        &Joints,
        Entity,
        &Facing,
        &Player,
    )>,
) {
    for (mut spawner, events, joints, parent, facing, player) in &mut query {
        for attack in events
            .get_matching_events(|action| {
                if let ActionEvent::Attack(attack) = action {
                    Some(attack.to_owned())
                } else {
                    None
                }
            })
            .into_iter()
        {
            let root = if let Some(joint) = attack.to_hit.joint {
                // Attach to that joint if joint is presented
                *joints
                    .nodes
                    // Need to use the opposite joint if model is flipped
                    .get(&if facing.to_flipped() {
                        joint.flip()
                    } else {
                        joint
                    })
                    .unwrap()
            } else {
                parent
            };

            spawner.spawn_attack(
                &mut commands,
                &models,
                attack,
                clock.frame,
                root,
                facing,
                *player,
                tfs.get(root).unwrap().translation(),
            );
        }
    }
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
