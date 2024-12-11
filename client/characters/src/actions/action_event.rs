use bevy::prelude::*;

use foundation::{
    ActionId, Animation, Area, PickupRequest, SamuraiAnimation, SoundEffect, StatusCondition,
    StatusFlag, VfxRequest, VoiceLine,
};

use crate::{FlashRequest, GaugeType, Movement};

use super::{AnimationRequest, Attack};

#[derive(Clone, Default, Event)]
pub enum ActionEvent {
    Animation(AnimationRequest),
    Sound(SoundEffect),
    StartAction(ActionId),
    SpawnHitbox(Attack),
    ClearMovement,
    Movement(Movement),
    Teleport(Vec2),
    Condition(StatusCondition),
    ClearCondition(StatusFlag),
    ForceStand,
    ForceCrouch,
    ForceAir,
    SayVoiceLine(VoiceLine),
    ModifyResource(GaugeType, i32),
    ClearResource(GaugeType),
    SnapToOpponent {
        sideswitch: bool,
    },
    HitStun(usize),
    BlockStun(usize),
    LaunchStun(Vec2),
    Hitstop, // TODO: Add strength
    CameraTilt(Vec2),
    CameraShake, // TODO: Add strength
    Zoom(f32),
    CharacterShake(f32),
    Flash(FlashRequest),
    ColorShift(Color, usize),
    RelativeVisualEffect(VfxRequest),
    AbsoluteVisualEffect(VfxRequest),
    ExpandHurtbox(Area, usize), // New area, how long it should hang around
    SpawnPickup(PickupRequest),
    #[default]
    Noop, // makes writing macros easier
    End, // Ends the move, return to neutral
}

impl std::fmt::Debug for ActionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionEvent::Animation(animation_request) => {
                write!(f, "Animation - {:?}", animation_request)
            }
            ActionEvent::Sound(sound_effect) => {
                write!(f, "Sound - {:?}", sound_effect)
            }
            ActionEvent::StartAction(action_id) => {
                write!(f, "StartAction - {:?}", action_id)
            }
            ActionEvent::SpawnHitbox(attack) => {
                write!(f, "SpawnHitbox - to hit: {:?}", attack.to_hit)
            }
            ActionEvent::ClearMovement => {
                write!(f, "ClearMovement")
            }
            ActionEvent::Movement(movement) => {
                write!(f, "Movement - {:?}", movement)
            }
            ActionEvent::Teleport(vec2) => {
                write!(f, "Teleport - {:?}", vec2)
            }
            ActionEvent::Condition(status_condition) => {
                write!(f, "Condition - {:?}", status_condition)
            }
            ActionEvent::ClearCondition(status_flag) => {
                write!(f, "ClearCondition - {:?}", status_flag)
            }
            ActionEvent::ForceStand => {
                write!(f, "ForceStand")
            }
            ActionEvent::ForceCrouch => {
                write!(f, "ForceCrouch")
            }
            ActionEvent::ForceAir => {
                write!(f, "ForceAir")
            }
            ActionEvent::SayVoiceLine(voice_line) => {
                write!(f, "SayVoiceLine - {:?}", voice_line)
            }
            ActionEvent::ModifyResource(resource_type, amount) => {
                write!(f, "ModifyResource - {:?} by {}", resource_type, amount)
            }
            ActionEvent::ClearResource(resource_type) => {
                write!(f, "ClearResource - {:?}", resource_type)
            }
            ActionEvent::SnapToOpponent { sideswitch } => {
                write!(f, "SnapToOpponent - {}", sideswitch)
            }
            ActionEvent::HitStun(duration) => {
                write!(f, "HitStun - {:?}", duration)
            }
            ActionEvent::BlockStun(duration) => {
                write!(f, "BlockStun - {:?}", duration)
            }
            ActionEvent::LaunchStun(vec2) => {
                write!(f, "LaunchStun - {:?}", vec2)
            }
            ActionEvent::Hitstop => {
                write!(f, "HitStop")
            }
            ActionEvent::CameraTilt(vec2) => {
                write!(f, "CameraTilt - {:?}", vec2)
            }
            ActionEvent::CameraShake => {
                write!(f, "CameraShake")
            }
            ActionEvent::CharacterShake(amount) => {
                write!(f, "CharacterShake - {:?}", amount)
            }
            ActionEvent::Flash(flash_request) => {
                write!(f, "Flash - {:?}", flash_request)
            }
            ActionEvent::ColorShift(color, duration) => {
                write!(f, "ColorShift - {:?} for {}", color, duration)
            }
            ActionEvent::RelativeVisualEffect(vfx_request) => {
                write!(f, "RelativeVisualEffect - {:?}", vfx_request)
            }
            ActionEvent::AbsoluteVisualEffect(vfx_request) => {
                write!(f, "AbsoluteVisualEffect - {:?}", vfx_request)
            }
            ActionEvent::ExpandHurtbox(area, duration) => {
                write!(f, "ExpandHurtbox - {:?} for {}", area, duration)
            }
            ActionEvent::Zoom(duration) => {
                write!(f, "Zoom - {:?}", duration)
            }
            ActionEvent::SpawnPickup(pickup_request) => {
                write!(f, "SpawnPickup - {:?}", pickup_request)
            }
            ActionEvent::Noop => {
                write!(f, "NO-OP")
            }
            ActionEvent::End => {
                write!(f, "End")
            }
        }
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
        ActionEvent::RelativeVisualEffect(value)
    }
}

impl From<SamuraiAnimation> for ActionEvent {
    fn from(value: SamuraiAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
