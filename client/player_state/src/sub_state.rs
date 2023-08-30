use bevy::prelude::*;

use characters::MoveHistory;
use wag_core::Facing;

#[derive(Reflect, Clone, Debug)]
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

#[derive(Reflect, Clone, Debug, Default)]
pub enum AirState {
    Freefall,
    Move(MoveHistory),
    #[default]
    Idle,
}

#[derive(Reflect, Clone, Debug, Default)]
pub enum StandState {
    Stun(Stun),
    Move(MoveHistory),
    Walk(Facing),
    #[default]
    Idle,
}

#[derive(Reflect, Clone, Debug, Default)]
pub enum CrouchState {
    Stun(Stun),
    Move(MoveHistory),
    #[default]
    Idle,
}
