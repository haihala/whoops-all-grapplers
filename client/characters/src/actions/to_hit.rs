use bevy::prelude::*;

use wag_core::{Area, Model};

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct ToHit {
    pub block_type: BlockType,
    pub hitbox: Hitbox,
    pub lifetime: Lifetime,
    pub velocity: Vec2,
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

#[derive(Clone, Copy, Debug, Reflect, Eq, PartialEq, Default)]
pub enum AttackHeight {
    Low,
    #[default]
    Mid,
    High,
}

#[derive(Clone, Copy, Debug, Reflect, Eq, PartialEq, Component)]
pub enum BlockType {
    Strike(AttackHeight),
    Grab,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Strike(AttackHeight::Mid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Projectile {
    pub model: Model,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct CharacterStateBoxes {
    pub head: Area,
    pub chest: Area,
    pub legs: Area,
    pub pushbox: Area,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct CharacterBoxes {
    pub standing: CharacterStateBoxes,
    pub crouching: CharacterStateBoxes,
    pub airborne: CharacterStateBoxes,
}

#[derive(Debug, Clone, Default, Component, Reflect)]
pub struct Hurtboxes {
    pub head: Area,
    pub chest: Area,
    pub legs: Area,
    pub extra: Vec<(Area, usize)>,
}
impl From<CharacterStateBoxes> for Hurtboxes {
    fn from(value: CharacterStateBoxes) -> Self {
        Self {
            head: value.head,
            chest: value.chest,
            legs: value.legs,
            extra: vec![],
        }
    }
}

impl Hurtboxes {
    pub fn as_vec(&self) -> Vec<Area> {
        vec![self.head, self.chest, self.legs]
            .into_iter()
            .chain(self.extra.clone().into_iter().map(|(a, _)| a))
            .collect()
    }

    pub fn expire(&mut self, frame: usize) {
        self.extra.retain(|(_, end)| frame <= *end);
    }
}

#[derive(Default, Clone, Copy, Deref, DerefMut, Debug, Component, Reflect, PartialEq)]
pub struct Hitbox(pub Area);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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
    pub(crate) fn until_owner_hit() -> Self {
        Self {
            frames: None,
            ..default()
        }
    }
    pub(crate) fn frames(frames: usize) -> Self {
        Self {
            frames: Some(frames),
            ..default()
        }
    }
}
