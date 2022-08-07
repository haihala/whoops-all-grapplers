use bevy_inspector_egui::Inspectable;
use types::MoveId;

use crate::{Action, Move};

use super::FlowControl;
#[derive(Debug, Default, Clone, Inspectable)]
pub struct MoveHistory {
    pub move_id: MoveId,
    #[inspectable(ignore)]
    pub move_data: Move,
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
        // TODO: This could probably use some unit tests
        let mut frame = 0;
        let mut output = None;
        for fc in self.past.iter() {
            if let FlowControl::Wait(frames, cancellable) = *fc {
                if cancellable {
                    if output.is_none() {
                        output = Some(frame);
                    }
                    // Importantly do nothing if output is some and this phase is cancellable
                } else {
                    output = None;
                }
                frame += frames;
            }
        }

        if let Some(FlowControl::Wait(_, cancellable)) = self.next_phase() {
            if cancellable {
                if output.is_none() {
                    output = Some(frame);
                }
                // Importantly do nothing if output is some and this phase is cancellable
            } else {
                output = None;
            }
        }
        output
    }

    pub fn cancellable_into_since(&self, other_move: &Move) -> Option<usize> {
        if self.move_data.move_type < other_move.move_type {
            // TODO: This only allows normal to special cancelling
            self.cancellable_since()
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool {
        self.past.len() == self.move_data.phases.len()
    }
}
