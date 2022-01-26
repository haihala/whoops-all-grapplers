use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::Player;

#[derive(Clone, Copy, Default)]
pub struct Hurtbox {
    pub offset: Vec3,
}

#[derive(Clone, Copy)]
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

pub struct PlayerCollisionTrigger {
    pub owner: Player,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HitProperty<PropType: Clone + Copy + PartialEq + Default> {
    pub on_hit: PropType,
    pub on_block: PropType,
}
impl<T: Clone + Copy + PartialEq + Default> HitProperty<T> {
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
impl<T: Clone + Copy + PartialEq + Default> From<(T, T)> for HitProperty<T> {
    fn from(input: (T, T)) -> Self {
        Self {
            on_hit: input.0,
            on_block: input.1,
        }
    }
}
impl<T: Clone + Copy + PartialEq + Default> From<T> for HitProperty<T> {
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

#[derive(Debug, Clone, Copy, PartialEq, Default, Inspectable)]
pub struct AttackDescriptor {
    // TODO: These could be made inspectable, this is a temporary solution
    #[inspectable(ignore)]
    pub damage: Option<Damage>,
    #[inspectable(ignore)]
    pub stun: Option<Stun>,
    #[inspectable(ignore)]
    pub knockback: Option<Knockback>,
    #[inspectable(ignore)]
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
