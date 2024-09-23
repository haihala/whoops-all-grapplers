use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, Area, CancelWindow, DummyAnimation, MizkuAnimation, SoundEffect,
    StatusCondition, VfxRequest,
};

use crate::{Attack, FlashRequest, Movement, ResourceType};

use super::AnimationRequest;

#[derive(Debug, Clone, PartialEq, Default, Event)]
pub enum ActionEvent {
    AllowCancel(CancelWindow),
    Animation(AnimationRequest),
    Sound(SoundEffect),
    StartAction(ActionId),
    Attack(Attack),
    ClearMovement,
    Movement(Movement),
    Condition(StatusCondition),
    ForceStand,
    ModifyResource(ResourceType, i32),
    ClearResource(ResourceType),
    SnapToOpponent {
        sideswitch: bool,
    },
    HitStun(usize),
    BlockStun(usize),
    LaunchStun(Vec2),
    Hitstop, // TODO: Add strength
    CameraTilt(Vec2),
    CameraShake, // TODO: Add strength
    Flash(FlashRequest),
    VisualEffect(VfxRequest),
    Lock(usize),                // duration
    ExpandHurtbox(Area, usize), // New area, how long it should hang around
    #[default]
    Noop,        // makes writing macros easier
    End,                        // Ends the move, return to neutral
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
impl From<VfxRequest> for ActionEvent {
    fn from(value: VfxRequest) -> Self {
        ActionEvent::VisualEffect(value)
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
