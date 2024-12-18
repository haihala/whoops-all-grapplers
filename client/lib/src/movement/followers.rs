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
        let [mut tf, target] = tfs.get_many_mut([entity, follow.target]).unwrap();

        tf.translation = target.translation + follow.offset;
    }
}
