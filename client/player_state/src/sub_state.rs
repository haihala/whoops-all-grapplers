use bevy::prelude::*;

use characters::ActionTracker;
use wag_core::Facing;

#[derive(Reflect, Clone, Debug, Hash)]
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

#[derive(Reflect, Clone, Debug, Default, Hash)]
pub enum AirState {
    Freefall,
    Move(ActionTracker),
    #[default]
    Idle,
}

#[derive(Reflect, Clone, Debug, Default, Hash)]
pub enum StandState {
    Stun(Stun),
    Move(ActionTracker),
    Walk(Facing),
    #[default]
    Idle,
}

#[derive(Reflect, Clone, Debug, Default, Hash)]
pub enum CrouchState {
    Stun(Stun),
    Move(ActionTracker),
    #[default]
    Idle,
}
