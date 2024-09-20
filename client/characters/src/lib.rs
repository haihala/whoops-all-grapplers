#![feature(trivial_bounds)]
#![feature(extract_if)]

mod action_tracker;
mod actions;
mod characters;
mod items;
mod resources;
mod situation;

pub(crate) use actions::*;

pub use action_tracker::ActionTracker;
pub use actions::{
    Action, ActionEvent, ActionEvents, ActionRequirement, AnimationRequest, Attack, AttackHeight,
    BlockType, FlashRequest, Hitbox, Hurtbox, Lifetime, Movement, ToHit,
};
pub use characters::{dummy, mizku, Character};
pub use items::{ConsumableType, Inventory, Item, ItemCategory};
pub use resources::{
    ChargeProperty, CounterVisual, RenderInstructions, ResourceBarVisual, ResourceType,
    SpecialProperty, WAGResource, WAGResources,
};
pub use situation::Situation;
