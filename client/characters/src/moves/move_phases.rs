use bevy::prelude::*;

use wag_core::{Animation, MoveId, SoundEffect, StatusCondition};

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
    ForceStand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CancelPolicy {
    IfHit,
    Always,
    Never,
}
impl CancelPolicy {
    pub fn can_cancel(&self, hit: bool) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::IfHit => hit,
        }
    }
}

#[derive(Clone, Copy)]
pub enum FlowControl {
    Wait(usize, CancelPolicy),
    Action(Action),
    Noop,
    DynamicAction(fn(Situation) -> Option<Action>),
    WaitUntil(fn(Situation) -> bool, Option<usize>),
}

impl std::fmt::Debug for FlowControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicAction(_) => f.debug_tuple("DynamicAction").finish(),
            Self::WaitUntil(_, timeout) => f
                .debug_tuple("WaitUntil with timeout")
                .field(timeout)
                .finish(),
            // Default
            Self::Wait(arg0, arg1) => f.debug_tuple("Wait").field(arg0).field(arg1).finish(),
            Self::Action(arg0) => f.debug_tuple("Action").field(arg0).finish(),
            Self::Noop => f.debug_tuple("Noop").finish(),
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
            (Self::Noop, Self::Noop) => true,
            (_, Self::DynamicAction(_)) | (Self::DynamicAction(_), _) => {
                panic!("Comparing to a dynamic one")
            }
            _ => false,
        }
    }
}
impl From<Action> for FlowControl {
    fn from(action: Action) -> Self {
        FlowControl::Action(action)
    }
}

impl From<(usize, CancelPolicy)> for FlowControl {
    fn from((time, cancel_policy): (usize, CancelPolicy)) -> Self {
        FlowControl::Wait(time, cancel_policy)
    }
}

impl From<usize> for FlowControl {
    fn from(time: usize) -> Self {
        FlowControl::Wait(time, CancelPolicy::Never)
    }
}
