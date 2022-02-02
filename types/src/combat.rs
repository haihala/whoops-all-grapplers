use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{MoveId, Player};

#[derive(Clone, Copy, Default, Component)]
pub struct Hurtbox {
    pub offset: Vec3,
}

#[derive(Clone, Copy, Component)]
pub struct Grabable {
    pub size: f32,
}

impl Default for Grabable {
    fn default() -> Self {
        Self { size: 0.5 }
    }
}

#[derive(Default, Clone, Copy, Debug, Inspectable, PartialEq)]
pub struct Hitbox {
    pub offset: Vec3,
    pub size: Vec2,
}
impl Hitbox {
    pub fn new(offset: Vec2, size: Vec2) -> Self {
        Self {
            offset: offset.extend(0.0),
            size,
        }
    }
}

#[derive(Clone, Copy, Debug, Inspectable, PartialEq)]
pub enum AttackHeight {
    Low,
    Mid,
    High,
}
impl Default for AttackHeight {
    fn default() -> Self {
        AttackHeight::Mid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum Lifetime {
    Phase,
    UntilHit,
    Frames(usize),
    Forever,
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Phase
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Inspectable)]
pub struct HitProperty<PropType: Clone + Copy + PartialEq + Default + Inspectable> {
    pub on_hit: PropType,
    pub on_block: PropType,
}
impl<T: Clone + Copy + PartialEq + Default + Inspectable> HitProperty<T> {
    pub fn new(on_hit: T, on_block: T) -> HitProperty<T> {
        HitProperty { on_hit, on_block }
    }

    pub fn get(&self, blocked: bool) -> T {
        if blocked {
            self.on_block
        } else {
            self.on_hit
        }
    }
}
impl<T: Clone + Copy + PartialEq + Default + Inspectable> From<(T, T)> for HitProperty<T> {
    fn from(input: (T, T)) -> Self {
        Self {
            on_hit: input.0,
            on_block: input.1,
        }
    }
}
impl<T: Clone + Copy + PartialEq + Default + Inspectable> From<T> for HitProperty<T> {
    fn from(input: T) -> Self {
        Self {
            on_hit: input,
            on_block: T::default(),
        }
    }
}

pub type Damage = HitProperty<i32>;
pub type Stun = HitProperty<usize>;
pub type Knockback = HitProperty<Vec3>;
pub type Pushback = HitProperty<Vec3>;

#[derive(Debug, Clone, Copy, PartialEq, Inspectable, Component)]
pub struct OnHitEffect {
    pub owner: Player,
    pub id: MoveId,

    pub fixed_height: Option<AttackHeight>,
    pub damage: Option<Damage>,
    pub stun: Option<Stun>,
    pub knockback: Option<Knockback>,
    pub pushback: Option<Pushback>,
}

impl Default for OnHitEffect {
    fn default() -> Self {
        Self {
            owner: Player::One,
            id: Default::default(),
            fixed_height: Default::default(),
            damage: Default::default(),
            stun: Default::default(),
            knockback: Default::default(),
            pushback: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Inspectable)]
pub struct SpawnDescriptor {
    pub damage: Option<Damage>,
    pub stun: Option<Stun>,
    pub knockback: Option<Knockback>,
    pub pushback: Option<Pushback>,

    pub speed: Option<Vec3>,
    pub hitbox: Hitbox,
    pub fixed_height: Option<AttackHeight>,
    pub lifetime: Lifetime,
    pub attached_to_player: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Inspectable)]
pub struct GrabDescription {
    pub damage: i32,
    pub impulse: Vec3,

    pub range: f32,
    pub offset: Vec2,
}
