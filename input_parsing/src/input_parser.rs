use crate::helper_types::{Diff, Frame};
use crate::input_reader::InputReader;
use crate::special::Special;
use bevy::utils::Instant;
use bevy::{prelude::*, utils::HashMap};
use moves::SpecialDefinition;
use player_state::PlayerState;
use types::{MoveType, Normal, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
pub struct InputParser {
    events: HashMap<MoveType, Instant>,

    registered_specials: HashMap<MoveType, Special>,
    registered_normals: HashMap<MoveType, Normal>,
    head: Frame,
    relative_stick: StickPosition,
}

impl Default for InputParser {
    fn default() -> Self {
        Self {
            events: Default::default(),
            registered_specials: Default::default(),
            registered_normals: Default::default(),
            head: Default::default(),
            relative_stick: Default::default(),
        }
    }
}
impl InputParser {
    pub fn load(
        specials: HashMap<MoveType, SpecialDefinition>,
        registered_normals: HashMap<MoveType, Normal>,
    ) -> Self {
        Self {
            registered_specials: specials
                .into_iter()
                .map(|(id, definition)| (id, definition.into()))
                .collect(),
            registered_normals,
            ..Default::default()
        }
    }

    pub fn register_special(&mut self, id: MoveType, special: Special) {
        self.registered_specials.insert(id, special);
    }

    pub fn register_normal(&mut self, id: MoveType, normal: Normal) {
        self.registered_normals.insert(id, normal);
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
        let old_stick = self.relative_stick;

        self.head.apply(diff.clone());

        let relative_diff = if flipped {
            self.relative_stick = self.head.stick_position.flip();
            diff.flip()
        } else {
            self.relative_stick = self.head.stick_position;
            diff
        };

        self.parse_specials(&relative_diff, old_stick);
        self.parse_normals(&relative_diff);
    }

    fn parse_specials(&mut self, diff: &Diff, old_stick: StickPosition) {
        let now = Instant::now();

        self.events.extend(
            self.registered_specials
                .iter_mut()
                .filter_map(|(id, special)| {
                    special.advance(diff, old_stick);
                    if special.is_done() {
                        special.clear();
                        return Some((*id, now));
                    }
                    None
                }),
        );
    }

    fn parse_normals(&mut self, diff: &Diff) {
        if diff.pressed.is_none() {
            // Normals have a button, if no buttons were pressed, no events can fire
            return;
        }

        let stick = self.relative_stick;
        let now = Instant::now();

        self.events.extend(
            self.registered_normals
                .iter()
                .filter(|(_, normal)| diff.pressed_contains(&normal.button))
                .filter(|(_, normal)| normal.stick.is_none() || stick == normal.stick.unwrap())
                .map(|(id, _)| (*id, now)),
        );
    }

    fn purge_old_events(&mut self) {
        self.events.retain(|_, timestamp| {
            timestamp.elapsed().as_secs_f32() < constants::EVENT_REPEAT_PERIOD
        })
    }

    #[cfg(test)]
    fn with_special(id: MoveType, special: Special) -> InputParser {
        let mut parser = InputParser::default();
        parser.register_special(id, special);
        parser
    }

    #[cfg(test)]
    fn with_normal(id: MoveType, normal: Normal) -> InputParser {
        let mut parser = InputParser::default();
        parser.register_normal(id, normal);
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

    use moves::MotionDefinition;
    use player_state::PlayerState;
    use types::GameButton;

    use crate::helper_types::{ButtonUpdate, InputChange};

    use super::*;

    #[test]
    fn hadouken_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_special(
            moves::ryan::HADOUKEN,
            Special::from((
                MotionDefinition::from(vec![2, 3, 6]),
                Some(GameButton::Fast),
            )),
        ));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::HADOUKEN);
    }

    #[test]
    fn early_button_hadouken_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_special(
            moves::ryan::HADOUKEN,
            Special::from((
                MotionDefinition::from(vec![2, 3, 6]),
                Some(GameButton::Fast),
            )),
        ));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_no_events(&mut world);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);

        assert_event_is_present(&mut &mut world, moves::ryan::HADOUKEN);
    }

    #[test]
    fn normal_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_normal(
            moves::ryan::PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: None,
            },
        ));

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::PUNCH);

        tick_frames(&mut world, &mut update_stage, 3);

        // Check that the event is still in (repeat works)
        assert_event_is_present(&mut &mut world, moves::ryan::PUNCH);

        // Wait for the event to leave the buffer
        sleep(Duration::from_secs_f32(
            constants::EVENT_REPEAT_PERIOD + 0.01,
        ));
        tick_frames(&mut world, &mut update_stage, 1);
        assert_no_events(&mut world);
    }

    #[test]
    fn command_normal_recognized() {
        let (mut world, mut update_stage) = test_setup(InputParser::with_normal(
            moves::ryan::COMMAND_PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: Some(StickPosition::S),
            },
        ));

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::COMMAND_PUNCH);
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
            assert_eq!(r.events.len(), 1);

            for (event, _) in r.events.iter() {
                assert_eq!(event, &id);
            }
        }
    }

    fn assert_no_events(world: &mut World) {
        for r in world.query::<&InputParser>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }
    }
}
