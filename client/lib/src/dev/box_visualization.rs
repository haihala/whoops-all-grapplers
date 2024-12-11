use bevy::prelude::*;
use characters::{Hitbox, Hurtboxes};
use foundation::{
    Area, Facing, GENERIC_AREA_VISUALIZATION_COLOR, HITBOX_VISUALIZATION_COLOR,
    HURTBOX_VISUALIZATION_COLOR, PUSHBOX_VISUALIZATION_COLOR,
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
    hurtboxes: Query<(&Transform, &Facing, &Hurtboxes)>,
) {
    for (tf, facing, boxes) in &hurtboxes {
        for hurtbox in boxes.as_vec() {
            gizmos.rect(
                tf.translation + facing.mirror_vec2(hurtbox.center()).extend(0.0),
                Quat::default(),
                hurtbox.size(),
                HURTBOX_VISUALIZATION_COLOR,
            );
        }
    }
}

pub(super) fn visualize_pushboxes(
    mut gizmos: Gizmos,
    pushboxes: Query<(&Transform, &Facing, &Pushbox)>,
) {
    for (tf, facing, pushbox) in &pushboxes {
        gizmos.rect(
            tf.translation + facing.mirror_vec2(pushbox.center()).extend(0.0),
            Quat::default(),
            pushbox.size(),
            PUSHBOX_VISUALIZATION_COLOR,
        )
    }
}

pub(super) fn visualize_generic_areas(mut gizmos: Gizmos, areas: Query<(&Transform, &Area)>) {
    for (tf, area) in &areas {
        gizmos.rect(
            tf.translation,
            Quat::default(),
            area.size(),
            GENERIC_AREA_VISUALIZATION_COLOR,
        )
    }
}
