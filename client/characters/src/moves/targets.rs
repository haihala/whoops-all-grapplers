use bevy::prelude::*;
use wag_core::Area;

#[derive(Debug, Clone, Copy, Default, Component, DerefMut, Deref, Reflect)]
pub struct Hurtbox(pub Area);
