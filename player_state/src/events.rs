use bevy_inspector_egui::Inspectable;

use types::{GrabDescription, MoveId, SpawnDescriptor};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum StateEvent {
    Attack(MoveId, SpawnDescriptor),
    Grab(GrabDescription),
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
