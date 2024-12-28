use bevy::prelude::*;
use foundation::{
    Animation, StatusCondition, StatusFlag, VfxRequest, VisualEffect, ON_THROW_HITSTOP,
};

use crate::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, FlashRequest, GaugeType, Situation,
};

pub struct ThrowEffectBuilder {
    self_animation: Animation,
    target_animation: Animation,
    self_duration: usize,
    target_duration: usize,
    lock_duration: usize,
    damage: i32,
    launch_impulse: Vec2,
    extra_target_events: Vec<ActionEvent>,
}
impl ThrowEffectBuilder {
    pub fn new(
        self_animation: impl Into<Animation>,
        self_duration: usize,
        target_animation: impl Into<Animation>,
        target_duration: usize,
    ) -> Self {
        Self {
            self_duration,
            target_duration,
            self_animation: self_animation.into(),
            target_animation: target_animation.into(),
            lock_duration: self_duration.min(target_duration) - 1,
            launch_impulse: Vec2::ZERO,
            extra_target_events: vec![],
            damage: 0,
        }
    }

    pub fn with_extra_target_events(self, extra_target_events: Vec<ActionEvent>) -> Self {
        Self {
            extra_target_events,
            ..self
        }
    }

    pub fn with_launch_impulse(self, launch_impulse: Vec2) -> Self {
        Self {
            // Flip X to make describing backwards knockback use a positive number
            launch_impulse: Vec2::new(-launch_impulse.x, launch_impulse.y),
            ..self
        }
    }

    pub fn with_damage(self, damage: i32) -> Self {
        Self { damage, ..self }
    }

    pub fn build(self) -> (Action, Action) {
        debug_assert!(self.lock_duration < self.target_duration);
        debug_assert!(self.lock_duration < self.self_duration);

        (
            Action {
                transient: false,
                input: None,
                script: Box::new(move |situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![
                            ActionEvent::FlipVisuals,
                            AnimationRequest {
                                invert: true,
                                ignore_action_speed: true,
                                ..AnimationRequest::from(self.target_animation)
                            }
                            .into(),
                            ActionEvent::RelativeVisualEffect(VfxRequest {
                                effect: VisualEffect::ThrowTarget,
                                tf: Transform::from_translation(Vec3::Y),
                                ..default()
                            }),
                            ActionEvent::CameraShake,
                            ActionEvent::Zoom(1.0),
                            ActionEvent::Hitstop(ON_THROW_HITSTOP),
                            ActionEvent::Flash(FlashRequest::hit_flash()),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::MovementLock,
                                expiration: Some(self.lock_duration),
                                ..default()
                            }),
                        ];
                    }

                    if situation.elapsed() == self.lock_duration {
                        return vec![
                            ActionEvent::LaunchStun(self.launch_impulse),
                            ActionEvent::ModifyResource(GaugeType::Health, -self.damage),
                            ActionEvent::FlipVisuals,
                        ]
                        .into_iter()
                        .chain(self.extra_target_events.clone())
                        .collect();
                    }

                    // Done this way to avoid including animation speed from items
                    if situation.elapsed() >= self.target_duration {
                        vec![ActionEvent::End]
                    } else {
                        vec![]
                    }
                }),
                requirement: ActionRequirement::default(),
            },
            Action {
                transient: false,
                input: None,
                script: Box::new(move |situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![
                            AnimationRequest {
                                ignore_action_speed: true,
                                ..AnimationRequest::from(self.self_animation)
                            }
                            .into(),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::MovementLock,
                                expiration: Some(self.lock_duration),
                                ..default()
                            }),
                            ActionEvent::Hitstop(ON_THROW_HITSTOP),
                        ];
                    }

                    // Done this way to avoid including animation speed from items
                    if situation.elapsed() >= self.self_duration {
                        vec![ActionEvent::End]
                    } else {
                        vec![]
                    }
                }),
                requirement: ActionRequirement::default(),
            },
        )
    }
}
