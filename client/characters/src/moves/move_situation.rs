use input_parsing::InputParser;

use crate::{Inventory, MoveHistory, Resources};

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
