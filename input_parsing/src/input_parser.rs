use crate::{
    helper_types::{Diff, Frame},
    input_reader::InputReader,
    motion_input::MotionInput,
    EVENT_REPEAT_PERIOD,
};

use bevy::{
    prelude::*,
    utils::{HashMap, Instant},
};

use player_state::PlayerState;
use types::{MoveId, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
#[derive(Debug, Default)]
pub struct InputParser {
    events: HashMap<MoveId, Instant>,

    registered_inputs: HashMap<MoveId, MotionInput>,
    head: Frame,
    relative_stick: StickPosition,
}
impl InputParser {
    pub fn load(inputs: HashMap<MoveId, &str>) -> Self {
        Self {
            registered_inputs: inputs
                .into_iter()
                .map(|(id, definition)| (id, definition.into()))
                .collect(),
            ..Default::default()
        }
    }

    pub fn register_input(&mut self, id: MoveId, input: MotionInput) {
        self.registered_inputs.insert(id, input);
    }

    pub fn get_absolute_stick_position(&self) -> StickPosition {
        self.head.stick_position
    }

    pub fn get_relative_stick_position(&self) -> StickPosition {
        self.relative_stick
    }

    pub fn get_events(&self) -> Vec<MoveId> {
        self.events.clone().into_iter().map(|(id, _)| id).collect()
    }

    pub fn consume_event(&mut self, event: MoveId) {
        self.events.remove(&event);
    }

    fn add_frame(&mut self, diff: Diff, flipped: bool) {
        self.head.apply(diff.clone());

        let relative_diff = if flipped {
            self.relative_stick = self.head.stick_position.flip();
            diff.flip()
        } else {
            self.relative_stick = self.head.stick_position;
            diff
        };

        self.parse_inputs(&relative_diff);
    }

    fn parse_inputs(&mut self, diff: &Diff) {
        let now = Instant::now();
        let frame = &self.head;

        self.events
            .extend(self.registered_inputs.iter_mut().filter_map(|(id, input)| {
                input.advance(diff, frame);
                if input.is_done() {
                    input.clear();
                    return Some((*id, now));
                }
                None
            }));
    }

    fn purge_old_events(&mut self) {
        self.events
            .retain(|_, timestamp| timestamp.elapsed().as_secs_f32() < EVENT_REPEAT_PERIOD)
    }

    #[cfg(test)]
    fn with_input(id: MoveId, input: &str) -> InputParser {
        let mut parser = InputParser::default();
        parser.register_input(id, input.into());
        parser
    }
}

pub fn parse_input(mut characters: Query<(&mut InputParser, &mut InputReader, &PlayerState)>) {
    for (mut parser, mut reader, state) in characters.iter_mut() {
        if reader.readable() {
            parser.add_frame(reader.read().unwrap(), state.flipped());
        }
        parser.purge_old_events();
    }
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use moves::test::{SECOND_TEST_MOVE, TEST_MOVE};
    use player_state::PlayerState;
    use types::GameButton;

    use crate::{
        helper_types::{ButtonUpdate, InputChange},
        CHARGE_TIME, MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS,
    };

    use super::*;

    #[test]
    fn hadouken_recognized() {
        let mut interface = TestInterface::with_input("236l");

        interface.add_stick_and_tick(StickPosition::S);
        interface.add_stick_and_tick(StickPosition::SE);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn inputs_expire() {
        let mut interface = TestInterface::with_input("236l");

        interface.add_stick_and_tick(StickPosition::S);
        interface.add_stick_and_tick(StickPosition::SE);
        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_no_events();
    }

    #[test]
    fn sonic_boom_recognized() {
        let mut interface = TestInterface::with_input("c46l");

        interface.add_stick_and_tick(StickPosition::W);
        interface.sleep(CHARGE_TIME);

        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn sonic_boom_needs_charge() {
        let mut interface = TestInterface::with_input("c46l");

        interface.add_stick_and_tick(StickPosition::W);
        interface.tick();

        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_no_events();
    }

    #[test]
    fn normal_recognized_and_events_repeat_and_clear() {
        let mut interface = TestInterface::with_input("l");

        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();

        // Check that the event is still in (repeat works)
        interface.multi_tick(3);
        interface.assert_test_event_is_present();

        // Wait for the event to leave the buffer
        interface.sleep(EVENT_REPEAT_PERIOD);
        interface.assert_no_events();
    }

    #[test]
    fn command_normal_recognized() {
        let mut interface = TestInterface::with_input("2l");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn slow_command_normal_recognized() {
        let mut interface = TestInterface::with_input("2l");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();

        interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized() {
        let mut interface = TestInterface::with_input("[lh]");

        interface.add_button_and_tick(GameButton::Light);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Heavy);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized_despite_order() {
        let mut interface = TestInterface::with_input("[lh]");

        interface.add_button_and_tick(GameButton::Heavy);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Light);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multiple_events() {
        let mut interface = TestInterface::with_inputs("2l", "l");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Light);
        interface.assert_both_test_events_are_present();
    }

    struct TestInterface {
        world: World,
        stage: SystemStage,
    }
    impl TestInterface {
        fn with_input(input: &str) -> TestInterface {
            TestInterface::with_parser(InputParser::with_input(TEST_MOVE, input))
        }

        fn with_inputs(input: &str, second_input: &str) -> TestInterface {
            TestInterface::with_parser(InputParser::load(
                vec![(TEST_MOVE, input), (SECOND_TEST_MOVE, second_input)]
                    .into_iter()
                    .collect(),
            ))
        }

        fn with_parser(parser: InputParser) -> TestInterface {
            let mut world = World::default();

            let mut stage = SystemStage::parallel();
            stage.add_system(parse_input.system());

            world
                .spawn()
                .insert(parser)
                .insert(PlayerState::default())
                .insert(InputReader::with_pad(Gamepad(1)));

            let mut tester = TestInterface { world, stage };
            tester.tick();

            tester
        }

        fn tick(&mut self) {
            self.stage.run(&mut self.world);
        }

        fn multi_tick(&mut self, frames: usize) {
            for _ in 0..frames {
                self.tick();
            }
        }

        fn add_button_and_tick(&mut self, button: GameButton) {
            self.add_input(InputChange::Button(button, ButtonUpdate::Pressed));
            self.tick();
        }

        fn add_stick_and_tick(&mut self, stick: StickPosition) {
            self.add_input(InputChange::Stick(stick));
            self.tick();
        }

        fn add_input(&mut self, change: InputChange) {
            for mut reader in self
                .world
                .query::<&mut InputReader>()
                .iter_mut(&mut self.world)
            {
                reader.push(change);
            }
        }

        fn sleep(&mut self, seconds: f32) {
            sleep(Duration::from_secs_f32(seconds + 0.1));
            self.tick();
        }

        fn assert_test_event_is_present(&mut self) {
            self.assert_event_is_present(TEST_MOVE);
        }

        fn assert_both_test_events_are_present(&mut self) {
            self.assert_event_is_present(TEST_MOVE);
            self.assert_event_is_present(SECOND_TEST_MOVE);
        }

        fn assert_event_is_present(&mut self, id: MoveId) {
            let parser = self
                .world
                .query::<&InputParser>()
                .iter(&self.world)
                .next()
                .unwrap();

            assert!(parser.events.contains_key(&id));
        }

        fn assert_no_events(&mut self) {
            let parser = self
                .world
                .query::<&InputParser>()
                .iter(&self.world)
                .next()
                .unwrap();

            assert!(
                parser.events.is_empty(),
                "Expected no events, found {:?}",
                parser.events,
            );
        }
    }
}
