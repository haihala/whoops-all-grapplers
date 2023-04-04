use std::collections::{HashMap, HashSet};

use crate::{
    helper_types::{Diff, Frame},
    input_stream::InputStream,
    motion_input::MotionInput,
};

use bevy::prelude::*;

use wag_core::{Facing, GameButton, MoveId, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
#[derive(Debug, Default, Component)]
pub struct InputParser {
    events: Vec<MoveId>,

    moves: HashMap<&'static str, Vec<MoveId>>,
    inputs: HashMap<&'static str, MotionInput>,
    head: Frame,
    ready: bool,
}
impl InputParser {
    pub(crate) fn new(new_inputs: HashMap<MoveId, &'static str>) -> Self {
        let mut moves: HashMap<&'static str, Vec<MoveId>> = HashMap::new();
        let mut inputs: HashMap<&'static str, MotionInput> = HashMap::new();

        for (move_id, input_str) in new_inputs.into_iter() {
            let input = input_str.into();
            inputs.insert(input_str, input);

            if let Some(ids) = moves.get_mut(input_str) {
                ids.push(move_id);
            } else {
                moves.insert(input_str, vec![move_id]);
            }
        }

        Self {
            moves,
            inputs,
            ..default()
        }
    }

    pub fn get_pressed(&self) -> HashSet<GameButton> {
        self.head.pressed.clone()
    }

    pub fn get_relative_stick_position(&self) -> StickPosition {
        // Because the parser is never aware of side, it will always think the player is looking to the right
        self.head.stick_position
    }

    pub fn get_events(&self) -> Vec<MoveId> {
        self.events.clone()
    }

    pub fn head_is_clear(&self) -> bool {
        self.head.stick_position == StickPosition::Neutral && self.head.pressed.is_empty()
    }

    fn flip(&mut self) {
        let diff = Diff {
            stick_move: Some(self.head.stick_position.mirror()),
            ..default()
        };

        self.add_frame(diff);
    }

    fn add_frame(&mut self, diff: Diff) {
        // This needs to happen before relative_stick is set to enable inputs that permit holding a direction as the first requirement
        self.parse_inputs(diff.clone(), self.head.clone());
        self.head.apply(diff);
    }

    fn parse_inputs(&mut self, diff: Diff, old_head: Frame) {
        let completed_inputs = self.inputs.iter_mut().filter_map(|(input_str, input)| {
            input.advance(&diff, old_head.clone());
            if input.is_done() {
                input.clear();
                return Some(*input_str);
            }
            None
        });

        let new_events = completed_inputs.flat_map(|input_str| self.moves[input_str].clone());

        self.events.extend(new_events);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

pub fn parse_input<T: InputStream + Component>(
    mut characters: Query<(&mut InputParser, &mut T, &Facing)>,
) {
    for (mut parser, mut reader, facing) in &mut characters {
        parser.ready = reader.is_ready();
        if let Some(diff) = reader.read() {
            parser.add_frame(if facing.to_flipped() {
                // Flip the inputs
                diff.mirrored()
            } else {
                diff
            });
        }
    }
}

// Since the parser doesn't get events if the inputs don't change, it's good to give a pseudo event when sides change
pub fn flip_parsers_on_side_change(mut parsers: Query<&mut InputParser, Changed<Facing>>) {
    for mut parser in &mut parsers {
        parser.flip();
    }
}

#[cfg(test)]
mod test {
    use bevy::ecs::schedule::ScheduleLabel;
    use std::thread::sleep;
    use std::time::Duration;
    use wag_core::GameButton;

    use crate::{
        helper_types::InputEvent,
        testing::{TestInputBundle, TestStream},
        MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS,
    };

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

        interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

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

        interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn slow_command_motion_recognized() {
        let mut interface = TestInterface::with_input("28");

        interface.add_stick_and_tick(StickPosition::S);
        interface.assert_no_events();

        interface.sleep(MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS);

        interface.add_stick_and_tick(StickPosition::N);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized() {
        let mut interface = TestInterface::with_input("[fs]");

        interface.add_button_and_tick(GameButton::Fast);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Strong);
        interface.assert_test_event_is_present();
    }

    #[test]
    fn multibutton_normal_recognized_despite_order() {
        let mut interface = TestInterface::with_input("[fs]");

        interface.add_button_and_tick(GameButton::Strong);
        interface.assert_no_events();
        interface.add_button_and_tick(GameButton::Fast);
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

    #[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
    struct TestSchedule;

    struct TestInterface {
        app: App,
    }
    impl TestInterface {
        fn with_input(input: &'static str) -> TestInterface {
            TestInterface::new(vec![(MoveId::TestMove, input)])
        }

        fn with_inputs(input: &'static str, second_input: &'static str) -> TestInterface {
            TestInterface::new(vec![
                (MoveId::TestMove, input),
                (MoveId::SecondTestMove, second_input),
            ])
        }

        fn new(moves: Vec<(MoveId, &'static str)>) -> TestInterface {
            let mut app = App::new();
            app.add_system(parse_input::<TestStream>);

            app.world.spawn((
                TestInputBundle::new(moves.into_iter().collect()),
                Facing::Right,
            ));

            let mut tester = TestInterface { app };
            tester.tick();

            tester
        }

        fn tick(&mut self) {
            self.app.update();
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
                .world
                .query::<&mut TestStream>()
                .iter_mut(&mut self.app.world)
            {
                reader.push(change.clone());
            }
        }

        fn sleep(&mut self, seconds: f32) {
            sleep(Duration::from_secs_f32(seconds + 0.1));
            self.tick();
        }

        fn assert_test_event_is_present(&mut self) {
            self.assert_event_is_present(MoveId::TestMove);
        }

        fn assert_both_test_events_are_present(&mut self) {
            self.assert_event_is_present(MoveId::TestMove);
            self.assert_event_is_present(MoveId::SecondTestMove);
        }

        fn assert_event_is_present(&mut self, id: MoveId) {
            let events = self.get_parser_events();
            assert!(events.contains(&id), "Event {:?} was not present", &id);
        }

        fn assert_no_events(&mut self) {
            let events = self.get_parser_events();
            assert!(events.is_empty(), "Expected no events, found {:?}", events,);
        }

        // Running a query requires mutable access I guess?
        fn get_parser_events(&mut self) -> Vec<MoveId> {
            self.app
                .world
                .query::<&InputParser>()
                .iter(&self.app.world)
                .next()
                .unwrap()
                .events
                .clone()
        }
    }
}
