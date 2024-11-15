#![feature(trivial_bounds)]

mod action_tracker;
mod actions;
mod builders;
mod characters;
mod hit_data;
mod items;
mod resources;
mod situation;

pub(crate) use actions::*;
pub(crate) use builders::*;

pub use action_tracker::ActionTracker;
pub use actions::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, Attack, AttackHeight, BlockType,
    CharacterStateBoxes, FlashRequest, Hitbox, Hurtboxes, Lifetime, Movement, ToHit,
};
pub use characters::{samurai, Character};
pub use hit_data::{HitEffect, HitInfo};
pub use items::{ConsumableType, Inventory, Item, ItemCategory};
pub use resources::{
    ChargeProperty, CounterVisual, RenderInstructions, ResourceBarVisual, ResourceType,
    SpecialProperty, WAGResource, WAGResources,
};
pub use situation::Situation;
