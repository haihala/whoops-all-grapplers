use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use types::{AttackDescriptor, MoveId};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum StateEvent {
    Jump(Vec3),
    Attack(MoveId, AttackDescriptor),
    Recovery,
    PhaseChange,
    Null,
}

impl Default for StateEvent {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        StateEvent::Null
    }
}
