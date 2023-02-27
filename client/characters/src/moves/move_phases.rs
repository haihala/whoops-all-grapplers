use bevy::prelude::*;

use wag_core::{Animation, DummyAnimation, MoveId, SoundEffect, StatusCondition};

use crate::{resources::Cost, Attack};

use super::Situation;

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, FromReflect)]
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

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Default)]
pub enum Action {
    // TODO: Figure out a better way to handle actions that change depending on game state
    // Maybe hoist AnimationRequest?
    Animation(Animation),
    RecipientAnimation(Animation),
    AnimationAtFrame(Animation, usize),
    RecipientAnimationAtFrame(Animation, usize),
    Sound(SoundEffect),
    Move(MoveId),
    Attack(Attack),
    Movement(Movement),
    Pay(Cost),
    Condition(StatusCondition),
    #[default]
    ForceStand,
    TakeDamage(usize),
    SnapToOpponent,
    SideSwitch,
    HitStun(usize),
    BlockStun(usize),
    Launch,
}
impl From<Attack> for Action {
    fn from(value: Attack) -> Self {
        Action::Attack(value)
    }
}
impl From<Animation> for Action {
    fn from(value: Animation) -> Self {
        Action::Animation(value)
    }
}
impl From<Movement> for Action {
    fn from(value: Movement) -> Self {
        Action::Movement(value)
    }
}
impl From<DummyAnimation> for Action {
    fn from(value: DummyAnimation) -> Self {
        Action::Animation(Animation::Dummy(value))
    }
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

#[derive(Clone)]
pub enum FlowControl {
    Wait(usize, CancelPolicy),
    Actions(Vec<Action>),
    DynamicActions(fn(Situation) -> Vec<Action>),
    WaitUntil(fn(Situation) -> bool, Option<usize>),
}

impl std::fmt::Debug for FlowControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicActions(_) => f.debug_tuple("DynamicActions").finish(),
            Self::WaitUntil(_, timeout) => f
                .debug_tuple("WaitUntil with timeout")
                .field(timeout)
                .finish(),
            // Default
            Self::Wait(arg0, arg1) => f.debug_tuple("Wait").field(arg0).field(arg1).finish(),
            Self::Actions(arg0) => f.debug_tuple("Actions").field(arg0).finish(),
        }
    }
}
impl PartialEq for FlowControl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Wait(time1, cancellable1), Self::Wait(time2, cancellable2)) => {
                time1 == time2 && cancellable1 == cancellable2
            }
            (Self::Actions(actions1), Self::Actions(actions2)) => actions1 == actions2,
            (_, Self::DynamicActions(_)) | (Self::DynamicActions(_), _) => {
                panic!("Comparing to a dynamic one")
            }
            _ => false,
        }
    }
}
impl From<Action> for FlowControl {
    fn from(action: Action) -> Self {
        FlowControl::Actions(vec![action])
    }
}
impl From<Vec<Action>> for FlowControl {
    fn from(actions: Vec<Action>) -> Self {
        FlowControl::Actions(actions)
    }
}

impl From<Attack> for FlowControl {
    fn from(value: Attack) -> Self {
        Action::Attack(value).into()
    }
}

impl From<Animation> for FlowControl {
    fn from(value: Animation) -> Self {
        Action::Animation(value).into()
    }
}

impl From<Movement> for FlowControl {
    fn from(value: Movement) -> Self {
        Action::Movement(value).into()
    }
}
impl From<DummyAnimation> for FlowControl {
    fn from(value: DummyAnimation) -> Self {
        Action::Animation(Animation::Dummy(value)).into()
    }
}
