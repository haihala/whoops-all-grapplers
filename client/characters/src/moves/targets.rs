use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use wag_core::Area;

#[derive(Debug, Clone, Copy, Default, Component, DerefMut, Deref, Inspectable)]
pub struct Hurtbox(pub Area);
