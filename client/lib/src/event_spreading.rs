use std::time::Duration;

use bevy::prelude::*;

use characters::{ActionEvent, Attack, GaugeType};

use foundation::{ActionId, Area, SimpleState, StatusFlag, VfxRequest, VoiceLine};

#[derive(Debug, Event)]
pub struct StartAction(pub ActionId);

#[derive(Event)]
pub struct SpawnHitbox(pub Attack);

#[derive(Debug, Event)]
pub struct ClearMovement;

#[derive(Debug, Event)]
pub struct ForceState(pub SimpleState);

#[derive(Debug, Event, Clone, Copy)]
pub struct ModifyResource {
    pub resource: GaugeType,
    pub amount: i32,
}

#[derive(Debug, Event)]
pub struct ClearResource(pub GaugeType);

#[derive(Debug, Event)]
pub struct SnapToOpponent {
    pub sideswitch: bool,
}

#[derive(Debug, Event)]
pub struct UpdateHitstun(pub usize);

#[derive(Debug, Event)]
pub struct UpdateBlockstun(pub usize);

#[derive(Debug, Event)]
pub struct LaunchImpulse(pub Vec2);

#[derive(Debug, Event)]
pub struct StartHitstop(pub Duration);

#[derive(Debug, Event)]
pub struct TiltCamera(pub Vec2);

#[derive(Debug, Event)]
pub struct ZoomCamera(pub f32);

#[derive(Debug, Event)]
pub struct ShakeCamera;

#[derive(Debug, Event)]
pub struct SpawnRelativeVfx(pub VfxRequest);

#[derive(Debug, Event)]
pub struct SpawnVfx(pub VfxRequest);

#[derive(Debug, Event)]
pub struct EndAction;

#[derive(Debug, Event)]
pub struct ExpandHurtbox {
    pub area: Area,
    pub duration: usize,
}

#[derive(Debug, Event)]
pub struct ActivateVoiceline(pub VoiceLine);

#[derive(Debug, Event)]
pub struct ShakeCharacter(pub f32);

#[derive(Debug, Event)]
pub struct TeleportEvent(pub Vec2);

#[derive(Debug, Event)]
pub struct ColorShift(pub Color, pub usize);

#[derive(Debug, Event)]
pub struct ClearStatus(pub StatusFlag);

#[derive(Debug, Event)]
pub struct FlipVisuals;

pub fn spread_events(trigger: Trigger<ActionEvent>, mut commands: Commands) {
    match trigger.event() {
        ActionEvent::Animation(ar) => {
            commands.trigger_targets(*ar, trigger.entity());
        }
        ActionEvent::Sound(sfx) => {
            commands.trigger(*sfx);
        }
        ActionEvent::StartAction(act) => {
            commands.trigger_targets(StartAction(act.to_owned()), trigger.entity());
        }
        ActionEvent::SpawnHitbox(atk) => {
            commands.trigger_targets(SpawnHitbox(atk.clone()), trigger.entity());
        }
        ActionEvent::ClearMovement => {
            commands.trigger_targets(ClearMovement, trigger.entity());
        }
        ActionEvent::Movement(mov) => {
            commands.trigger_targets(*mov, trigger.entity());
        }
        ActionEvent::Condition(cond) => {
            commands.trigger_targets(cond.to_owned(), trigger.entity());
        }
        ActionEvent::ForceStand => {
            commands.trigger_targets(ForceState(SimpleState::Stand), trigger.entity());
        }
        ActionEvent::ForceCrouch => {
            commands.trigger_targets(ForceState(SimpleState::Crouch), trigger.entity());
        }
        ActionEvent::ForceAir => {
            commands.trigger_targets(ForceState(SimpleState::Air), trigger.entity());
        }
        ActionEvent::ModifyResource(rt, amount) => {
            commands.trigger_targets(
                ModifyResource {
                    resource: *rt,
                    amount: *amount,
                },
                trigger.entity(),
            );
        }
        ActionEvent::ClearResource(rt) => {
            commands.trigger_targets(ClearResource(*rt), trigger.entity());
        }
        ActionEvent::SnapToOpponent { sideswitch } => {
            commands.trigger_targets(
                SnapToOpponent {
                    sideswitch: *sideswitch,
                },
                trigger.entity(),
            );
        }
        // TODO: Maybe these could be compressed to one event that contains a struct?
        ActionEvent::HitStun(hs) => {
            commands.trigger_targets(UpdateHitstun(*hs), trigger.entity());
        }
        ActionEvent::BlockStun(bs) => {
            commands.trigger_targets(UpdateBlockstun(*bs), trigger.entity());
        }
        ActionEvent::LaunchStun(impulse) => {
            commands.trigger_targets(LaunchImpulse(*impulse), trigger.entity());
        }
        ActionEvent::Hitstop => {
            // TODO: Enable event to set the duration
            commands.trigger(StartHitstop(Duration::from_millis(100)));
        }
        ActionEvent::CameraTilt(tilt) => {
            commands.trigger_targets(TiltCamera(*tilt), trigger.entity());
        }
        ActionEvent::Zoom(duration) => {
            commands.trigger_targets(ZoomCamera(*duration), trigger.entity());
        }
        ActionEvent::CameraShake => {
            commands.trigger(ShakeCamera);
        }
        ActionEvent::Flash(fr) => {
            commands.trigger_targets(*fr, trigger.entity());
        }
        ActionEvent::AbsoluteVisualEffect(vfx) => {
            commands.trigger(SpawnVfx(*vfx));
        }
        ActionEvent::RelativeVisualEffect(vfx) => {
            commands.trigger_targets(SpawnRelativeVfx(*vfx), trigger.entity());
        }
        ActionEvent::End => {
            commands.trigger_targets(EndAction, trigger.entity());
        }
        ActionEvent::ExpandHurtbox(area, duration) => {
            commands.trigger_targets(
                ExpandHurtbox {
                    area: *area,
                    duration: *duration,
                },
                trigger.entity(),
            );
        }
        ActionEvent::SayVoiceLine(line) => {
            commands.trigger_targets(ActivateVoiceline(*line), trigger.entity());
        }
        ActionEvent::CharacterShake(amount) => {
            commands.trigger_targets(ShakeCharacter(*amount), trigger.entity());
        }
        ActionEvent::Teleport(amount) => {
            commands.trigger_targets(TeleportEvent(*amount), trigger.entity());
        }
        ActionEvent::ColorShift(color, frames) => {
            commands.trigger_targets(ColorShift(*color, *frames), trigger.entity());
        }
        ActionEvent::ClearCondition(flag) => {
            commands.trigger_targets(ClearStatus(flag.clone()), trigger.entity());
        }
        ActionEvent::SpawnPickup(pickup_request) => {
            commands.trigger_targets(*pickup_request, trigger.entity())
        }
        ActionEvent::FlipVisuals => {
            commands.trigger_targets(FlipVisuals, trigger.entity());
        }
        ActionEvent::Noop => {}
    }
}
