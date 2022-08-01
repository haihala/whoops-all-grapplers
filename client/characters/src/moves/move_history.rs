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

#[derive(Debug, Default, Clone, Inspectable)]
pub struct MoveHistory {
    pub move_id: MoveId,
    #[inspectable(ignore)]
    pub move_data: Move,
    pub started: usize,
    pub cancellable_since: Option<usize>,
    #[inspectable(ignore)]
    pub past: Vec<FlowControl>,
    #[inspectable(ignore)]
    pub unprocessed_events: Vec<Action>,
    pub has_hit: bool,
}

impl Situation<'_> {
    pub fn new_actions(&self) -> Vec<FlowControl> {
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
            let mut collector = vec![];

            for future_event in history.move_data.phases.iter().skip(skip_to) {
                if let Some(new_event) =
                    Self::handle_flow_control(future_event.to_owned(), self.clone(), unused_time)
                {
                    if let FlowControl::Wait(time, _) = new_event {
                        unused_time -= time;
                    }
                    collector.push(new_event);
                } else {
                    // There is a time block
                    break;
                }
            }
            collector
        } else {
            panic!("Asking for actions but history is not set aka move is not set");
        }
    }

    fn handle_flow_control(
        future_event: FlowControl,
        situation: Situation,
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
            FlowControl::Dynamic(fun) => {
                Self::handle_flow_control(fun(situation.clone()), situation, unused_time)
            }
        }
    }
}
