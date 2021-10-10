use std::{collections::VecDeque, time::Instant};

use crate::helper_types::{ButtonUpdate, Diff, Frame, InputChange, OwnedChange};
use crate::special::Special;
use bevy::{prelude::*, utils::HashMap};
use moves::SpecialDefinition;
use types::{GameButton, MoveType, Normal, PlayerState, StickPosition};

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
pub struct InputReader {
    events: HashMap<MoveType, Instant>,

    controller: Option<Gamepad>,
    registered_specials: HashMap<MoveType, Special>,
    registered_normals: HashMap<MoveType, Normal>,
    head: Frame,
    relative_stick: StickPosition,

    // This is a workaround to dpad inputs
    // Not an elegant one, but the first that came to mind
    temp_stick: StickPosition,
}

impl Default for InputReader {
    fn default() -> Self {
        Self {
            controller: None,
            events: Default::default(),
            registered_specials: Default::default(),
            registered_normals: Default::default(),
            head: Default::default(),
            relative_stick: Default::default(),
            temp_stick: Default::default(),
        }
    }
}
impl InputReader {
    pub fn is_active(&self) -> bool {
        self.controller.is_some()
    }

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
        self.temp_stick = self.head.stick_position;

        let relative_diff = if flipped {
            self.relative_stick = self.temp_stick.flip();
            diff.flip()
        } else {
            self.relative_stick = self.temp_stick;
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
        self.events
            .retain(|_, timestamp| timestamp.elapsed().as_secs_f32() < crate::EVENT_REPEAT_PERIOD)
    }
}

pub fn parse_controller_input(
    gamepad_events: EventReader<GamepadEvent>,
    unused_pads: ResMut<VecDeque<Gamepad>>,
    mut readers: Query<(&mut InputReader, &PlayerState)>,
) {
    let raw_events = handle_raw_events(gamepad_events, unused_pads, &mut readers);
    update_readers(readers, raw_events);
}

/// Returns a vector of input updates made by players that control characters
fn handle_raw_events(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut unused_pads: ResMut<VecDeque<Gamepad>>,
    mut readers: &mut Query<(&mut InputReader, &PlayerState)>,
) -> Vec<OwnedChange> {
    gamepad_events
        .iter()
        .filter_map(|GamepadEvent(id, kind)| match kind {
            GamepadEventType::Connected => {
                pad_connection(id, &mut readers, &mut unused_pads);
                None
            }
            GamepadEventType::Disconnected => {
                pad_disconnection(id, &mut readers, &mut unused_pads);
                None
            }
            GamepadEventType::AxisChanged(axis, new_value) => {
                axis_change(id, *axis, *new_value, &mut readers)
            }
            GamepadEventType::ButtonChanged(button, new_value) => {
                button_change(id, *button, *new_value, &mut readers)
            }
        })
        .collect()
}

fn pad_connection(
    id: &Gamepad,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
    unused_pads: &mut ResMut<VecDeque<Gamepad>>,
) {
    println!("New gamepad connected with ID: {:?}", id);

    for (mut reader, _) in readers.iter_mut() {
        if reader.controller.is_none() {
            reader.controller = Some(*id);
            return;
        }
    }

    unused_pads.push_back(*id);
}

fn pad_disconnection(
    id: &Gamepad,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
    unused_pads: &mut ResMut<VecDeque<Gamepad>>,
) {
    println!("Gamepad disconnected with ID: {:?}", id);
    let next_in_queue = unused_pads.pop_front();

    for (mut reader, _) in readers.iter_mut() {
        if let Some(controller) = reader.controller {
            if controller == *id {
                reader.controller = next_in_queue;
                return;
            }
        }
    }
}

fn axis_change(
    id: &Gamepad,
    axis: GamepadAxisType,
    new_value: f32,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
) -> Option<OwnedChange> {
    let current = readers
        .iter_mut()
        .find_map(|(reader, _)| {
            if reader.controller == Some(*id) {
                Some(reader.temp_stick)
            } else {
                None
            }
        })
        .unwrap();

    let stick = if new_value.abs() > crate::STICK_DEAD_ZONE {
        match axis {
            // Even though DPad axis are on the list, they don't fire
            GamepadAxisType::LeftStickX | GamepadAxisType::RightStickX | GamepadAxisType::DPadX => {
                IVec2::new(new_value.signum() as i32, 0).into()
            }
            GamepadAxisType::LeftStickY | GamepadAxisType::RightStickY | GamepadAxisType::DPadY => {
                IVec2::new(0, new_value.signum() as i32).into()
            }
            // No clue what these are
            GamepadAxisType::LeftZ => todo!(),
            GamepadAxisType::RightZ => todo!(),
        }
    } else {
        IVec2::new(0, 0).into()
    };

    if stick != current {
        Some(OwnedChange {
            change: InputChange::Stick(stick),
            controller: *id,
        })
    } else {
        None
    }
}

