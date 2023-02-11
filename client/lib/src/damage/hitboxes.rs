use bevy::prelude::*;

use characters::{Action, Attack, HitTracker, Hitbox, Lifetime};
use player_state::PlayerState;
use wag_core::{Area, Clock, Facing, Joints, Owner, Player};

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
        attack: Attack,
        frame: usize,
        parent: Entity,
        facing: &Facing,
        player: Player,
        parent_position: Vec3,
    ) {
        let offset = facing.mirror_vec3(attack.to_hit.hitbox.center().extend(0.0));
        let absolute_position = parent_position + offset;
        let transform = Transform::from_translation(if attack.to_hit.projectile.is_none() {
            offset
        } else {
            absolute_position
        });

        let mut builder = commands.spawn((
            SpatialBundle {
                transform,
                global_transform: Transform::from_translation(absolute_position).into(),
                ..default()
            },
            HitTracker::new(attack.to_hit.hits),
            Owner(player),
            Hitbox(Area::from_center_size(
                Vec2::ZERO, // Position is set into the object directly
                attack.to_hit.hitbox.size(),
            )),
            attack.clone(),
        ));

        if let Some(velocity) = attack.to_hit.velocity {
            builder.insert(ConstantVelocity::new(
                facing.mirror_vec3(velocity.extend(0.0)),
            ));
        }

        if let Some(model) = attack.to_hit.projectile.map(|p| p.model) {
            builder.with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: models[&model].clone(),
                    ..default()
                });
            });
        }

        let new_hitbox = builder.id();
        if attack.to_hit.projectile.is_none() {
            commands.entity(parent).push_children(&[new_hitbox]);
        }
        let mut lifetime = attack.to_hit.lifetime;
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
    tfs: Query<&GlobalTransform>,
    mut query: Query<(
        &mut HitboxSpawner,
        &mut PlayerState,
        &Joints,
        Entity,
        &Facing,
        &Player,
    )>,
) {
    for (mut spawner, mut state, joints, parent, facing, player) in &mut query {
        for attack in state
            .drain_matching_actions(|action| {
                if let Action::Attack(attack) = action {
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
