use bevy::prelude::*;

use characters::{ActionEvent, Attack, GaugeType};

use foundation::{ActionId, Area, SimpleState, StatusFlag, VfxRequest, VoiceLine};

#[derive(Debug, Event)]
pub struct StartAction(pub ActionId);

#[derive(Event)]
pub struct SpawnHitbox(pub Attack);

#[derive(Debug, Event)]
pub struct MultiplyMomentum(pub Vec2);

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
pub struct StartHitstop(pub usize);

#[derive(Debug, Event)]
pub struct TiltCamera(pub Vec2);

#[derive(Debug, Event)]
pub struct ZoomCamera(pub f32);

#[derive(Debug, Event)]
pub struct ShakeCamera;

#[derive(Debug, Event)]
pub struct SpawnRelativeVfx(pub VfxRequest);

#[derive(Debug, Event)]
pub struct SpawnVfx(pub VfxRequest, pub Option<Entity>);

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
            commands.trigger_targets(*ar, trigger.target());
        }
        ActionEvent::Sound(sfx) => {
            commands.trigger(*sfx);
        }
        ActionEvent::StartAction(act) => {
            commands.trigger_targets(StartAction(act.to_owned()), trigger.target());
        }
        ActionEvent::SpawnHitbox(atk) => {
            commands.trigger_targets(SpawnHitbox(atk.clone()), trigger.target());
        }
        ActionEvent::MultiplyMomentum(amount) => {
            commands.trigger_targets(MultiplyMomentum(*amount), trigger.target());
        }
        ActionEvent::Movement(mov) => {
            commands.trigger_targets(*mov, trigger.target());
        }
        ActionEvent::Condition(cond) => {
            commands.trigger_targets(cond.to_owned(), trigger.target());
        }
        ActionEvent::ForceStand => {
            commands.trigger_targets(ForceState(SimpleState::Stand), trigger.target());
        }
        ActionEvent::ForceCrouch => {
            commands.trigger_targets(ForceState(SimpleState::Crouch), trigger.target());
        }
        ActionEvent::ForceAir => {
            commands.trigger_targets(ForceState(SimpleState::Air), trigger.target());
        }
        ActionEvent::ModifyResource(rt, amount) => {
            commands.trigger_targets(
                ModifyResource {
                    resource: *rt,
                    amount: *amount,
                },
                trigger.target(),
            );
        }
        ActionEvent::ClearResource(rt) => {
            commands.trigger_targets(ClearResource(*rt), trigger.target());
        }
        ActionEvent::SnapToOpponent { sideswitch } => {
            commands.trigger_targets(
                SnapToOpponent {
                    sideswitch: *sideswitch,
                },
                trigger.target(),
            );
        }
        // TODO: Maybe these could be compressed to one event that contains a struct?
        ActionEvent::HitStun(hs) => {
            commands.trigger_targets(UpdateHitstun(*hs), trigger.target());
        }
        ActionEvent::BlockStun(bs) => {
            commands.trigger_targets(UpdateBlockstun(*bs), trigger.target());
        }
        ActionEvent::LaunchStun(impulse) => {
            commands.trigger_targets(LaunchImpulse(*impulse), trigger.target());
        }
        ActionEvent::Hitstop(frames) => {
            commands.trigger_targets(StartHitstop(*frames), trigger.target());
        }
        ActionEvent::CameraTilt(tilt) => {
            commands.trigger_targets(TiltCamera(*tilt), trigger.target());
        }
        ActionEvent::Zoom(duration) => {
            commands.trigger_targets(ZoomCamera(*duration), trigger.target());
        }
        ActionEvent::CameraShake => {
            commands.trigger(ShakeCamera);
        }
        ActionEvent::Flash(fr) => {
            commands.trigger_targets(*fr, trigger.target());
        }
        ActionEvent::AbsoluteVisualEffect(vfx) => {
            commands.trigger(SpawnVfx(vfx.clone(), None));
        }
        ActionEvent::RelativeVisualEffect(vfx) => {
            commands.trigger_targets(SpawnRelativeVfx(vfx.clone()), trigger.target());
        }
        ActionEvent::End => {
            commands.trigger_targets(EndAction, trigger.target());
        }
        ActionEvent::ExpandHurtbox(area, duration) => {
            commands.trigger_targets(
                ExpandHurtbox {
                    area: *area,
                    duration: *duration,
                },
                trigger.target(),
            );
        }
        ActionEvent::SayVoiceLine(line) => {
            commands.trigger_targets(ActivateVoiceline(*line), trigger.target());
        }
        ActionEvent::CharacterShake(amount) => {
            commands.trigger_targets(ShakeCharacter(*amount), trigger.target());
        }
        ActionEvent::Teleport(amount) => {
            commands.trigger_targets(TeleportEvent(*amount), trigger.target());
        }
        ActionEvent::ColorShift(color, frames) => {
            commands.trigger_targets(ColorShift(*color, *frames), trigger.target());
        }
        ActionEvent::ClearCondition(flag) => {
            commands.trigger_targets(ClearStatus(flag.clone()), trigger.target());
        }
        ActionEvent::SpawnPickup(pickup_request) => {
            commands.trigger_targets(*pickup_request, trigger.target())
        }
        ActionEvent::FlipVisuals => {
            commands.trigger_targets(FlipVisuals, trigger.target());
        }
        ActionEvent::Noop => {}
    }
}
