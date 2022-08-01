use std::collections::{HashMap, HashSet};

use crate::{
    helper_types::{Diff, Frame},
    input_stream::InputStream,
    motion_input::MotionInput,
};

use bevy::prelude::*;

use types::{Facing, GameButton, MoveId, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
#[derive(Debug, Default, Component)]
pub struct InputParser {
    events: Vec<MoveId>,

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
            ..default()
        }
    }

    pub fn register_input(&mut self, id: MoveId, input: MotionInput) {
        self.registered_inputs.insert(id, input);
    }

    pub fn get_pressed(&self) -> HashSet<GameButton> {
        self.head.pressed.clone()
    }

    pub fn get_absolute_stick_position(&self) -> StickPosition {
        self.head.stick_position
    }

    pub fn get_relative_stick_position(&self) -> StickPosition {
        self.relative_stick
    }

    pub fn drain_events(&mut self) -> Vec<MoveId> {
        self.events.drain(..).collect()
    }

    pub fn head_is_clear(&self) -> bool {
        self.head.stick_position == StickPosition::Neutral && self.head.pressed.is_empty()
    }

    fn add_frame(&mut self, diff: Diff, facing: &Facing) {
        // This needs to happen before relative_stick is set to enable inputs that permit holding a direction as the first requirement
        self.parse_inputs(
            Diff {
                stick_move: diff.stick_move.map(|stick| facing.mirror_stick(stick)),
                ..diff.clone()
            },
            self.relative_stick,
        );

        self.head.apply(diff);
        self.relative_stick = facing.mirror_stick(self.head.stick_position);
    }

    fn parse_inputs(&mut self, diff: Diff, old_stick: StickPosition) {
        self.events
            .extend(self.registered_inputs.iter_mut().filter_map(|(id, input)| {
                input.advance(&diff, old_stick);
                if input.is_done() {
                    input.clear();
                    return Some(*id);
                }
                None
            }));
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub fn parse_input<T: InputStream + Component>(
    mut characters: Query<(&mut InputParser, &mut T, &Facing)>,
) {
    for (mut parser, mut reader, facing) in characters.iter_mut() {
        if let Some(diff) = reader.read() {
            parser.add_frame(diff, facing);
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use types::GameButton;

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
    fn multidirection_permits_skipping() {
        let mut interface = TestInterface::with_input("[41]6f");

        interface.add_stick_and_tick(StickPosition::SW);
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
        world: World,
        stage: SystemStage,
    }
    impl TestInterface {
        fn with_input(input: &str) -> TestInterface {
            TestInterface::new(vec![(MoveId::TestMove, input)])
        }

        fn with_inputs(input: &str, second_input: &str) -> TestInterface {
            TestInterface::new(vec![
                (MoveId::TestMove, input),
                (MoveId::SecondTestMove, second_input),
            ])
        }

        fn new(moves: Vec<(MoveId, &str)>) -> TestInterface {
            let mut world = World::default();

            let mut stage = SystemStage::parallel();
            stage.add_system(parse_input::<TestStream>);

            world
                .spawn()
                .insert_bundle(TestInputBundle::new(moves.into_iter().collect()))
                .insert(Facing::Right);

            let mut tester = TestInterface { world, stage };
            tester.tick();

            tester
        }

        fn tick(&mut self) {
            self.stage.run(&mut self.world);
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
                .world
                .query::<&mut TestStream>()
                .iter_mut(&mut self.world)
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
            let parser = self
                .world
                .query::<&InputParser>()
                .iter(&self.world)
                .next()
                .unwrap();

            assert!(
                parser.events.contains(&id),
                "Event {:?} was not present",
                &id
            );
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
