use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use wag_core::{Animation, Area, Model};

#[derive(Default, Clone, Copy, Deref, DerefMut, Debug, Component, Inspectable, PartialEq)]
pub struct Hitbox(pub Area);

#[derive(Clone, Copy, Debug, Inspectable, Eq, PartialEq, Default)]
pub enum AttackHeight {
    Low,
    #[default]
    Mid,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
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

#[derive(Debug, Clone, Copy, PartialEq, Inspectable, Component)]
pub struct OnHitEffect {
    pub damage: usize,
    pub stun: usize,
    pub knockback: Vec2,
    pub pushback: Vec2,
    pub launches: bool,
    pub forced_animation: Option<Animation>,
    pub side_switch: bool,
}

impl Default for OnHitEffect {
    fn default() -> Self {
        Self {
            damage: 1,
            stun: 15,
            knockback: Vec2::X * 2.0,
            pushback: Vec2::ZERO,
            launches: false,
            forced_animation: None,
            side_switch: false,
        }
    }
}

const FRAMES_BETWEEN_HITS: usize = 10;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Inspectable, Component)]
pub struct HitTracker {
    pub hits: usize,
    pub last_hit_frame: Option<usize>,
    pub hit_intangible: bool,
}
impl HitTracker {
    pub fn new(hits: usize) -> Self {
        Self { hits, ..default() }
    }
    pub fn active(&self, current_frame: usize) -> bool {
        self.last_hit_frame
            .map(|frame| frame + FRAMES_BETWEEN_HITS <= current_frame)
            .unwrap_or(true)
    }
    pub fn register_hit(&mut self, current_frame: usize) {
        self.hits -= 1;
        self.last_hit_frame = Some(current_frame);
    }
}
impl Default for HitTracker {
    fn default() -> Self {
        Self {
            hits: 1,
            last_hit_frame: None,
            hit_intangible: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Inspectable, Eq, PartialEq, Default, Component)]
pub enum BlockType {
    Constant(AttackHeight),
    Grab,
    #[default]
    Dynamic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Projectile {
    pub model: Model,
}