fn button_change(
    id: &Gamepad,
    button: GamepadButtonType,
    new_value: f32,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
) -> Option<OwnedChange> {
    // TODO: real button mappings
    let update = if new_value > 0.1 {
        ButtonUpdate::Pressed
    } else {
        ButtonUpdate::Released
    };

    if let Some(change) = match button {
        GamepadButtonType::South => Some(InputChange::Button(GameButton::Fast, update)),
        GamepadButtonType::East => Some(InputChange::Button(GameButton::Heavy, update)),

        GamepadButtonType::DPadUp => dpad_position(id, readers, new_value as i32, None, Some(1)),
        GamepadButtonType::DPadDown => dpad_position(id, readers, new_value as i32, None, Some(-1)),
        GamepadButtonType::DPadLeft => dpad_position(id, readers, new_value as i32, Some(-1), None),
        GamepadButtonType::DPadRight => dpad_position(id, readers, new_value as i32, Some(1), None),

        _ => None,
    } {
        return Some(OwnedChange {
            change,
            controller: *id,
        });
    }
    None
}

fn dpad_position(
    id: &Gamepad,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
    value: i32,
    delta_x: Option<i32>,
    delta_y: Option<i32>,
) -> Option<InputChange> {
    for (mut reader, _) in readers.iter_mut() {
        if reader.controller == Some(*id) {
            let mut stick: IVec2 = reader.temp_stick.into();
            if let Some(x) = delta_x {
                stick.x = x * value;
            }
            if let Some(y) = delta_y {
                stick.y = y * value;
            }
            reader.temp_stick = stick.into();
            return Some(InputChange::Stick(reader.temp_stick));
        }
    }
    None
}

fn update_readers(
    mut readers: Query<(&mut InputReader, &PlayerState)>,
    raw_events: Vec<OwnedChange>,
) {
    let diffs = combine_raw_diffs(raw_events, &mut readers);

    for (mut reader, state) in readers.iter_mut() {
        if let Some(controller) = reader.controller {
            if let Some(diff) = diffs.get(&controller) {
                reader.add_frame(diff.to_owned(), state.flipped());
            }
        }
        reader.purge_old_events();
    }
}

fn combine_raw_diffs(
    raw_events: Vec<OwnedChange>,
    readers: &mut Query<(&mut InputReader, &PlayerState)>,
) -> HashMap<Gamepad, Diff> {
    readers
        .iter_mut()
        .map(|(reader, _)| reader)
        .filter_map(|reader| reader.controller)
        .map(|controller| {
            (
                controller,
                raw_events
                    .iter()
                    .filter(|x| x.controller == controller)
                    .fold(Diff::default(), |a, b| a.apply(&b.change)),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use moves::MotionDefinition;
    use types::PlayerState;

    use super::*;

    #[test]
    fn hadouken_recognized() {
        let mut reader = InputReader::default();
        reader.register_special(
            moves::ryan::HADOUKEN,
            Special::from((
                MotionDefinition::from(vec![2, 3, 6]),
                Some(GameButton::Fast),
            )),
        );
        let (mut world, mut update_stage) = test_setup(reader);

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::HADOUKEN);
    }

    #[test]
    fn early_button_hadouken_recognized() {
        let mut reader = InputReader::default();
        reader.register_special(
            moves::ryan::HADOUKEN,
            Special::from((
                MotionDefinition::from(vec![2, 3, 6]),
                Some(GameButton::Fast),
            )),
        );
        let (mut world, mut update_stage) = test_setup(reader);

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);
        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::SE);
        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_no_events(&mut world);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::E);

        assert_event_is_present(&mut &mut world, moves::ryan::HADOUKEN);
    }

    #[test]
    fn normal_recognized() {
        let mut reader = InputReader::default();
        reader.register_normal(
            moves::ryan::PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: None,
            },
        );

        let (mut world, mut update_stage) = test_setup(reader);

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::PUNCH);

        // Run a few frames
        update_stage.run(&mut world);
        update_stage.run(&mut world);
        update_stage.run(&mut world);

        // Check that the event is still in (repeat works)
        assert_event_is_present(&mut &mut world, moves::ryan::PUNCH);

        // Wait for the event to leave the buffer
        sleep(Duration::from_secs_f32(crate::EVENT_REPEAT_PERIOD));
        update_stage.run(&mut world);

        assert_no_events(&mut world);
    }

    #[test]
    fn command_normal_recognized() {
        let mut reader = InputReader::default();
        reader.register_normal(
            moves::ryan::COMMAND_PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: Some(StickPosition::S),
            },
        );

        let (mut world, mut update_stage) = test_setup(reader);

        add_stick_and_tick(&mut world, &mut update_stage, StickPosition::S);

        assert_no_events(&mut world);

        add_button_and_tick(&mut world, &mut update_stage, GameButton::Fast);

        assert_event_is_present(&mut &mut world, moves::ryan::COMMAND_PUNCH);
    }

    fn test_setup(mut reader: InputReader) -> (World, SystemStage) {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(fake_parser.system());

        reader.controller = Some(Gamepad(1));

        world.spawn().insert(reader).insert(PlayerState::default());

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        // Initial tick
        update_stage.run(&mut world);

        (world, update_stage)
    }

    fn fake_parser(
        readers: Query<(&mut InputReader, &PlayerState)>,
        mut events: ResMut<Vec<OwnedChange>>,
    ) {
        update_readers(readers, events.to_vec());
        events.clear();
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
        let mut changes = world.get_resource_mut::<Vec<OwnedChange>>().unwrap();
        changes.push(OwnedChange {
            controller: Gamepad(1),
            change,
        });
    }

    fn assert_event_is_present(world: &mut World, id: MoveType) {
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 1);

            for (event, _) in r.events.iter() {
                assert_eq!(event, &id);
            }
        }
    }

    fn assert_no_events(world: &mut World) {
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }
    }
}
