use std::f32::consts::PI;

use bevy::prelude::*;
use types::{LRDirection, Player, Players};

#[derive(Component, Deref, DerefMut)]
pub struct PlayerModel(pub Player);

const MODEL_X_OFFSET: f32 = 0.1;

pub fn model_flipper(
    directions: Query<&LRDirection>,
    mut models: Query<(&mut Transform, &PlayerModel)>,
    players: Res<Players>,
) {
    for (mut tf, pm) in models.iter_mut() {
        let facing = directions.get(players.get(**pm)).unwrap();
        let (rot, pos) = match facing {
            LRDirection::Left => (Quat::from_rotation_y(-PI / 2.0), Vec3::X * MODEL_X_OFFSET),
            LRDirection::Right => (Quat::from_rotation_y(PI / 2.0), -Vec3::X * MODEL_X_OFFSET),
        };

        tf.rotation = rot;
        tf.translation = pos;
    }
}
