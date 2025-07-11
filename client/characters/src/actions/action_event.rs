use bevy::prelude::*;

use foundation::{
    ActionId, Animation, Area, PickupRequest, RoninAnimation, SoundRequest, StatusCondition,
    StatusFlag, VfxRequest, VoiceLine,
};

use crate::{FlashRequest, GaugeType, Movement};

use super::{AnimationRequest, Attack};

#[derive(Clone, Default, Event)]
pub enum ActionEvent {
    Animation(AnimationRequest),
    Sound(SoundRequest),
    StartAction(ActionId),
    SpawnHitbox(Attack),
    MultiplyMomentum(Vec2),
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
    FlipVisuals,
    HitStun(usize),
    BlockStun(usize),
    LaunchStun(Vec2),
    Hitstop(usize),
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
                write!(f, "Animation - {animation_request:?}")
            }
            ActionEvent::Sound(sound_effect) => {
                write!(f, "Sound - {sound_effect:?}")
            }
            ActionEvent::StartAction(action_id) => {
                write!(f, "StartAction - {action_id:?}")
            }
            ActionEvent::SpawnHitbox(attack) => {
                write!(f, "SpawnHitbox - to hit: {:?}", attack.to_hit)
            }
            ActionEvent::MultiplyMomentum(amount) => {
                write!(f, "MultiplyMomentum - {amount:?}")
            }
            ActionEvent::Movement(movement) => {
                write!(f, "Movement - {movement:?}")
            }
            ActionEvent::Teleport(offset) => {
                write!(f, "Teleport - {offset:?}")
            }
            ActionEvent::Condition(status_condition) => {
                write!(f, "Condition - {status_condition:?}")
            }
            ActionEvent::ClearCondition(status_flag) => {
                write!(f, "ClearCondition - {status_flag:?}")
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
                write!(f, "SayVoiceLine - {voice_line:?}")
            }
            ActionEvent::ModifyResource(resource_type, amount) => {
                write!(f, "ModifyResource - {resource_type:?} by {amount}")
            }
            ActionEvent::ClearResource(resource_type) => {
                write!(f, "ClearResource - {resource_type:?}")
            }
            ActionEvent::SnapToOpponent { sideswitch } => {
                write!(f, "SnapToOpponent - {sideswitch}")
            }
            ActionEvent::HitStun(duration) => {
                write!(f, "HitStun - {duration:?}")
            }
            ActionEvent::BlockStun(duration) => {
                write!(f, "BlockStun - {duration:?}")
            }
            ActionEvent::LaunchStun(impulse) => {
                write!(f, "LaunchStun - {impulse:?}")
            }
            ActionEvent::Hitstop(frames) => {
                write!(f, "HitStop - {frames:?}")
            }
            ActionEvent::CameraTilt(tilt) => {
                write!(f, "CameraTilt - {tilt:?}",)
            }
            ActionEvent::CameraShake => {
                write!(f, "CameraShake")
            }
            ActionEvent::CharacterShake(amount) => {
                write!(f, "CharacterShake - {amount:?}")
            }
            ActionEvent::Flash(flash_request) => {
                write!(f, "Flash - {flash_request:?}")
            }
            ActionEvent::ColorShift(color, duration) => {
                write!(f, "ColorShift - {color:?} for {duration}")
            }
            ActionEvent::RelativeVisualEffect(vfx_request) => {
                write!(f, "RelativeVisualEffect - {vfx_request:?}")
            }
            ActionEvent::AbsoluteVisualEffect(vfx_request) => {
                write!(f, "AbsoluteVisualEffect - {vfx_request:?}")
            }
            ActionEvent::ExpandHurtbox(area, duration) => {
                write!(f, "ExpandHurtbox - {area:?} for {duration}")
            }
            ActionEvent::Zoom(duration) => {
                write!(f, "Zoom - {duration:?}")
            }
            ActionEvent::SpawnPickup(pickup_request) => {
                write!(f, "SpawnPickup - {pickup_request:?}")
            }
            ActionEvent::FlipVisuals => {
                write!(f, "FlipVisuals")
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
impl From<SoundRequest> for ActionEvent {
    fn from(value: SoundRequest) -> Self {
        ActionEvent::Sound(value)
    }
}
impl From<VfxRequest> for ActionEvent {
    fn from(value: VfxRequest) -> Self {
        ActionEvent::RelativeVisualEffect(value)
    }
}

impl From<RoninAnimation> for ActionEvent {
    fn from(value: RoninAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
