use bevy::prelude::*;
use std::collections::VecDeque;
use wag_core::ActionId;

use crate::{Action, ActionBlock, CancelRule, ContinuationRequirement};

#[derive(Debug, Clone, Default, Reflect)]
pub struct ActionTracker {
    pub has_hit: bool,
    // Stores a function pointer in a variant and reflect(ignore) doesn't work on that for some reason.
    #[reflect(ignore)]
    pub blocker: ContinuationRequirement,
    pub cancel_policy: CancelRule,
    #[reflect(ignore)] // Recursive down there
    pub upcoming_blocks: VecDeque<ActionBlock>,
    pub start_frame: usize,
    pub current_block_start_frame: usize,
    pub action_id: ActionId,
    cancel_breakpoints: Vec<(CancelRule, usize)>,
}
impl ActionTracker {
    /// Assumes that the actions from the first block have been processed
    /// It's easier to type that way.
    pub fn new(action_id: ActionId, action: Action, start_frame: usize) -> Self {
        let first_action = action.script[0].clone();

        Self {
            has_hit: false,
            action_id,
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

    pub fn cancellable_into_since(&self, action_id: ActionId, action: Action) -> Option<usize> {
        let mut output = None;

        // TODO: Reverse iteration would make sense here
        for (policy, frame) in self.cancel_breakpoints.iter() {
            let can_cancel = policy.can_cancel(self.has_hit, action_id, action.category.clone());
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
    use crate::actions::ActionCategory;

    use super::*;

    #[test]
    fn sanity_check() {
        let mut tracker = ActionTracker::new(
            ActionId::TestMove,
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelRule::never(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelRule::any(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        assert_eq!(
            tracker.cancellable_into_since(
                ActionId::TestMove,
                Action {
                    category: ActionCategory::Normal,
                    ..default()
                }
            ),
            None
        );

        tracker.pop_next(12);

        assert_eq!(
            tracker.cancellable_into_since(
                ActionId::TestMove,
                Action {
                    category: ActionCategory::Normal,
                    ..default()
                }
            ),
            Some(12)
        );
    }

    #[test]
    fn sequential_windows() {
        let mut tracker = ActionTracker::new(
            ActionId::TestMove,
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelRule::any(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelRule::any(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        tracker.pop_next(10);
        assert_eq!(
            tracker.cancellable_into_since(
                ActionId::TestMove,
                Action {
                    category: ActionCategory::Normal,
                    ..default()
                }
            ),
            Some(0)
        );
    }

    #[test]
    fn closed_window() {
        let basic_normal = Action {
            category: ActionCategory::Normal,
            ..default()
        };

        let mut tracker = ActionTracker::new(
            ActionId::TestMove,
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelRule::never(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelRule::any(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelRule::never(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                ],
                ..default()
            },
            0,
        );

        assert_eq!(
            tracker.cancellable_into_since(ActionId::TestMove, basic_normal.clone()),
            None
        );

        tracker.pop_next(12);

        assert_eq!(
            tracker.cancellable_into_since(ActionId::TestMove, basic_normal.clone()),
            Some(12)
        );

        tracker.pop_next(20);

        assert_eq!(
            tracker.cancellable_into_since(ActionId::TestMove, basic_normal),
            None
        );
    }

    #[test]
    fn depends_on_action() {
        let mut tracker = ActionTracker::new(
            ActionId::TestMove,
            Action {
                script: vec![
                    ActionBlock {
                        cancel_policy: CancelRule::normal_recovery(),
                        exit_requirement: ContinuationRequirement::Time(10),
                        ..default()
                    },
                    ActionBlock {
                        cancel_policy: CancelRule::any(),
                        exit_requirement: ContinuationRequirement::Time(10),
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
            tracker.cancellable_into_since(
                ActionId::TestMove,
                Action {
                    category: ActionCategory::Special,
                    ..default()
                }
            ),
            Some(0)
        );

        assert_eq!(
            tracker.cancellable_into_since(
                ActionId::TestMove,
                Action {
                    category: ActionCategory::Normal,
                    ..default()
                }
            ),
            Some(10)
        );
    }
}
