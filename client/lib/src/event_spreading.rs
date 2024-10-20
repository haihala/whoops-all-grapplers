use bevy::prelude::*;
use characters::{ActionEvent, AnimationRequest, Attack, FlashRequest, Movement, ResourceType};
use wag_core::{ActionId, Area, CancelWindow, SoundEffect, StatusCondition, VfxRequest};

#[derive(Debug, Event)]
pub struct AllowCancel(pub CancelWindow);

#[derive(Debug, Event)]
pub struct StartAnimation(pub AnimationRequest);

#[derive(Debug, Event)]
pub struct PlaySound(pub SoundEffect);

#[derive(Debug, Event)]
pub struct StartAction(pub ActionId);

#[derive(Debug, Event)]
pub struct SpawnHitbox(pub Attack);

#[derive(Debug, Event)]
pub struct ClearMovement;

#[derive(Debug, Event)]
pub struct AddMovement(pub Movement);

#[derive(Debug, Event)]
pub struct AddCondition(pub StatusCondition);

#[derive(Debug, Event)]
pub struct ForceStand;

#[derive(Debug, Event)]
pub struct ModifyResource {
    pub resource: ResourceType,
    pub amount: i32,
}

#[derive(Debug, Event)]
pub struct ClearResource(pub ResourceType);

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
pub struct StartHitstop;

#[derive(Debug, Event)]
pub struct TiltCamera(pub Vec2);

#[derive(Debug, Event)]
pub struct ShakeCamera;

#[derive(Debug, Event)]
pub struct FlashPlayer(pub FlashRequest);

#[derive(Debug, Event)]
pub struct SpawnRelativeVfx(pub VfxRequest);

#[derive(Debug, Event)]
pub struct SpawnVfx(pub VfxRequest);

#[derive(Debug, Event)]
pub struct LockPlayer(pub usize);

#[derive(Debug, Event)]
pub struct EndAction;

#[derive(Debug, Event)]
pub struct ExpandHurtbox {
    pub area: Area,
    pub duration: usize,
}

pub fn spread_events(trigger: Trigger<ActionEvent>, mut commands: Commands) {
    match trigger.event() {
        ActionEvent::AllowCancel(cw) => {
            commands.trigger_targets(AllowCancel(cw.to_owned()), trigger.entity());
        }
        ActionEvent::Animation(ar) => {
            commands.trigger_targets(StartAnimation(ar.to_owned()), trigger.entity());
        }
        ActionEvent::Sound(sfx) => {
            commands.trigger(PlaySound(sfx.to_owned()));
        }
        ActionEvent::StartAction(act) => {
            commands.trigger_targets(StartAction(act.to_owned()), trigger.entity());
        }
        ActionEvent::Attack(atk) => {
            commands.trigger_targets(SpawnHitbox(atk.to_owned()), trigger.entity());
        }
        ActionEvent::ClearMovement => {
            commands.trigger_targets(ClearMovement, trigger.entity());
        }
        ActionEvent::Movement(mov) => {
            commands.trigger_targets(AddMovement(mov.to_owned()), trigger.entity());
        }
        ActionEvent::Condition(cond) => {
            commands.trigger_targets(AddCondition(cond.to_owned()), trigger.entity());
        }
        ActionEvent::ForceStand => {
            commands.trigger_targets(ForceStand, trigger.entity());
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
            commands.trigger(StartHitstop);
        }
        ActionEvent::CameraTilt(tilt) => {
            commands.trigger_targets(TiltCamera(*tilt), trigger.entity());
        }
        ActionEvent::CameraShake => {
            commands.trigger(ShakeCamera);
        }
        ActionEvent::Flash(fr) => {
            commands.trigger_targets(FlashPlayer(*fr), trigger.entity());
        }
        ActionEvent::VisualEffect(vfx) => {
            commands.trigger_targets(SpawnRelativeVfx(*vfx), trigger.entity());
        }
        ActionEvent::Lock(dur) => {
            commands.trigger_targets(LockPlayer(*dur), trigger.entity());
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
        ActionEvent::Noop => {}
    }
}
