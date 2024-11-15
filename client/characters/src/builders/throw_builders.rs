use bevy::prelude::*;
use wag_core::{Animation, StatusCondition, StatusFlag};

use crate::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, FlashRequest, ResourceType, Situation,
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
        assert!(self.lock_duration < self.target_duration);
        assert!(self.lock_duration < self.self_duration);

        (
            Action {
                input: None,
                script: Box::new(move |situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![
                            AnimationRequest {
                                invert: true,
                                ..AnimationRequest::from(self.target_animation)
                            }
                            .into(),
                            ActionEvent::Flash(FlashRequest::hit_flash()),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::MovementLock,
                                expiration: Some(self.lock_duration),
                                ..default()
                            }),
                        ];
                    }

                    if situation.on_frame(self.lock_duration) {
                        return vec![
                            ActionEvent::LaunchStun(self.launch_impulse),
                            ActionEvent::ModifyResource(ResourceType::Health, -self.damage),
                        ]
                        .into_iter()
                        .chain(self.extra_target_events.clone())
                        .collect();
                    }

                    situation.end_at(self.target_duration)
                }),
                requirement: ActionRequirement::default(),
            },
            Action {
                input: None,
                script: Box::new(move |situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![
                            self.self_animation.into(),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::MovementLock,
                                expiration: Some(self.lock_duration),
                                ..default()
                            }),
                        ];
                    }

                    situation.end_at(self.self_duration)
                }),
                requirement: ActionRequirement::default(),
            },
        )
    }
}
