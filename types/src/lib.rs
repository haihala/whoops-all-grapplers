mod direction;
pub use direction::*;

mod inputs;
pub use inputs::{GameButton, StickPosition};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::fmt::{Debug, Display};

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.

pub type MoveType = u32;

#[derive(EnumIter, Inspectable, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Player {
    One,
    Two,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Inspectable, Clone, Copy)]
pub struct Hit {
    pub damage: f32,
    pub hit_stun: usize,
    pub block_stun: usize,
    pub hit_knockback: Vec3,
    pub block_knockback: Vec3,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            damage: 10.0,
            hit_stun: 30,
            block_stun: 15,
            hit_knockback: Vec3::new(2.0, 2.0, 0.0),
            block_knockback: Vec3::new(1.0, 0.0, 0.0),
        }
    }
}
