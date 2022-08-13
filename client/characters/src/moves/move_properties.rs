use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use types::{Area, Model};

#[derive(Debug, Clone, Copy, Default, Component, DerefMut, Deref, Inspectable)]
pub struct Hurtbox(pub Area);

#[derive(Clone, Component)]
pub struct Grabable {
    pub size: f32,
    pub queue: Vec<GrabDescription>,
}

impl Default for Grabable {
    fn default() -> Self {
        Self {
            size: 0.5,
            queue: vec![],
        }
    }
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Inspectable)]
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

#[derive(Debug, Clone, Copy, PartialEq, Inspectable, Component, Default)]
pub struct OnHitEffect {
    pub fixed_height: Option<AttackHeight>,
    pub damage: Damage,
    pub stun: Stun,
    pub knockback: Knockback,
    pub pushback: Pushback,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Inspectable, Component)]
pub struct HitTracker {
    pub hits: usize,
    pub last_hit_frame: Option<usize>,
}
impl HitTracker {
    pub fn new(hits: usize) -> Self {
        Self { hits, ..default() }
    }
}
impl Default for HitTracker {
    fn default() -> Self {
        Self {
            hits: 1,
            last_hit_frame: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Inspectable)]
pub struct SpawnDescriptor {
    pub damage: Damage,
    pub stun: Stun,
    pub hits: usize,
    pub knockback: Knockback,
    pub pushback: Pushback,
    pub model: Option<Model>,

    /// Hitbox is moved at this constant speed
    pub speed: Vec3,
    pub hitbox: Hitbox,
    pub fixed_height: Option<AttackHeight>,
    pub lifetime: Lifetime,
    pub attached_to_player: bool,
}

impl Default for SpawnDescriptor {
    fn default() -> Self {
        Self {
            damage: (10, 1).into(),
            stun: (15, 5).into(),
            speed: Vec3::ZERO,
            hits: 1,
            hitbox: Hitbox(Area::new(1.0, 1.2, 0.2, 0.2)),
            fixed_height: None,
            lifetime: Lifetime::default(),
            attached_to_player: true,
            knockback: (Vec3::X * 2.0, Vec3::X * 1.0).into(),
            pushback: (Vec3::X * 1.0, Vec3::X * 0.5).into(),
            model: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Inspectable)]
pub struct GrabDescription {
    pub damage: i32,
    pub impulse: Vec3,

    pub range: f32,
    pub offset: Vec2,
}

impl Default for GrabDescription {
    fn default() -> Self {
        Self {
            damage: 10,
            impulse: Vec3::new(2.0, 5.0, 0.0),
            range: 1.0,
            offset: Vec2::ZERO,
        }
    }
}
