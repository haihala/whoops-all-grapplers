use bevy::prelude::*;

use types::{Animation, MoveId, SoundEffect};

use crate::{resources::Cost, SpawnDescriptor};

use super::{move_history::Situation, GrabDescription};

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub amount: Vec2,
    pub duration: usize,
}
impl Movement {
    pub(crate) fn impulse(amount: Vec2) -> Self {
        Self {
            amount,
            duration: 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Animation(Animation),
    Sound(SoundEffect),
    Move(MoveId),
    Hitbox(SpawnDescriptor),
    Grab(GrabDescription),
    Movement(Movement),
    Pay(Cost),
    // TODO: Separate projectiles from normal attacks
}

#[derive(Clone, Copy)]
pub enum FlowControl {
    Wait(usize, bool),
    Action(Action),
    Dynamic(fn(Situation) -> FlowControl),
}

impl std::fmt::Debug for FlowControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wait(arg0, arg1) => f.debug_tuple("Wait").field(arg0).field(arg1).finish(),
            Self::Action(arg0) => f.debug_tuple("Action").field(arg0).finish(),
            Self::Dynamic(_) => f.debug_tuple("Dynamic").finish(),
        }
    }
}
impl From<Action> for FlowControl {
    fn from(action: Action) -> Self {
        FlowControl::Action(action)
    }
}

impl From<(usize, bool)> for FlowControl {
    fn from((time, cancellable): (usize, bool)) -> Self {
        FlowControl::Wait(time, cancellable)
    }
}

impl From<usize> for FlowControl {
    fn from(time: usize) -> Self {
        FlowControl::Wait(time, false)
    }
}
