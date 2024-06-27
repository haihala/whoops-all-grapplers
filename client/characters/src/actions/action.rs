use bevy::prelude::*;
use wag_core::Animation;

use crate::{
    ActionBlock, ActionEvent, ActionRequirement, AnimationRequest, Attack, CancelRule,
    ContinuationRequirement, FlashRequest, ResourceType,
};

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect)]
pub enum ActionCategory {
    Dash,
    Jump,
    Throw,
    Other,
    NeutralNormal,
    CommandNormal,
    Special,
    Super,
    FollowUp,
    Forced, // For throw recipients
}
impl ActionCategory {
    pub fn can_be_standard_cancelled_into(&self) -> bool {
        match self {
            ActionCategory::Dash
            | ActionCategory::Jump
            | ActionCategory::NeutralNormal
            | ActionCategory::CommandNormal
            | ActionCategory::Special
            | ActionCategory::Super => true,

            ActionCategory::Other   // For stuff like parry
            | ActionCategory::Throw // For throws
            | ActionCategory::FollowUp  // These use a different cancel mechanism
            | ActionCategory::Forced => false,  // These are forced actions like parry flash and throw recipient
        }
    }
}

#[derive(Clone)]
pub struct Action {
    pub input: Option<&'static str>,
    pub category: ActionCategory,
    pub script: Vec<ActionBlock>,
    pub requirements: Vec<ActionRequirement>,
}
impl Action {
    pub fn new(
        input: Option<&'static str>,
        category: ActionCategory,
        script: Vec<ActionBlock>,
        requirements: Vec<ActionRequirement>,
    ) -> Self {
        Self {
            input,
            category,
            script,
            requirements,
        }
    }

    pub fn grounded(
        input: Option<&'static str>,
        category: ActionCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(input, category, script, vec![ActionRequirement::Grounded])
    }

    pub fn airborne(
        input: Option<&'static str>,
        category: ActionCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(input, category, script, vec![ActionRequirement::Airborne])
    }

    pub fn throw_hit(animation: impl Into<Animation>, duration: usize) -> Self {
        Action::new(
            None,
            ActionCategory::Forced,
            vec![ActionBlock {
                events: vec![
                    animation.into().into(),
                    // TODO: This causes a bug for throwers. Air throw needs lock
                    ActionEvent::Lock((duration, false)),
                ],
                exit_requirement: ContinuationRequirement::Time(duration),
                ..default()
            }],
            vec![],
        )
    }

    pub fn throw_target(
        animation: impl Into<Animation>,
        duration: usize,
        sideswitch: bool,
        damage: i32,
        launch_impulse: Vec2,
    ) -> Self {
        Self::throw_target_with_split_duration(
            animation,
            duration,
            sideswitch,
            duration,
            damage,
            launch_impulse,
        )
    }

    pub fn throw_target_with_split_duration(
        animation: impl Into<Animation>,
        lock_duration: usize,
        lock_sideswitch: bool,
        animation_duration: usize,
        damage: i32,
        launch_impulse: Vec2,
    ) -> Self {
        Action::new(
            None,
            ActionCategory::Forced,
            vec![ActionBlock {
                events: vec![
                    ActionEvent::Animation(AnimationRequest {
                        animation: animation.into(),
                        invert: true,
                        ..default()
                    }),
                    ActionEvent::ModifyResource(ResourceType::Health, -damage),
                    if launch_impulse == Vec2::ZERO {
                        ActionEvent::Noop
                    } else {
                        ActionEvent::Launch {
                            impulse: launch_impulse,
                        }
                    },
                    ActionEvent::Flash(FlashRequest::hit_flash()),
                    ActionEvent::Hitstop,
                    ActionEvent::Lock((lock_duration, lock_sideswitch)),
                ],
                exit_requirement: ContinuationRequirement::Time(animation_duration),
                ..default()
            }],
            vec![],
        )
    }
    pub fn ground_normal(
        input: &'static str,
        animation: impl Into<Animation>,
        startup: usize,
        attack: Attack,
        recovery: usize,
    ) -> Self {
        Self::normal(
            vec![ActionRequirement::Grounded],
            input,
            animation,
            startup,
            attack,
            recovery,
        )
    }

    pub fn air_normal(
        input: &'static str,
        animation: impl Into<Animation>,
        startup: usize,
        attack: Attack,
        recovery: usize,
    ) -> Self {
        Self::normal(
            vec![ActionRequirement::Airborne],
            input,
            animation,
            startup,
            attack,
            recovery,
        )
    }

    pub fn normal(
        requirements: Vec<ActionRequirement>,
        input: &'static str,
        animation: impl Into<Animation>,
        startup: usize,
        attack: Attack,
        recovery: usize,
    ) -> Self {
        let category = if input.len() == 1 {
            ActionCategory::NeutralNormal
        } else {
            ActionCategory::CommandNormal
        };

        Action::new(
            Some(input),
            category.clone(),
            vec![
                ActionBlock {
                    events: vec![animation.into().into()],
                    exit_requirement: ContinuationRequirement::Time(startup),
                    ..default()
                },
                ActionBlock {
                    events: vec![attack.into()],
                    exit_requirement: ContinuationRequirement::Time(recovery),
                    cancel_policy: CancelRule::cancel_out_of(category),
                    mutator: None,
                },
            ],
            requirements,
        )
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::grounded(
            None,
            ActionCategory::NeutralNormal, // Not a huge fan of this
            vec![ActionBlock {
                events: vec![Animation::default().into()],
                exit_requirement: ContinuationRequirement::Time(100),
                cancel_policy: CancelRule::never(),
                mutator: None,
            }],
        )
    }
}

impl std::fmt::Debug for Action {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("input", &self.input)
            .field("cancel category", &self.category)
            .finish()
    }
}
