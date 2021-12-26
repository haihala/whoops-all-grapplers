use crate::helper_types::{Diff, Frame};
use crate::input_reader::InputReader;
use crate::motion_input::MotionInput;

use bevy::utils::Instant;
use bevy::{prelude::*, utils::HashMap};

use player_state::PlayerState;
use types::{MoveType, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
pub struct InputParser {
    events: HashMap<MoveType, Instant>,

    registered_inputs: HashMap<MoveType, MotionInput>,
    head: Frame,
    relative_stick: StickPosition,
}

impl Default for InputParser {
    fn default() -> Self {
        Self {
            events: Default::default(),
            registered_inputs: Default::default(),
            head: Default::default(),
            relative_stick: Default::default(),
        }
    }
}
impl InputParser {
    pub fn load(inputs: HashMap<MoveType, &str>) -> Self {
        Self {
            registered_inputs: inputs
                .into_iter()
                .map(|(id, definition)| (id, definition.into()))
                .collect(),
            ..Default::default()
        }
    }

    pub fn register_input(&mut self, id: MoveType, input: MotionInput) {
        self.registered_inputs.insert(id, input);
    }

    pub fn get_absolute_stick_position(&self) -> StickPosition {
        self.head.stick_position
    }

    pub fn get_relative_stick_position(&self) -> StickPosition {
        self.relative_stick
    }

    pub fn get_events(&self) -> Vec<MoveType> {
        self.events.clone().into_iter().map(|(id, _)| id).collect()
    }

    pub fn consume_event(&mut self, event: &MoveType) {
        self.events.remove(event);
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
        self.events.retain(|_, timestamp| {
            timestamp.elapsed().as_secs_f32() < constants::EVENT_REPEAT_PERIOD
        })
    }

    #[cfg(test)]
    fn with_input(id: MoveType, input: &str) -> InputParser {
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

    use moves::test::TEST_MOVE;
    use player_state::PlayerState;
    use types::GameButton;

    use crate::helper_types::{ButtonUpdate, InputChange};

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

        interface.sleep(constants::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_no_events();
    }

    #[test]
    fn sonic_boom_recognized() {
        let mut interface = TestInterface::with_input("c46f");

        interface.add_stick_and_tick(StickPosition::W);
        interface.sleep(constants::CHARGE_TIME);

        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn sonic_boom_needs_charge() {
        let mut interface = TestInterface::with_input("c46f");

        interface.add_stick_and_tick(StickPosition::W);
        interface.tick();

        interface.add_stick_and_tick(StickPosition::E);
        interface.assert_no_events();

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_no_events();
    }

    #[test]
    fn normal_recognized_and_events_repeat_and_clear() {
        let mut interface = TestInterface::with_input("f");

        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();

        // Check that the event is still in (repeat works)
        interface.multi_tick(3);
        interface.assert_test_event_is_present();

        // Wait for the event to leave the buffer
        interface.sleep(constants::EVENT_REPEAT_PERIOD);
        interface.assert_no_events();
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

        interface.sleep(constants::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized() {
        let mut interface = TestInterface::with_input("[fh]");

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Heavy);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized_despite_order() {
        let mut interface = TestInterface::with_input("[fh]");

        interface.add_button_and_tick(GameButton::Heavy);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    struct TestInterface {
        world: World,
        stage: SystemStage,
    }
    impl TestInterface {
        fn with_input(input: &str) -> TestInterface {
            let mut world = World::default();

            let mut stage = SystemStage::parallel();
            stage.add_system(parse_input.system());

            world
                .spawn()
                .insert(InputParser::with_input(TEST_MOVE, input))
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

        fn assert_event_is_present(&mut self, id: MoveType) {
            for r in self.world.query::<&InputParser>().iter(&self.world) {
                assert_eq!(
                    r.events.len(),
                    1,
                    "Expected one event, found {}",
                    r.events.len()
                );

                for (event, _) in r.events.iter() {
                    assert_eq!(event, &id, "Expected id '{}', found '{}'", id, event);
                }
            }
        }

        fn assert_test_event_is_present(&mut self) {
            self.assert_event_is_present(TEST_MOVE)
        }

        fn assert_no_events(&mut self) {
            for r in self.world.query::<&InputParser>().iter(&self.world) {
                assert_eq!(
                    r.events.len(),
                    0,
                    "Expected no events, found {:?}",
                    r.events
                );
            }
        }
    }
}
