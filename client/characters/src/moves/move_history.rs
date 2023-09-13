use bevy::prelude::*;
use wag_core::MoveId;

use crate::{Action, Move};

use super::{CancelCategory, FlowControl};

#[derive(Debug, Default, Clone, Reflect)]
pub struct MoveHistory {
    pub move_id: MoveId,
    #[reflect(ignore)]
    pub move_data: Move,
    pub frame_skip: usize,
    pub started: usize,
    #[reflect(ignore)]
    pub past: Vec<FlowControl>,
    #[reflect(ignore)]
    pub unprocessed_events: Vec<Action>,
    pub has_hit: bool,
}

impl MoveHistory {
    fn next_phase(&self) -> Option<FlowControl> {
        self.move_data.phases.get(self.past.len()).cloned()
    }

    fn cancellable_since(&self, target_category: CancelCategory) -> Option<usize> {
        self.past
            .iter()
            .chain(self.next_phase().iter())
            .filter_map(|fc| {
                if let FlowControl::Wait(frames, cancel_policy) = fc.clone() {
                    Some((frames, cancel_policy))
                } else {
                    None
                }
            })
            .fold(
                (None, self.started),
                |(output, fc_start_frame), (frames, cancel_policy)| {
                    let next_start_frame = fc_start_frame + frames;
                    let updated_output = if cancel_policy.can_cancel(self.has_hit, target_category)
                    {
                        if output.is_some() {
                            // Still cancellable, keep on trucking
                            output
                        } else {
                            // Cancellable just now, set since time to current frame
                            Some(fc_start_frame)
                        }
                    } else {
                        // Not cancellable, so cancellable since is None
                        None
                    };

                    (updated_output, next_start_frame)
                },
            )
            .0
    }

    pub fn cancellable_into_since(&self, other_move: &Move) -> Option<usize> {
        if let Some(FlowControl::Wait(_, rule)) = self.past.last() {
            if rule.can_cancel(self.has_hit, other_move.cancel_category) {
                return self.cancellable_since(other_move.cancel_category);
            }
        }

        None
    }

    pub fn is_done(&self) -> bool {
        self.past.len() == self.move_data.phases.len()
    }

    pub fn add_actions_from(&mut self, fcs: Vec<FlowControl>) {
        self.unprocessed_events
            .extend(fcs.into_iter().flat_map(|fc| {
                if let FlowControl::Actions(actions) = fc {
                    actions
                } else {
                    vec![]
                }
            }));
    }
}

#[cfg(test)]
mod test {
    use crate::moves::CancelPolicy;

    use super::*;
    use wag_core::Animation;

    #[test]
    fn sanity_check() {
        let history = MoveHistory {
            move_id: MoveId::TestMove,
            ..default()
        };

        assert!(!history.is_done());
        assert!(!history.has_hit);
        assert!(history.next_phase() == Some(Action::Animation(Animation::TPose).into()));
        assert!(history.cancellable_since(CancelCategory::Everything) == None)
    }

    #[test]
    fn basic_cancellability() {
        let started = 69;
        let duration = 10;
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            FlowControl::Wait(duration, CancelPolicy::any()),
        ];

        let history = MoveHistory {
            move_id: MoveId::TestMove,
            started,
            past: phases.clone(),
            move_data: Move {
                phases,
                ..default()
            },
            ..default()
        };

        assert!(history.is_done());
        assert!(history.cancellable_since(CancelCategory::Everything) == Some(started));
    }
}
