use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Follow {
    pub target: Entity,
    pub offset: Vec3,
}

pub(super) fn update_followers(
    mut followers: Query<(&Follow, &mut Transform)>,
    targets: Query<&GlobalTransform>,
) {
    for (follow, mut tf) in &mut followers {
        // This has 1 frame delay due to how transforms get updated
        tf.translation = targets.get(follow.target).unwrap().translation() + follow.offset;
    }
}
