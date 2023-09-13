use std::vec;

use input_parsing::InputParser;
use wag_core::StatusCondition;

use crate::{Action, Inventory, MoveHistory, Properties};

use super::{CancelPolicy, FlowControl};

#[derive(Debug, Clone)]
pub struct Situation<'a> {
    pub inventory: &'a Inventory,
    pub history: Option<MoveHistory>,
    pub grounded: bool,
    pub properties: &'a Properties,
    pub parser: &'a InputParser,
    pub conditions: Vec<StatusCondition>,
    pub current_frame: usize,
}

impl Situation<'_> {
    pub fn new_fcs(&self) -> Vec<FlowControl> {
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
                if let Some(mut new_event) =
                    self.handle_flow_control(future_event.to_owned(), unused_time)
                {
                    match new_event {
                        FlowControl::Wait(time, _) => {
                            unused_time -= time;
                        }
                        FlowControl::Actions(actions) => {
                            // TODO: This could be cleaned up a ton
                            new_event = FlowControl::Actions(
                                actions
                                    .into_iter()
                                    .map(|action| {
                                        if history.frame_skip == 0 {
                                            action
                                        } else if let Action::Animation(animation) = action {
                                            Action::AnimationAtFrame(animation, history.frame_skip)
                                        } else if let Action::RecipientAnimation(animation) = action
                                        {
                                            Action::RecipientAnimationAtFrame(
                                                animation,
                                                history.frame_skip,
                                            )
                                        } else {
                                            action
                                        }
                                    })
                                    .collect(),
                            );
                        }
                        _ => {}
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
            FlowControl::DynamicActions(fun) => Some(FlowControl::Actions(fun(self.clone()))),
            FlowControl::WaitUntil(fun, timeout) => {
                let timed_out = timeout.map(|time| time <= unused_time).unwrap_or(false);
                if timed_out || fun(self.clone()) {
                    // We've hit a timeout or the wait condition has been met
                    // Store it in move history as a constant wait
                    Some(FlowControl::Wait(unused_time, CancelPolicy::never()))
                } else {
                    None
                }
            }
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{moves::Attack, Action, Move};

    use super::*;
    use bevy::prelude::*;
    use wag_core::{Animation, MoveId};

    struct SituationWrapper {
        inventory: Inventory,
        history: Option<MoveHistory>,
        grounded: bool,
        properties: Properties,
        parser: InputParser,
        conditions: Vec<StatusCondition>,
        current_frame: usize,
    }
    impl Default for SituationWrapper {
        fn default() -> Self {
            Self {
                inventory: Default::default(),
                history: Default::default(),
                grounded: true,
                conditions: vec![],
                properties: Properties::testing_default(),
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
                properties: &self.properties,
                parser: &self.parser,
                current_frame: self.current_frame,
                conditions: self.conditions.clone(),
            }
            .new_fcs()
        }

        fn assert_actions(&self, comparison: &[FlowControl]) {
            let actions = self.get_actions();
            assert!(actions == comparison, "{:?} != {:?}", actions, comparison);
        }

        fn set_time(&mut self, frame: usize) {
            self.current_frame = frame;
        }

        fn update_history(&mut self) {
            let mut history = self.history.clone().unwrap();
            history.past.extend(self.get_actions().into_iter());
            self.history = Some(history);
        }

        fn register_hit(&mut self) {
            let mut history = self.history.clone().unwrap();
            history.has_hit = true;
            self.history = Some(history);
        }

        fn assert_done(&self) {
            assert!(self.is_done());
        }

        fn assert_not_done(&self) {
            assert!(!self.is_done());
        }

        fn is_done(&self) -> bool {
            let mut history = self.history.clone().unwrap();
            history.past.extend(self.get_actions().into_iter());
            history.is_done()
        }
    }

    #[test]
    fn sanity_check() {
        let phases = vec![];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases);
        sw.assert_done();
    }

    #[test]
    fn single_action() {
        let phases = vec![Action::Animation(Animation::TPose).into()];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases);
        sw.assert_done();
    }

    #[test]
    fn multiple_actions() {
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            Attack::default().into(),
        ];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases);
        sw.assert_done();
    }

    #[test]
    fn wait_gate() {
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            Attack::default().into(),
            FlowControl::Wait(10, CancelPolicy::never()),
            Action::Animation(Animation::TPose).into(),
        ];
        let mut sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases[..2]);
        sw.assert_not_done();

        sw.set_time(10);
        sw.assert_actions(&phases);
        sw.assert_done();
    }

    #[test]
    fn wait_gate_partial() {
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            Attack::default().into(),
            FlowControl::Wait(10, CancelPolicy::never()),
            Action::Animation(Animation::TPose).into(),
        ];

        let mut sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases[..2]);
        sw.update_history();
        sw.assert_not_done();

        sw.set_time(10);
        sw.assert_actions(&phases[2..]);
        sw.assert_done();
    }

    #[test]
    fn dynamics() {
        let phases = vec![FlowControl::DynamicActions(|situation: Situation| {
            vec![if situation.history.unwrap().has_hit {
                Action::Animation(Animation::TPose)
            } else {
                Attack::default().into()
            }]
        })];

        let mut sw = SituationWrapper::with_phases(phases);

        sw.assert_actions(&[Attack::default().into()]);

        sw.register_hit();
        sw.assert_actions(&[Action::Animation(Animation::TPose).into()]);
    }
}
