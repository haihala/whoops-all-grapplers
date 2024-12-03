use crate::{motion_input::MotionInput, ParrotStream};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use wag_core::{ActionId, Clock, Facing, GameButton, InputEvent, InputState, StickPosition};

#[derive(Debug, Component, Clone, Reflect)]
pub struct InputHistory {
    pub event: InputEvent,
    pub state: InputState,
    pub(crate) facing: Facing,
    pub(crate) frame: usize,
}

// This is only for shortening tests
impl Default for InputHistory {
    fn default() -> Self {
        Self {
            event: InputEvent::Point(StickPosition::Neutral),
            state: InputState::default(),
            facing: Facing::default(),
            frame: 0,
        }
    }
}
impl InputHistory {
    pub(crate) fn handle_facing(&self, absolute: bool) -> (InputEvent, InputState) {
        if !absolute && self.facing.to_flipped() {
            // Relative, the usual case
            (
                match self.event {
                    InputEvent::Point(sp) => InputEvent::Point(sp.mirror()),
                    other => other,
                },
                InputState {
                    stick_position: self.state.stick_position.mirror(),
                    ..self.state.clone()
                },
            )
        } else {
            (self.event, self.state.clone())
        }
    }
}

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
#[derive(Debug, Default, Component, Clone, Reflect)]
pub struct InputParser {
    events: Vec<ActionId>,
    inputs: Vec<(MotionInput, Vec<ActionId>)>,
    history: Vec<InputHistory>,
    state: InputState,
}

impl InputParser {
    pub(crate) fn new(new_inputs: HashMap<ActionId, String>) -> Self {
        let motions: Vec<MotionInput> = new_inputs
            .iter()
            .map(|(_, input_str)| MotionInput::from(input_str.clone()))
            .collect();

        let mut inputs = vec![];

        for motion in motions {
            // Remove duplicates
            if inputs.iter().any(|(input, _)| input == &motion) {
                continue;
            }

            let actions = new_inputs
                .iter()
                .filter_map(|(action_id, input_str)| {
                    if motion == input_str.clone().into() {
                        Some(*action_id)
                    } else {
                        None
                    }
                })
                .collect();

            inputs.push((motion, actions));
        }

        Self {
            inputs,
            ..default()
        }
    }

    pub fn get_complexity(&self, action: ActionId) -> usize {
        for (input, actions) in &self.inputs {
            if actions.contains(&action) {
                return input.complexity();
            }
        }
        panic!("Could not find input");
    }

    pub fn get_stick_pos(&self) -> StickPosition {
        self.state.stick_position
    }

    pub fn get_pressed(&self) -> HashSet<GameButton> {
        self.state.pressed.clone()
    }

    pub fn get_events(&self) -> Vec<ActionId> {
        self.events.clone()
    }

    pub fn head_is_clear(&self) -> bool {
        self.state.stick_position == StickPosition::Neutral && self.state.pressed.is_empty()
    }

