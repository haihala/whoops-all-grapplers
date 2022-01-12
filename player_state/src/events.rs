use bevy_inspector_egui::Inspectable;

use types::{AbsoluteDirection, Hitbox, MoveId};

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum JumpDirection {
    Neutral,
    Diagonal(AbsoluteDirection),
    Null,
}
impl Default for JumpDirection {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        JumpDirection::Null
    }
}

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum StateEvent {
    Jump(JumpDirection),
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
    Null,
}

impl Default for StateEvent {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        StateEvent::Null
    }
}
