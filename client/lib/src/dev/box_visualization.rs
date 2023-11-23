use bevy::prelude::*;
use characters::{Hitbox, Hurtbox};

use crate::{assets::Colors, physics::Pushbox};

pub(super) fn visualize_hitboxes(
    mut gizmos: Gizmos,
    colors: Res<Colors>,
    hitboxes: Query<(&GlobalTransform, &Hitbox)>,
) {
    for (gtf, hitbox) in &hitboxes {
        gizmos.rect(
            gtf.translation() + hitbox.center().extend(0.0),
            Quat::default(),
            hitbox.size(),
            colors.hitbox,
        )
    }
}
pub(super) fn visualize_hurtboxes(
    mut gizmos: Gizmos,
    colors: Res<Colors>,
    hurtboxes: Query<(&GlobalTransform, &Hurtbox)>,
) {
    for (gtf, hurtbox) in &hurtboxes {
        gizmos.rect(
            gtf.translation() + hurtbox.center().extend(0.0),
            Quat::default(),
            hurtbox.size(),
            colors.hurtbox,
        )
    }
}
pub(super) fn visualize_pushboxes(
    mut gizmos: Gizmos,
    colors: Res<Colors>,
    pushboxes: Query<(&Transform, &Pushbox)>,
) {
    for (tf, pushbox) in &pushboxes {
        gizmos.rect(
            tf.translation + (pushbox.center().extend(0.0)),
            Quat::default(),
            pushbox.size(),
            colors.pushbox,
        )
    }
}
