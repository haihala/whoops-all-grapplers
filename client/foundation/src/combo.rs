use bevy::prelude::Component;

#[derive(Debug, Component, Clone, Copy)]
pub struct Combo {
    pub hits: usize,
    pub old_health: i32,
}
