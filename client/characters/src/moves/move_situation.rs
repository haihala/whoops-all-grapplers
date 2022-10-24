use std::vec;

use input_parsing::InputParser;
use wag_core::StatusCondition;

use crate::{Action, Inventory, MoveHistory, Resources};

use super::FlowControl;

#[derive(Debug, Clone)]
pub struct Situation<'a> {
    pub inventory: &'a Inventory,
    pub history: Option<MoveHistory>,
    pub grounded: bool,
    pub resources: &'a Resources,
    pub parser: &'a InputParser,
    pub conditions: &'a Vec<StatusCondition>,
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
                if let Some(mut new_event) =
                    self.handle_flow_control(future_event.to_owned(), unused_time)
                {
                    match new_event {
                        FlowControl::Wait(time, _) => {
                            unused_time -= time;
                        }
                        FlowControl::Action(Action::Animation(animation)) => {
                            if history.frame_skip != 0 {
                                // This'll make it so that if a move is fast-forwarded due to frame fitting, the animation will be in sync
                                new_event =
                                    Action::AnimationAtFrame(animation, history.frame_skip).into();
                            }
                        }
                        FlowControl::Noop => {
                            continue; // So that noops don't end up in the handled_events list
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
            FlowControl::Action(action) => Some(FlowControl::Action(action)),
            FlowControl::Dynamic(fun) => self.handle_flow_control(fun(self.clone()), unused_time),
            FlowControl::Noop => Some(FlowControl::Noop),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Action, Move, OnHitEffect, ToHit};

    use super::*;
    use bevy::prelude::*;
    use wag_core::{Animation, MoveId};

    struct SituationWrapper {
        inventory: Inventory,
        history: Option<MoveHistory>,
        grounded: bool,
        resources: Resources,
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
                resources: &self.resources,
                parser: &self.parser,
                current_frame: self.current_frame,
                conditions: &self.conditions,
            }
            .new_actions()
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
            Action::Attack(ToHit::default(), OnHitEffect::default()).into(),
        ];
        let sw = SituationWrapper::with_phases(phases.clone());

        sw.assert_actions(&phases);
        sw.assert_done();
    }

    #[test]
    fn wait_gate() {
        let phases = vec![
            Action::Animation(Animation::TPose).into(),
            Action::Attack(ToHit::default(), OnHitEffect::default()).into(),
            FlowControl::Wait(10, false),
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
            Action::Attack(ToHit::default(), OnHitEffect::default()).into(),
            FlowControl::Wait(10, false),
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
        let phases = vec![FlowControl::Dynamic(|situation: Situation| {
            if situation.history.unwrap().has_hit {
                Action::Animation(Animation::TPose).into()
            } else {
                Action::Attack(ToHit::default(), OnHitEffect::default()).into()
            }
        })];

        let mut sw = SituationWrapper::with_phases(phases);

        sw.assert_actions(&[Action::Attack(ToHit::default(), OnHitEffect::default()).into()]);

        sw.register_hit();
        sw.assert_actions(&[Action::Animation(Animation::TPose).into()]);
    }
}
