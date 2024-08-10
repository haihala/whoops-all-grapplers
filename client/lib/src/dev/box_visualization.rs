use bevy::prelude::*;
use characters::{Hitbox, Hurtbox};
use wag_core::{
    HITBOX_VISUALIZATION_COLOR, HURTBOX_VISUALIZATION_COLOR, PUSHBOX_VISUALIZATION_COLOR,
};

use crate::movement::Pushbox;

pub(super) fn visualize_hitboxes(mut gizmos: Gizmos, hitboxes: Query<(&GlobalTransform, &Hitbox)>) {
    for (gtf, hitbox) in &hitboxes {
        gizmos.rect(
            gtf.translation() + hitbox.center().extend(0.0),
            Quat::default(),
            hitbox.size(),
            HITBOX_VISUALIZATION_COLOR,
        )
    }
}
pub(super) fn visualize_hurtboxes(
    mut gizmos: Gizmos,
    hurtboxes: Query<(&GlobalTransform, &Hurtbox)>,
) {
    for (gtf, hurtbox) in &hurtboxes {
        gizmos.rect(
            gtf.translation() + hurtbox.center().extend(0.0),
            Quat::default(),
            hurtbox.size(),
            HURTBOX_VISUALIZATION_COLOR,
        )
    }
}
pub(super) fn visualize_pushboxes(mut gizmos: Gizmos, pushboxes: Query<(&Transform, &Pushbox)>) {
    for (tf, pushbox) in &pushboxes {
        gizmos.rect(
            tf.translation + (pushbox.center().extend(0.0)),
            Quat::default(),
            pushbox.size(),
            PUSHBOX_VISUALIZATION_COLOR,
        )
    }
}
