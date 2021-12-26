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
    pub fn load(inputs: HashMap<MoveType, &'static str>) -> Self {
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
    fn with_input(id: MoveType, input: &'static str) -> InputParser {
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
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "236f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);
        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    #[test]
    fn inputs_expire() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "236f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);
        assert_no_events(&mut world);

        sleep(Duration::from_secs_f32(
            constants::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS + 0.1,
        ));
        tick_frames(&mut world, &mut update_stage, 1);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_no_events(&mut world);
    }

    #[test]
    fn sonic_boom_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "c46f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::W);
        sleep(Duration::from_secs_f32(constants::CHARGE_TIME + 0.01));
        tick_frames(&mut world, &mut update_stage, 1);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);
        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    #[test]
    fn sonic_boom_needs_charge() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "c46f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::W);
        tick_frames(&mut world, &mut update_stage, 1);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);
        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_no_events(&mut world);
    }

    #[test]
    fn normal_recognized_and_events_repeat_and_clear() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "f"));

        assert_no_events(&mut world);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);

        tick_frames(&mut world, &mut update_stage, 3);
        // Check that the event is still in (repeat works)
        assert_event_is_present(&mut world, TEST_MOVE);

        // Wait for the event to leave the buffer
        sleep(Duration::from_secs_f32(
            constants::EVENT_REPEAT_PERIOD + 0.01,
        ));
        tick_frames(&mut world, &mut update_stage, 1);
        assert_no_events(&mut world);
    }

    #[test]
    fn command_normal_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "2f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        assert_no_events(&mut world);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    #[test]
    fn slow_command_normal_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "2f"));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        assert_no_events(&mut world);

        sleep(Duration::from_secs_f32(
            constants::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS + 0.1,
        ));
        tick_frames(&mut world, &mut update_stage, 1);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    #[test]
    fn multibutton_normal_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "[fh]"));

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_no_events(&mut world);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Heavy);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    #[test]
    fn multibutton_normal_recognized_despite_order() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_input(TEST_MOVE, "[fh]"));

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Heavy);
        assert_no_events(&mut world);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);
        assert_event_is_present(&mut world, TEST_MOVE);
    }

    fn test_setup(parser: InputParser) -> (World, SystemStage) {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(parse_input.system());

        world
            .spawn()
            .insert(parser)
            .insert(PlayerState::default())
            .insert(InputReader::with_pad(Gamepad(1)));

        // Initial tick
        update_stage.run(&mut world);

        (world, update_stage)
    }

    fn tick_frames(mut world: &mut World, update_stage: &mut SystemStage, frames: i32) {
        for _ in 0..frames {
            update_stage.run(&mut world);
        }
    }

    fn add_button_and_tick(
        mut world: &mut World,
        update_stage: &mut SystemStage,
        button: GameButton,
    ) {
        add_input(
            &mut world,
            InputChange::Button(button, ButtonUpdate::Pressed),
        );
        update_stage.run(&mut world);
    }

    fn add_stick_and_tick(
        mut world: &mut World,
        update_stage: &mut SystemStage,
        stick: StickPosition,
    ) {
        add_input(&mut world, InputChange::Stick(stick));
        update_stage.run(&mut world);
    }

    fn add_input(world: &mut World, change: InputChange) {
        for mut reader in world.query::<&mut InputReader>().iter_mut(world) {
            reader.push(change);
        }
    }

    fn assert_event_is_present(world: &mut World, id: MoveType) {
        for r in world.query::<&InputParser>().iter(&world) {
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

    fn assert_no_events(world: &mut World) {
        for r in world.query::<&InputParser>().iter(&world) {
            assert_eq!(
                r.events.len(),
                0,
                "Expected no events, found {:?}",
                r.events
            );
        }
    }
}
