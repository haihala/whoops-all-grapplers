use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, Area, CancelWindow, DummyAnimation, SamuraiAnimation, SoundEffect,
    StatusCondition, VfxRequest, VoiceLine,
};

use crate::{FlashRequest, Movement, ResourceType};

use super::{AnimationRequest, Attack};

#[derive(Clone, Default, Event)]
pub enum ActionEvent {
    AllowCancel(CancelWindow),
    Animation(AnimationRequest),
    Sound(SoundEffect),
    StartAction(ActionId),
    SpawnHitbox(Attack),
    ClearMovement,
    Movement(Movement),
    Condition(StatusCondition),
    ForceStand,
    SayVoiceLine(VoiceLine),
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
    CharacterShake(f32),
    Flash(FlashRequest),
    RelativeVisualEffect(VfxRequest),
    AbsoluteVisualEffect(VfxRequest),
    ExpandHurtbox(Area, usize), // New area, how long it should hang around
    #[default]
    Noop,        // makes writing macros easier
    End,                        // Ends the move, return to neutral
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
        ActionEvent::RelativeVisualEffect(value)
    }
}
// This isn't a great way to do this, but it's the best I can think of for now
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
impl From<SamuraiAnimation> for ActionEvent {
    fn from(value: SamuraiAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
