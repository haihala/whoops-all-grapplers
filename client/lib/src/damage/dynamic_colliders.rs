use bevy::{prelude::*, utils::HashMap};
use characters::Hurtbox;
use wag_core::{Area, Joint, JointCollider, Joints, Owner, Player, Players};

pub(super) fn create_colliders(
    mut commands: Commands,
    players: Query<(Entity, &Player, &Joints)>,
    joint_tfs: Query<&GlobalTransform>,
    existing_colliders: Query<(&JointCollider, &Owner)>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let mut all_ready = true;

    for (entity, player, joints) in &players {
        // Can't create colliders before nodes have been linked
        if joints.nodes.is_empty() {
            continue;
        }

        for collider in &joints.colliders {
            if existing_colliders
                .iter()
                .any(|(jc, owner)| jc == collider && **owner == *player)
            {
                // This one is already spawned
                continue;
            }

            let Some(bounding_box) =
                collider_joints_bounding_box(collider, &joints.nodes, &joint_tfs)
            else {
                // Nodes not yet linked and can't find a neat bounding box
                continue;
            };

            all_ready = false;

            let hitbox = Hurtbox(bounding_box);
            let name = format!("{:?}", collider);

            commands
                .spawn((
                    SpatialBundle::default(),
                    hitbox,
                    collider.to_owned(),
                    Name::new(name),
                    Owner(*player),
                ))
                .set_parent(entity);
        }
    }

    if all_ready {
        *done = true;
    }
}

fn collider_joints_bounding_box(
    collider: &JointCollider,
    owned_joints: &HashMap<Joint, Entity>,
    tfs: &Query<&GlobalTransform>,
) -> Option<Area> {
    let points: Vec<Vec3> = collider
        .joints
        .iter()
        .filter_map(|joint| {
            tfs.get(*owned_joints.get(joint).unwrap())
                .map(|tf| tf.translation())
                .ok()
        })
        .collect();

    if points.is_empty() {
        None
    } else {
        Some(bounding_box(points, collider.padding))
    }
}

fn bounding_box(points: Vec<Vec3>, padding: f32) -> Area {
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    for point in points {
        let x = point.x;
        let y = point.y;

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    Area::from_sides(
        max_y + padding,
        min_y - padding,
        min_x - padding,
        max_x + padding,
    )
}

use strum::IntoEnumIterator;

pub(super) fn update_colliders(
    mut colliders: Query<(&mut Hurtbox, &JointCollider, &Owner)>,
    joints: Query<(&Joints, &Transform)>,
    joint_tfs: Query<&GlobalTransform>,
    players: Res<Players>,
) {
    for player in Player::iter() {
        let player_entity = players.get(player);
        let (player_joints, player_tf) = joints.get(player_entity).unwrap();
        for collider in &player_joints.colliders {
            if let Some(mut hurtbox) = colliders
                .iter_mut()
                .find(|(_, jc, owner)| *jc == collider && ***owner == player)
                .map(|(hb, _, _)| hb)
            {
                **hurtbox =
                    collider_joints_bounding_box(collider, &player_joints.nodes, &joint_tfs)
                        .unwrap()
                        .with_offset(-player_tf.translation.truncate());
            }
        }
    }
}