    fn input_change(&mut self, events: Vec<InputEvent>, facing: Facing, frame: usize) {
        let mut new_history = vec![];
        for event in events.into_iter() {
            new_history.push(InputHistory {
                event,
                state: self.state.clone(),
                facing,
                frame,
            });

            // State gotta get updated in order
            self.state.apply(event);
        }

        // First element in history is the latest
        new_history.reverse();

        for (input, actions) in &self.inputs {
            let past: Vec<InputHistory> = self
                .history
                .iter()
                .take_while(|hist| hist.frame + input.buffer_window_size() >= frame)
                .cloned()
                .collect();

            let already_complete = input.contained_in(&past);
            if already_complete {
                continue;
            }

            let present: Vec<_> = new_history
                .clone()
                .into_iter()
                .chain(past.clone())
                .collect();
            let complete_with_new_input = input.contained_in(&present);
            if complete_with_new_input {
                let mut evs = actions.clone();
                self.events.append(&mut evs);
            }
        }

        self.history = new_history
            .into_iter()
            .chain(self.history.clone())
            .collect();
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub fn parse_input(
    mut characters: Query<(&mut InputParser, &ParrotStream, &Facing)>,
    clock: Res<Clock>,
) {
    for (mut parser, reader, facing) in &mut characters {
        let evs = reader.next_read.clone();
        if !evs.is_empty() {
            parser.input_change(evs, *facing, clock.frame);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PadBundle;

    use super::*;

    #[test]
    fn hadouken_recognized() {
        let mut interface = TestInterface::with_input("236f");

        interface.add_stick_and_tick(StickPosition::S);
        interface.add_stick_and_tick(StickPosition::SE);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn inputs_expire() {
        let mut interface = TestInterface::with_input("236f");

        interface.add_stick_and_tick(StickPosition::S);
        interface.add_stick_and_tick(StickPosition::SE);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.sleep(100);

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_no_events();
    }

    #[test]
    fn normal_recognized() {
        let mut interface = TestInterface::with_input("f");

        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn command_normal_recognized() {
        let mut interface = TestInterface::with_input("2f");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn slow_command_normal_recognized() {
        let mut interface = TestInterface::with_input("2f");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();

        // TODO: New system handles time differently
        // interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn slow_command_motion_recognized() {
        let mut interface = TestInterface::with_input("28");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();

        // TODO: New system handles time differently
        // interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_stick_and_tick(StickPosition::N);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multidirection_permits_skipping_first() {
        let mut interface = TestInterface::with_input("[41]6f");

        interface.add_stick_and_tick(StickPosition::SW);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multidirection_permits_skipping_second() {
        let mut interface = TestInterface::with_input("[41]6f");

        interface.add_stick_and_tick(StickPosition::W);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multiple_events() {
        let mut interface = TestInterface::with_inputs("2f", "f");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_both_test_events_are_present();
    }

    struct TestInterface {
        app: App,
    }
    impl TestInterface {
        fn with_input(input: &'static str) -> TestInterface {
            TestInterface::new(vec![(ActionId::TestMove, input)])
        }

        fn with_inputs(input: &'static str, second_input: &'static str) -> TestInterface {
            TestInterface::new(vec![
                (ActionId::TestMove, input),
                (ActionId::SecondTestMove, second_input),
            ])
        }

        fn new(moves: Vec<(ActionId, &'static str)>) -> TestInterface {
            let mut app = App::new();
            app.add_systems(Update, parse_input);

            app.world_mut().spawn((
                PadBundle::without_generic_inputs(
                    moves
                        .into_iter()
                        .map(|(id, dsl)| (id, dsl.to_string()))
                        .collect(),
                ),
                Facing::Right,
            ));

            app.init_resource::<Time>();
            app.init_resource::<Clock>();

            let mut tester = TestInterface { app };
            tester.tick();

            tester
        }

        fn tick(&mut self) {
            self.app
                .world_mut()
                .get_resource_mut::<Clock>()
                .unwrap()
                .frame += 1;
            self.app.update();
            for mut reader in self
                .app
                .world_mut()
                .query::<&mut ParrotStream>()
                .iter_mut(&mut self.app.world_mut())
            {
                reader.next_read.clear();
            }
        }

        fn add_button_and_tick(&mut self, button: GameButton) {
            self.add_input(InputEvent::Press(button));
            self.tick();
        }

        fn add_stick_and_tick(&mut self, stick: StickPosition) {
            self.add_input(InputEvent::Point(stick));
            self.tick();
        }

        fn add_input(&mut self, change: InputEvent) {
            for mut reader in self
                .app
                .world_mut()
                .query::<&mut ParrotStream>()
                .iter_mut(&mut self.app.world_mut())
            {
                reader.next_read.push(change.clone());
            }
        }

        fn assert_test_event_is_present(&mut self) {
            self.assert_event_is_present(ActionId::TestMove);
        }

        fn assert_both_test_events_are_present(&mut self) {
            self.assert_event_is_present(ActionId::TestMove);
            self.assert_event_is_present(ActionId::SecondTestMove);
        }

        fn assert_event_is_present(&mut self, id: ActionId) {
            let events = self.get_parser_events();
            assert!(events.contains(&id), "Event {:?} was not present", &id);
        }

        fn assert_no_events(&mut self) {
            let events = self.get_parser_events();
            assert!(events.is_empty(), "Expected no events, found {:?}", events,);
        }

        // Running a query requires mutable access I guess?
        fn get_parser_events(&mut self) -> Vec<ActionId> {
            self.app
                .world_mut()
                .query::<&InputParser>()
                .iter(&self.app.world())
                .next()
                .unwrap()
                .events
                .clone()
        }

        fn sleep(&mut self, ticks: i32) {
            for _ in 0..ticks {
                self.tick();
            }
        }
    }
}
