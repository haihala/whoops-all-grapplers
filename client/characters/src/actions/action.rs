use bevy::prelude::*;
use wag_core::Animation;

use crate::{
    ActionBlock, ActionRequirement, Attack, CancelCategory, CancelRule, ContinuationRequirement,
};

#[derive(Clone)]
pub struct Action {
    pub input: Option<&'static str>,
    pub cancel_category: CancelCategory,
    pub script: Vec<ActionBlock>,
    pub requirements: Vec<ActionRequirement>,
}
impl Action {
    pub fn new(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
        requirements: Vec<ActionRequirement>,
    ) -> Self {
        Self {
            input,
            cancel_category,
            script,
            requirements,
        }
    }

    pub fn grounded(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(
            input,
            cancel_category,
            script,
            vec![ActionRequirement::Grounded],
        )
    }

    pub fn airborne(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(
            input,
            cancel_category,
            script,
            vec![ActionRequirement::Airborne],
        )
    }

    pub fn throw_hit(animation: impl Into<Animation>, duration: usize) -> Self {
        Action::new(
            None,
            CancelCategory::Uncancellable,
            vec![ActionBlock {
                events: vec![animation.into().into()],
                exit_requirement: ContinuationRequirement::Time(duration),
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
        let cancel_type = if input.len() == 1 {
            CancelCategory::Normal
        } else {
            CancelCategory::CommandNormal
        };

        Action::new(
            Some(input),
            cancel_type.clone(),
            vec![
                ActionBlock {
                    events: vec![animation.into().into()],
                    exit_requirement: ContinuationRequirement::Time(startup),
                    ..default()
                },
                ActionBlock {
                    events: vec![attack.into()],
                    exit_requirement: ContinuationRequirement::Time(recovery),
                    cancel_policy: CancelRule::cancel_out_of(cancel_type),
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
            CancelCategory::Any,
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
            .field("cancel category", &self.cancel_category)
            .finish()
    }
}
