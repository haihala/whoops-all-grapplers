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
    Action, ActionEvent, Attack, AttackHeight, BlockType, Hitbox, Hurtbox, Lifetime, Movement,
    ToHit,
};
pub use characters::{dummy, Character};
pub use items::{Inventory, Item, ItemCategory};
pub use resources::{
    ChargeProperty, ResourceBarVisual, ResourceType, SpecialProperty, WAGResource, WAGResources,
};
pub use situation::Situation;
