use std::collections::VecDeque;

use crate::{Action, ActionBlock, CancelPolicy, Requirement};

#[derive(Debug, Clone, Default)]
pub struct ActionTracker {
    pub has_hit: bool,
    pub blocker: Requirement,
    pub cancel_policy: CancelPolicy,
    pub upcoming_blocks: VecDeque<ActionBlock>,
    pub start_frame: usize,
    pub current_block_start_frame: usize,
    cancel_breakpoints: Vec<(CancelPolicy, usize)>,
}
impl ActionTracker {
    /// Assumes that the actions from the first block have been processed
    /// It's easier to type that way.
    pub fn new(action: Action, start_frame: usize) -> Self {
        let first_action = action.script[0].clone();

        Self {
            has_hit: false,
            blocker: first_action.exit_requirement,
            cancel_policy: first_action.cancel_policy.clone(),
            upcoming_blocks: action.script.into_iter().skip(1).collect(),
            start_frame,
            current_block_start_frame: start_frame,
            cancel_breakpoints: vec![(first_action.cancel_policy, start_frame)],
        }
    }

    pub fn pop_next(&mut self, frame: usize) -> Option<ActionBlock> {
        if let Some(next) = self.upcoming_blocks.pop_front() {
            self.cancel_breakpoints
                .push((next.cancel_policy.clone(), frame));
            self.current_block_start_frame = frame;

            return Some(next);
        }
        None
    }

    pub fn cancellable_into_since(&self, action: Action) -> Option<usize> {
        let mut output = None;

        for (policy, frame) in self.cancel_breakpoints.iter() {
            let can_cancel = policy.can_cancel(self.has_hit, action.cancel_category);
            let was_cancellable = output.is_some();

            if can_cancel && !was_cancellable {
                // Mark as cancellable since that window started
                output = Some(*frame);
            } else if !can_cancel && was_cancellable {
                output = None;
            }
        }

        output
    }

    pub fn last_breakpoint_frame(&self) -> Option<usize> {
        self.cancel_breakpoints.last().map(|(_, frame)| *frame)
    }
}

#[cfg(test)]
mod test_cancellable_into_since {
    use crate::actions::CancelCategory;

    use super::*;
    use bevy::prelude::*;

    #[test]
    fn sanity_check() {
        let mut tracker = ActionTracker::new(
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelPolicy::never(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::any(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::Normal,
                ..default()
            }),
            None
        );

        tracker.pop_next(12);

        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::Normal,
                ..default()
            }),
            Some(12)
        );
    }

    #[test]
    fn sequential_windows() {
        let mut tracker = ActionTracker::new(
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelPolicy::any(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::any(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        tracker.pop_next(10);
        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::Normal,
                ..default()
            }),
            Some(0)
        );
    }

    #[test]
    fn closed_window() {
        let basic_normal = Action {
            cancel_category: CancelCategory::Normal,
            ..default()
        };

        let mut tracker = ActionTracker::new(
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelPolicy::never(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::any(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::never(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        assert_eq!(tracker.cancellable_into_since(basic_normal.clone()), None);

        tracker.pop_next(12);

        assert_eq!(
            tracker.cancellable_into_since(basic_normal.clone()),
            Some(12)
        );

        tracker.pop_next(20);

        assert_eq!(tracker.cancellable_into_since(basic_normal), None);
    }

    #[test]
    fn depends_on_action() {
        let mut tracker = ActionTracker::new(
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelPolicy::command_normal_recovery(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelPolicy::any(),
                        exit_requirement: Requirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        tracker.has_hit = true;
        tracker.pop_next(10);
        tracker.pop_next(20);

        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::Special,
                ..default()
            }),
            Some(0)
        );

        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::CommandNormal,
                ..default()
            }),
            Some(10)
        );

        assert_eq!(
            tracker.cancellable_into_since(Action {
                cancel_category: CancelCategory::Normal,
                ..default()
            }),
            Some(20)
        );
    }
}
