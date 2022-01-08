use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::Player;

#[derive(Clone, Copy)]
pub struct Hurtbox {
    pub offset: Vec3,
    pub size: Vec2,
}
impl Hurtbox {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            offset: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Inspectable, PartialEq)]
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

#[derive(Debug, Inspectable, Clone, Copy, PartialEq)]
pub struct Hit {
    pub damage: f32,
    pub hit_stun: usize,
    pub block_stun: usize,
    pub hit_knockback: Vec3,
    pub block_knockback: Vec3,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            damage: 10.0,
            hit_stun: 30,
            block_stun: 15,
            hit_knockback: Vec3::new(2.0, 2.0, 0.0),
            block_knockback: Vec3::new(1.0, 0.0, 0.0),
        }
    }
}
