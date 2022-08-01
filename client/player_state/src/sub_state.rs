use bevy_inspector_egui::Inspectable;

use characters::MoveHistory;
use types::Facing;

#[derive(Inspectable, Clone, Debug, Default)]
pub enum AirState {
    Freefall,
    Move(MoveHistory),
    #[default]
    Idle,
}

#[derive(Inspectable, Clone, Debug, Default)]
pub enum StandState {
    Stun(usize),
    Move(MoveHistory),
    Walk(Facing),
    #[default]
    Idle,
}

#[derive(Inspectable, Clone, Debug, Default)]
pub enum CrouchState {
    Stun(usize),
    Move(MoveHistory),
    #[default]
    Idle,
}
