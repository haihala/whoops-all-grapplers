use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::Player;

#[derive(Clone, Copy, Default)]
pub struct Hurtbox {
    pub offset: Vec3,
}

impl Default for AttackHeight {
    fn default() -> Self {
        AttackHeight::Mid
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

#[derive(Clone, Copy, Debug, Inspectable, PartialEq)]
pub enum AttackHeight {
    Low,
    Mid,
    High,
}
#[derive(Debug, Inspectable, Clone, Copy, PartialEq)]
pub struct Hit {
    pub damage: i32,
    pub hit_stun: usize,
    pub block_stun: usize,
    pub hit_knockback: Vec3,
    pub block_knockback: Vec3,
    pub fixed_height: Option<AttackHeight>,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            damage: 10,
            hit_stun: 30,
            block_stun: 15,
            hit_knockback: Vec3::new(2.0, 2.0, 0.0),
            block_knockback: Vec3::new(1.0, 0.0, 0.0),
            fixed_height: None,
        }
    }
}
