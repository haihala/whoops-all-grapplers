use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use types::Area;

#[derive(Debug, Clone, Copy, Default, Component, DerefMut, Deref, Inspectable)]
pub struct Hurtbox(pub Area);
