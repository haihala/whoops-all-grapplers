use bevy_inspector_egui::Inspectable;

use characters::MoveSituation;
use types::Facing;

#[derive(Inspectable, Eq, PartialEq, Clone, Debug)]
pub enum AirState {
    Freefall,
    Move(MoveSituation),
    Idle,
}

impl Default for AirState {
    fn default() -> Self {
        AirState::Idle
    }
}

#[derive(Inspectable, Eq, PartialEq, Clone, Debug)]
pub enum StandState {
    Stun(usize),
    Move(MoveSituation),
    Walk(Facing),
    Idle,
}

impl Default for StandState {
    fn default() -> Self {
        StandState::Idle
    }
}

#[derive(Inspectable, Eq, PartialEq, Clone, Debug)]
pub enum CrouchState {
    Stun(usize),
    Move(MoveSituation),
    Idle,
}

impl Default for CrouchState {
    fn default() -> Self {
        CrouchState::Idle
    }
}
