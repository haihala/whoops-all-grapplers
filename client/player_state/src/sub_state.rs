use bevy_inspector_egui::Inspectable;

use characters::MoveHistory;
use types::Facing;

#[derive(Inspectable, Clone, Debug)]
pub enum Stun {
    Block(usize),
    Hit(usize),
}
impl Default for Stun {
    fn default() -> Self {
        panic!("Ought to never be used, just satisfies inspectable")
    }
}
impl Stun {
    pub fn get_frame(&self) -> usize {
        *match self {
            Stun::Block(frames) => frames,
            Stun::Hit(frames) => frames,
        }
    }
}

#[derive(Inspectable, Clone, Debug, Default)]
pub enum AirState {
    Freefall,
    Move(MoveHistory),
    #[default]
    Idle,
}

#[derive(Inspectable, Clone, Debug, Default)]
pub enum StandState {
    Stun(Stun),
    Move(MoveHistory),
    Walk(Facing),
    #[default]
    Idle,
}

#[derive(Inspectable, Clone, Debug, Default)]
pub enum CrouchState {
    Stun(Stun),
    Move(MoveHistory),
    #[default]
    Idle,
}
