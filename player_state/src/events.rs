use bevy_inspector_egui::Inspectable;

use types::AbsoluteDirection;

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
pub enum AnimationEvent {
    StartActive,
    EndActive,
    Recovered,
    Null,
}
impl Default for AnimationEvent {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        AnimationEvent::Null
    }
}

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum StateEvent {
    Jump(JumpDirection),
    AnimationUpdate(AnimationEvent),
    Null,
}

impl Default for StateEvent {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        StateEvent::Null
    }
}
