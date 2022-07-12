mod characters; // Where character specifics live
mod items; // Defines things for items
mod moves;
mod resources; // Defines things for moves

use items::{Item, ItemId};
use moves::{Branch, GrabDescription, Phase, Requirements};
use resources::Cost;

pub use self::characters::{dummy, Character};
pub use items::Inventory;
pub use moves::{
    AttackHeight, CancelLevel, Grabable, HitTracker, Hitbox, Hurtbox, Lifetime, Move, MoveAction,
    MoveId, MoveMobility, MoveSituation, OnHitEffect, PhaseKind, SpawnDescriptor,
};
pub use resources::Resources;
