use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, DummyAnimation, ItemId, MizkuAnimation, SoundEffect, StatusCondition,
};

use crate::{Attack, FlashRequest, Movement, ResourceType};

use super::AnimationRequest;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ActionEvent {
    Animation(AnimationRequest),
    Consume(ItemId),
    Sound(SoundEffect),
    StartAction(ActionId),
    Attack(Attack),
    ClearMovement,
    Movement(Movement),
    Condition(StatusCondition),
    #[default]
    ForceStand,
    ModifyResource(ResourceType, i32),
    ClearResource(ResourceType),
    SnapToOpponent,
    SideSwitch,
    HitStun(usize),
    BlockStun(usize),
    Launch {
        impulse: Vec2,
    },
    Hitstop, // TODO: Add strength
    CameraTilt(Vec2),
    CameraShake, // TODO: Add strength
    Flash(FlashRequest),
    Lock(usize), // duration, sideswitch
    Noop,        // makes writing macros easier
}

impl From<Attack> for ActionEvent {
    fn from(value: Attack) -> Self {
        ActionEvent::Attack(value)
    }
}
impl From<Animation> for ActionEvent {
    fn from(value: Animation) -> Self {
        ActionEvent::Animation(value.into())
    }
}
impl From<Movement> for ActionEvent {
    fn from(value: Movement) -> Self {
        ActionEvent::Movement(value)
    }
}
impl From<AnimationRequest> for ActionEvent {
    fn from(value: AnimationRequest) -> Self {
        ActionEvent::Animation(value)
    }
}
impl From<SoundEffect> for ActionEvent {
    fn from(value: SoundEffect) -> Self {
        ActionEvent::Sound(value)
    }
}
// This isn't a great way to do this, but it's the best I can think of for now
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
impl From<MizkuAnimation> for ActionEvent {
    fn from(value: MizkuAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
