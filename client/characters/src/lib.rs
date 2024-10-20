#![feature(trivial_bounds)]

mod action_tracker;
mod actions;
mod characters;
mod items;
mod resources;
mod situation;

pub(crate) use actions::*;

pub use action_tracker::ActionTracker;
pub use actions::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, Attack, AttackHeight, BlockType,
    CharacterStateBoxes, FlashRequest, Hitbox, Hurtboxes, Lifetime, Movement, ToHit,
};
pub use characters::{dummy, mizku, Character};
pub use items::{ConsumableType, Inventory, Item, ItemCategory};
pub use resources::{
    ChargeProperty, CounterVisual, RenderInstructions, ResourceBarVisual, ResourceType,
    SpecialProperty, WAGResource, WAGResources,
};
pub use situation::Situation;
