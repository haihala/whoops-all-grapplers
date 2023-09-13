use bevy::prelude::*;

use wag_core::{Animation, DummyAnimation, MoveId, SoundEffect, StatusCondition};

use crate::{Attack, PropertyType};

use super::Situation;

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
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

#[derive(Debug, Clone, PartialEq, Default, Reflect)]
pub enum ActionEvent {
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
    Condition(StatusCondition),
    #[default]
    ForceStand,
    ModifyProperty(PropertyType, i32),
    ClearProperty(PropertyType),
    SnapToOpponent,
    SideSwitch,
    HitStun(usize),
    BlockStun(usize),
    Launch,
}
impl From<Attack> for ActionEvent {
    fn from(value: Attack) -> Self {
        ActionEvent::Attack(value)
    }
}
impl From<Animation> for ActionEvent {
    fn from(value: Animation) -> Self {
        ActionEvent::Animation(value)
    }
}
impl From<Movement> for ActionEvent {
    fn from(value: Movement) -> Self {
        ActionEvent::Movement(value)
    }
}
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::Dummy(value))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum CancelCategory {
    Any,
    Jump,
    Dash,
    Normal,
    CommandNormal,
    Special,
    Everything, // Usable for tests as a "this is cancellable from anything that is cancellable"
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CancelRule {
    pub requires_hit: bool,
    pub category: CancelCategory,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CancelPolicy(pub Vec<CancelRule>);
impl CancelPolicy {
    pub fn never() -> Self {
        Self(vec![])
    }

    pub fn any() -> Self {
        Self(vec![CancelRule {
            requires_hit: false,
            category: CancelCategory::Any,
        }])
    }

    pub fn neutral_normal_recovery() -> Self {
        Self(vec![CancelRule {
            requires_hit: true,
            category: CancelCategory::CommandNormal,
        }])
    }

    pub fn command_normal_recovery() -> Self {
        Self(vec![CancelRule {
            requires_hit: true,
            category: CancelCategory::Special,
        }])
    }

    pub fn can_cancel(&self, hit: bool, cancel_category: CancelCategory) -> bool {
        self.0
            .iter()
            .any(|rule| (hit || !rule.requires_hit) && rule.category <= cancel_category)
    }
}

#[derive(Clone)]
pub enum FlowControl {
    Wait(usize, CancelPolicy),
    Actions(Vec<ActionEvent>),
    DynamicActions(fn(Situation) -> Vec<ActionEvent>),
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
impl From<ActionEvent> for FlowControl {
    fn from(action: ActionEvent) -> Self {
        FlowControl::Actions(vec![action])
    }
}
impl From<Vec<ActionEvent>> for FlowControl {
    fn from(actions: Vec<ActionEvent>) -> Self {
        FlowControl::Actions(actions)
    }
}

impl From<Attack> for FlowControl {
    fn from(value: Attack) -> Self {
        ActionEvent::Attack(value).into()
    }
}

impl From<Animation> for FlowControl {
    fn from(value: Animation) -> Self {
        ActionEvent::Animation(value).into()
    }
}

impl From<Movement> for FlowControl {
    fn from(value: Movement) -> Self {
        ActionEvent::Movement(value).into()
    }
}
impl From<DummyAnimation> for FlowControl {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::Dummy(value)).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cancel_sanity_check() {
        assert!(CancelPolicy::any().can_cancel(true, CancelCategory::Everything));
        assert!(CancelPolicy::any().can_cancel(false, CancelCategory::Everything));
        assert!(!CancelPolicy::never().can_cancel(true, CancelCategory::Everything));
    }
}
