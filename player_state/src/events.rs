use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use types::{Hitbox, MoveId};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum StateEvent {
    Jump(Vec3),
    Hitbox {
        hitbox: Hitbox,
        move_id: MoveId,
        ttl: usize,
    },
    Projectile {
        hitbox: Hitbox,
        speed: f32,
        move_id: MoveId,
        ttl: Option<usize>,
    },
    Recovery,
    Null,
}

impl Default for StateEvent {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        StateEvent::Null
    }
}
