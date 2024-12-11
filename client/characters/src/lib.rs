#![feature(trivial_bounds)]

mod actions;
mod bridging;
mod builders;
mod characters;
mod items;
mod resources;

pub(crate) use actions::*;
pub(crate) use builders::*;

pub use actions::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, Attack, AttackHeight, BlockType,
    CharacterStateBoxes, FlashRequest, Hitbox, Hurtboxes, Lifetime, Movement, ToHit,
};
pub use bridging::{ActionTracker, HitEffect, HitInfo, Situation};
pub use characters::{samurai, Character};
pub use items::{ConsumableType, Inventory, Item, ItemCategory};
pub use resources::{
    ChargeProperty, CounterVisual, Gauge, GaugeType, Gauges, RenderInstructions, ResourceBarVisual,
    SpecialProperty,
};
