use bevy_inspector_egui::Inspectable;

use characters::MoveSituation;
use types::Facing;

#[derive(Inspectable, Eq, PartialEq, Clone, Debug, Default)]
pub enum AirState {
    Freefall,
    Move(MoveSituation),
    #[default]
    Idle,
}

#[derive(Inspectable, Eq, PartialEq, Clone, Debug, Default)]
pub enum StandState {
    Stun(usize),
    Move(MoveSituation),
    Walk(Facing),
    #[default]
    Idle,
}

#[derive(Inspectable, Eq, PartialEq, Clone, Debug, Default)]
pub enum CrouchState {
    Stun(usize),
    Move(MoveSituation),
    #[default]
    Idle,
}
