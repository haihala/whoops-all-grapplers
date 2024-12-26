use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Follow {
    pub target: Entity,
    pub offset: Vec3,
}

pub(super) fn update_followers(
    followers: Query<(Entity, &Follow)>,
    mut tfs: Query<&mut Transform>,
) {
    for (entity, follow) in &followers {
        if let Ok([mut tf, target]) = tfs.get_many_mut([entity, follow.target]) {
            tf.translation = target.translation + follow.offset;
        } else if let Ok(mut tf) = tfs.get_mut(entity) {
            // Target doesn't exist, most likely because rollback
            // Move it to the moon
            tf.translation = Vec3::splat(10000.0);
        }
    }
}
