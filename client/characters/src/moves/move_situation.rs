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

#[cfg(test)]
mod test {
    use crate::{Action, Move, SpawnDescriptor};

    use super::*;
    use bevy::prelude::*;
    use types::{Animation, MoveId};

    struct SituationWrapper {
        inventory: Inventory,
        history: Option<MoveHistory>,
        grounded: bool,
        resources: Resources,
        parser: InputParser,
        current_frame: usize,
    }
    impl Default for SituationWrapper {
        fn default() -> Self {
            Self {
                inventory: Default::default(),
                history: Default::default(),
                grounded: true,
                resources: Default::default(),
                parser: Default::default(),
                current_frame: 1, // So that history can start at 0 and all initial actions are drained
            }
        }
    }

    impl SituationWrapper {
        fn with_phases(phases: Vec<FlowControl>) -> Self {
            let move_data = Move {
                phases,
                ..default()
            };

            Self {
                history: Some(MoveHistory {
                    move_data,
                    move_id: MoveId::TestMove,
                    ..default()
                }),
                ..default()
            }
        }

        fn get_actions(&self) -> Vec<FlowControl> {
            Situation {
                inventory: &self.inventory,
                history: self.history.clone(),
                grounded: self.grounded,
                resources: &&self.resources,
                parser: &&self.parser,
                current_frame: self.current_frame,
            }
            .new_actions()
        }

        fn assert_actions(&self, actions: Vec<FlowControl>) {
            assert!(self.get_actions() == actions);
        }
    }

    #[test]
    fn sanity_check() {
        let phases = vec![];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(phases);
    }

    #[test]
    fn single_action() {
        let phases = vec![Action::Animation(Animation::TPose).into()];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(phases);
    }

    #[test]
    fn multiple_actions() {
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            Action::Hitbox(SpawnDescriptor::default()).into(),
        ];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(phases);
    }
}
