use bevy_inspector_egui::Inspectable;
use wag_core::MoveId;

use crate::{Action, Move};

use super::FlowControl;
#[derive(Debug, Default, Clone, Inspectable)]
pub struct MoveHistory {
    pub move_id: MoveId,
    #[inspectable(ignore)]
    pub move_data: Move,
    pub frame_skip: usize,
    pub started: usize,
    #[inspectable(ignore)]
    pub past: Vec<FlowControl>,
    #[inspectable(ignore)]
    pub unprocessed_events: Vec<Action>,
    pub has_hit: bool,
}

impl MoveHistory {
    fn next_phase(&self) -> Option<FlowControl> {
        self.move_data.phases.get(self.past.len()).cloned()
    }

    fn cancellable_since(&self) -> Option<usize> {
        self.past
            .iter()
            .chain(self.next_phase().iter())
            .filter_map(|fc| {
                if let &FlowControl::Wait(frames, cancellable) = fc {
                    Some((frames, cancellable))
                } else {
                    None
                }
            })
            .fold(
                (None, self.started),
                |(output, fc_start_frame), (frames, cancel_policy)| {
                    let next_start_frame = fc_start_frame + frames;
                    let updated_output = if cancel_policy.can_cancel(self.has_hit) {
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

    pub fn cancellable_into(&self, other_move: &Move) -> bool {
        // TODO: This only allows normal to special cancelling
        self.move_data.move_type < other_move.move_type
    }

    pub fn cancellable_into_since(&self, other_move: &Move) -> Option<usize> {
        if self.cancellable_into(other_move) {
            self.cancellable_since()
        } else {
            None
        }
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
    use bevy::prelude::*;
    use wag_core::Animation;

    #[test]
    fn sanity_check() {
        let history = MoveHistory {
            move_id: MoveId::TestMove,
            move_data: Move {
                phases: vec![Action::Animation(Animation::TPose).into()],
                ..default()
            },
            ..default()
        };

        assert!(!history.is_done());
        assert!(!history.has_hit);
        assert!(history.next_phase() == Some(Action::Animation(Animation::TPose).into()));
        assert!(history.cancellable_since() == None)
    }

    #[test]
    fn basic_cancellability() {
        let started = 69;
        let duration = 10;
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            FlowControl::Wait(duration, CancelPolicy::Always),
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
        assert!(history.cancellable_since() == Some(started));
    }
}
