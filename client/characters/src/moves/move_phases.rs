use bevy::prelude::*;

use core::{Animation, MoveId, SoundEffect, StatusCondition};

use crate::resources::Cost;

use super::{OnHitEffect, Situation, ToHit};

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Animation(Animation),
    AnimationAtFrame(Animation, usize),
    Sound(SoundEffect),
    Move(MoveId),
    Attack(ToHit, OnHitEffect),
    Movement(Movement),
    Pay(Cost),
    Condition(StatusCondition),
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
impl PartialEq for FlowControl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Wait(time1, cancellable1), Self::Wait(time2, cancellable2)) => {
                time1 == time2 && cancellable1 == cancellable2
            }
            (Self::Action(action1), Self::Action(action2)) => action1 == action2,
            (_, Self::Dynamic(_)) | (Self::Dynamic(_), _) => panic!("Comparing to a dynamic one"),
            _ => false,
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
