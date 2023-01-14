use bevy::prelude::*;

use wag_core::{Area, Model};

#[derive(Debug, Clone, Copy, PartialEq, Reflect, FromReflect)]
pub struct ToHit {
    pub block_type: BlockType,
    pub hitbox: Hitbox,
    pub lifetime: Lifetime,
    pub velocity: Option<Vec2>,
    pub projectile: Option<Projectile>,
    pub hits: usize,
}

impl Default for ToHit {
    fn default() -> Self {
        Self {
            block_type: Default::default(),
            hitbox: Hitbox(Area::new(1.0, 1.2, 0.2, 0.2)),
            lifetime: Lifetime::default(),
            velocity: Default::default(),
            projectile: Default::default(),
            hits: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Reflect, FromReflect, Eq, PartialEq, Default)]
pub enum AttackHeight {
    Low,
    #[default]
    Mid,
    High,
}

#[derive(Clone, Copy, Debug, Reflect, FromReflect, Eq, PartialEq, Default, Component)]
pub enum BlockType {
    Constant(AttackHeight),
    Grab,
    #[default]
    Dynamic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub struct Projectile {
    pub model: Model,
}

#[derive(
    Default, Clone, Copy, Deref, DerefMut, Debug, Component, Reflect, FromReflect, PartialEq,
)]
pub struct Hitbox(pub Area);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub struct Lifetime {
    pub despawn_on_hit: bool,
    pub despawn_on_landing: bool,
    pub frames: Option<usize>,
}

impl Default for Lifetime {
    fn default() -> Self {
        Self {
            despawn_on_hit: true,
            despawn_on_landing: true,
            frames: Some(1),
        }
    }
}

impl Lifetime {
    pub(crate) fn eternal() -> Self {
        Self {
            despawn_on_hit: false,
            despawn_on_landing: false,
            frames: None,
        }
    }

    pub(crate) fn until_owner_hit() -> Self {
        Self {
            despawn_on_hit: true,
            despawn_on_landing: false,
            frames: None,
        }
    }

    pub(crate) fn frames(frames: usize) -> Self {
        Self {
            frames: Some(frames),
            ..Default::default()
        }
    }
}
