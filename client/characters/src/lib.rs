#![feature(trivial_bounds)]

mod characters;
mod items;
mod moves;
mod properties;

pub use self::characters::{dummy, Character};
pub use items::{Inventory, Item, ItemCategory};
pub use moves::{
    ActionEvent, Attack, AttackHeight, BlockType, Hitbox, Hurtbox, Lifetime, Move, MoveHistory,
    Movement, Situation, ToHit,
};
pub use properties::{
    BarRenderInstructions, ChargeProperty, Properties, Property, PropertyType, SpecialProperty,
};
