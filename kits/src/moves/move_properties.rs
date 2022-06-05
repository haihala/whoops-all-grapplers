use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::MoveId;

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub enum MoveAction {
    Move(MoveId),
    Phase(Phase),
}
impl Default for MoveAction {
    fn default() -> Self {
        panic!("This should never be called, exists to satisfy Inspectable");
    }
}
impl From<MoveId> for MoveAction {
    fn from(id: MoveId) -> Self {
        MoveAction::Move(id)
    }
}
impl From<Phase> for MoveAction {
    fn from(phase_data: Phase) -> Self {
        MoveAction::Phase(phase_data)
    }
}
impl MoveAction {
    pub fn get_duration(&self) -> Option<usize> {
        match self {
            MoveAction::Move(_) => None,
            MoveAction::Phase(phase_data) => Some(phase_data.duration),
        }
    }

    pub fn is_cancellable(&self) -> bool {
        match self {
            MoveAction::Move(_) => false,
            MoveAction::Phase(phase_data) => phase_data.cancellable,
        }
    }

    pub fn get_mobility(&self) -> Option<MoveMobility> {
        match self {
            MoveAction::Move(_) => None,
            MoveAction::Phase(phase_data) => phase_data.mobility,
        }
    }
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Phase {
    pub kind: PhaseKind,
    pub duration: usize,
    pub cancellable: bool,
    pub mobility: Option<MoveMobility>,
}

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub enum PhaseKind {
    Animation,
    Grab(GrabDescription),
    Attack(SpawnDescriptor),
}
impl Default for PhaseKind {
    fn default() -> Self {
        PhaseKind::Animation
    }
}

#[derive(Debug, Inspectable, Copy, Clone, PartialEq)]
pub enum MoveMobility {
    Impulse(Vec3),
    Perpetual(Vec3),
}

#[derive(Clone, Copy, Default, Component)]
pub struct Hurtbox {
    pub offset: Vec3,
}

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

#[derive(Clone, Copy, Debug, Inspectable, Eq, PartialEq)]
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
    pub id: MoveId, // Needed so we can despawn the hitbox when a hit is registered

    pub fixed_height: Option<AttackHeight>,
    pub damage: Option<Damage>,
    pub stun: Option<Stun>,
    pub knockback: Option<Knockback>,
    pub pushback: Option<Pushback>,
}

impl Default for OnHitEffect {
    fn default() -> Self {
        Self {
            id: default(),
            fixed_height: default(),
            damage: default(),
            stun: default(),
            knockback: default(),
            pushback: default(),
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
