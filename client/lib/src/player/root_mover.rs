use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct RootMover;

pub fn update_root_transform(
    mut tfs: Query<&mut Transform>,
    root_movers: Query<(Entity, &Parent), With<RootMover>>,
    parents: Query<&Parent>,
) {
    for (model, parent) in root_movers.into_iter() {
        let mut root = parent;
        while let Ok(next_parent) = parents.get(**root) {
            root = next_parent;
        }

        let [mut model_tf, mut root_tf] = tfs.get_many_mut([model, **root]).unwrap();

        if model_tf.translation != Vec3::ZERO {
            root_tf.translation += model_tf.translation;
            model_tf.translation = Vec3::ZERO;
        }
    }
}
