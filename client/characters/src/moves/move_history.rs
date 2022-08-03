use bevy_inspector_egui::Inspectable;
use input_parsing::InputParser;
use types::MoveId;

use crate::{Action, Inventory, Move, Resources};

use super::FlowControl;

#[derive(Debug, Clone)]
pub struct Situation<'a> {
    pub inventory: &'a Inventory,
    pub history: Option<MoveHistory>,
    pub grounded: bool,
    pub resources: &'a Resources,
    pub parser: &'a InputParser,
    pub current_frame: usize,
}

impl Situation<'_> {
    pub fn new_actions(&self) -> Vec<FlowControl> {
        // TODO: This could probably use some unit tests
        if let Some(ref history) = self.history {
            let past_duration = history
                .past
                .iter()
                .map(|fc| {
                    if let FlowControl::Wait(time, _) = fc {
                        *time
                    } else {
                        0
                    }
                })
                .sum::<usize>();

            let mut unused_time = self.current_frame - history.started - past_duration;
            let skip_to = history.past.len();
            let mut handled_events = vec![];

            for future_event in history.move_data.phases.iter().skip(skip_to) {
                if let Some(new_event) =
                    self.handle_flow_control(future_event.to_owned(), unused_time)
                {
                    if let FlowControl::Wait(time, _) = new_event {
                        unused_time -= time;
                    }
                    handled_events.push(new_event);
                } else {
                    // There is a time block
                    break;
                }
            }
            handled_events
        } else {
            panic!("Asking for actions but history is not set aka move is not set");
        }
    }

    fn handle_flow_control(
        &self,
        future_event: FlowControl,
        unused_time: usize,
    ) -> Option<FlowControl> {
        match future_event {
            FlowControl::Wait(time, _) => {
                if time > unused_time {
                    // There is a time block
                    None
                } else {
                    Some(future_event)
                }
            }
            FlowControl::Action(action) => Some(FlowControl::Action(action)),
            FlowControl::Dynamic(fun) => self.handle_flow_control(fun(self.clone()), unused_time),
        }
    }
}
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
}
