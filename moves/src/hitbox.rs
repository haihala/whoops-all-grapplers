use bevy::prelude::*;

use bevy::utils::HashMap;
use types::{Hit, MoveType, Player};

use crate::ryan::*;

#[derive(Default, Clone, Copy)]
pub struct Hitbox {
    offset: Vec3,
    pub size: Vec2,
    pub hit: Hit,
    pub owner: Option<Player>,
}
impl Hitbox {
    pub fn get_offset(&self, flipped: bool) -> Vec3 {
        if flipped {
            Vec3::new(-self.offset.x, self.offset.y, self.offset.z)
        } else {
            self.offset
        }
    }

    pub fn new(offset: Vec2, size: Vec2, hit: Hit) -> Self {
        Self {
            offset: offset.extend(0.0),
            size,
            hit,
            owner: None,
        }
    }
}

pub fn ryan_hitboxes() -> HashMap<MoveType, Hitbox> {
    vec![
        (
            HADOUKEN,
            Hitbox::new(
                Vec2::new(0.5, 0.5),
                Vec2::new(0.3, 0.2),
                Hit {
                    ..Default::default()
                },
            ),
        ),
        (
            PUNCH,
            Hitbox::new(
                Vec2::new(1.0, 0.5),
                Vec2::new(0.2, 0.3),
                Hit {
                    hit_knockback: Vec3::new(2.0, 2.0, 0.0),
                    ..Default::default()
                },
            ),
        ),
        (
            COMMAND_PUNCH,
            Hitbox::new(
                Vec2::new(0.5, 0.5),
                Vec2::new(1.0, 1.0),
                Hit {
                    ..Default::default()
                },
            ),
        ),
    ]
    .into_iter()
    .collect()
}
