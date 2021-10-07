use std::{collections::VecDeque, time::Instant};

use crate::{
    ButtonUpdate, Diff, Frame, GameButton, InputChange, Normal, OwnedChange, Special, StickPosition,
};

use bevy::{prelude::*, utils::HashMap};
use uuid::Uuid;

/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
pub struct InputReader {
    pub flipped: bool,
    events: HashMap<Uuid, Instant>,

    controller: Option<Gamepad>,
    registered_specials: HashMap<Uuid, Special>,
    registered_normals: HashMap<Uuid, Normal>,
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
            flipped: false,

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
    pub fn register_special(&mut self, id: Uuid, special: Special) {
        self.registered_specials.insert(id, special);
    }

    pub fn register_normal(&mut self, id: Uuid, normal: Normal) {
        self.registered_normals.insert(id, normal);
    }

    pub fn get_absolute_stick_position(&self) -> StickPosition {
        self.head.stick_position
    }

    pub fn get_relative_stick_position(&self) -> StickPosition {
        self.relative_stick
    }

    pub fn drain_events(&mut self) -> Vec<Uuid> {
        self.events.drain().map(|(id, _)| id).collect()
    }

    fn add_frame(&mut self, diff: Diff) {
        self.head.apply(diff.clone());
        self.temp_stick = self.head.stick_position;

        let relative_diff = if self.flipped {
            self.relative_stick = self.temp_stick.flip();
            diff.flip()
        } else {
            self.relative_stick = self.temp_stick;
            diff
        };

        self.parse_specials(&relative_diff);
        self.parse_normals(&relative_diff);
    }

    fn parse_specials(&mut self, diff: &Diff) {
        let now = Instant::now();

        self.events.extend(
            self.registered_specials
                .iter_mut()
                .filter_map(|(id, special)| {
                    if special.advance(diff) {
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
    mut readers: Query<&mut InputReader>,
) {
    let raw_events = handle_raw_events(gamepad_events, unused_pads, &mut readers);
    update_readers(readers, raw_events);
}

/// Returns a vector of input updates made by players that control characters
fn handle_raw_events(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut unused_pads: ResMut<VecDeque<Gamepad>>,
    mut readers: &mut Query<&mut InputReader>,
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
                Some(axis_change(id, *axis, *new_value))
            }
            GamepadEventType::ButtonChanged(button, new_value) => {
                button_change(id, *button, *new_value, &mut readers)
            }
        })
        .collect()
}

fn pad_connection(
    id: &Gamepad,
    readers: &mut Query<&mut InputReader>,
    unused_pads: &mut ResMut<VecDeque<Gamepad>>,
) {
    println!("New gamepad connected with ID: {:?}", id);

    for mut reader in readers.iter_mut() {
        if reader.controller.is_none() {
            reader.controller = Some(*id);
            return;
        }
    }

    unused_pads.push_back(*id);
}

fn pad_disconnection(
    id: &Gamepad,
    readers: &mut Query<&mut InputReader>,
    unused_pads: &mut ResMut<VecDeque<Gamepad>>,
) {
    println!("Gamepad disconnected with ID: {:?}", id);
    let next_in_queue = unused_pads.pop_front();

    for mut reader in readers.iter_mut() {
        if let Some(controller) = reader.controller {
            if controller == *id {
                reader.controller = next_in_queue;
                return;
            }
        }
    }
}

fn axis_change(id: &Gamepad, axis: GamepadAxisType, new_value: f32) -> OwnedChange {
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

    OwnedChange {
        change: InputChange::Stick(stick),
        controller: *id,
    }
}

fn button_change(
    id: &Gamepad,
    button: GamepadButtonType,
    new_value: f32,
    readers: &mut Query<&mut InputReader>,
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
    readers: &mut Query<&mut InputReader>,
    value: i32,
    delta_x: Option<i32>,
    delta_y: Option<i32>,
) -> Option<InputChange> {
    for mut reader in readers.iter_mut() {
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

fn update_readers(mut readers: Query<&mut InputReader>, raw_events: Vec<OwnedChange>) {
    let diffs = combine_raw_diffs(raw_events, &mut readers);

    for mut reader in readers.iter_mut() {
        if let Some(controller) = reader.controller {
            if let Some(diff) = diffs.get(&controller) {
                reader.add_frame(diff.to_owned());
                reader.purge_old_events();
            }
        }
    }
}

fn combine_raw_diffs(
    raw_events: Vec<OwnedChange>,
    readers: &mut Query<&mut InputReader>,
) -> HashMap<Gamepad, Diff> {
    readers
        .iter_mut()
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

    use super::*;

    #[test]
    fn hadouken_recognized() {
        let id = Uuid::new_v4();
        let mut reader = InputReader::default();
        reader.register_special(
            id,
            Special {
                motion: vec![2, 3, 6].into(),
                button: Some(GameButton::Fast),
            },
        );
        let (mut world, mut update_stage) = test_setup(reader);

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        // Down
        add_input(&mut world, InputChange::Stick(StickPosition::S));
        update_stage.run(&mut world);

        // Down forward
        add_input(&mut world, InputChange::Stick(StickPosition::SE));
        update_stage.run(&mut world);

        // Forward
        add_input(&mut world, InputChange::Stick(StickPosition::E));
        update_stage.run(&mut world);

        // Check that the event isn't recognized before the button
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }

        // Button to finish
        add_input(
            &mut world,
            InputChange::Button(GameButton::Fast, ButtonUpdate::Pressed),
        );
        update_stage.run(&mut world);

        assert_event_is_present(&mut &mut world, id);
    }

    #[test]
    fn normal_recognized() {
        let id = Uuid::new_v4();
        let mut reader = InputReader::default();
        reader.register_normal(
            id,
            Normal {
                button: GameButton::Fast,
                stick: None,
            },
        );

        let (mut world, mut update_stage) = test_setup(reader);

        // Check that the event isn't recognized before the button
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }

        // Button
        add_input(
            &mut world,
            InputChange::Button(GameButton::Fast, ButtonUpdate::Pressed),
        );
        update_stage.run(&mut world);

        assert_event_is_present(&mut &mut world, id);

        // Run a few frames
        update_stage.run(&mut world);
        update_stage.run(&mut world);
        update_stage.run(&mut world);

        // Check that the event is still in (repeat works)
        assert_event_is_present(&mut &mut world, id);

        // Wait for the event to leave the buffer
        sleep(Duration::from_secs_f32(crate::EVENT_REPEAT_PERIOD));
        update_stage.run(&mut world);

        // Check that the event is deleted
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }
    }

    #[test]
    fn command_normal_recognized() {
        let id = Uuid::new_v4();

        let mut reader = InputReader::default();
        reader.register_normal(
            id,
            Normal {
                button: GameButton::Fast,
                stick: Some(StickPosition::S),
            },
        );

        let (mut world, mut update_stage) = test_setup(reader);

        // Down
        add_input(&mut world, InputChange::Stick(StickPosition::S));
        update_stage.run(&mut world);

        // Check that the event isn't recognized before the button
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 0);
        }

        // Button
        add_input(
            &mut world,
            InputChange::Button(GameButton::Fast, ButtonUpdate::Pressed),
        );
        update_stage.run(&mut world);

        assert_event_is_present(&mut &mut world, id);
    }

    fn test_setup(mut reader: InputReader) -> (World, SystemStage) {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(fake_parser.system());

        reader.controller = Some(Gamepad(1));

        world.spawn().insert(reader);

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        // Initial tick
        update_stage.run(&mut world);

        (world, update_stage)
    }

    fn fake_parser(readers: Query<&mut InputReader>, mut events: ResMut<Vec<OwnedChange>>) {
        update_readers(readers, events.to_vec());
        events.clear();
    }

    fn add_input(world: &mut World, change: InputChange) {
        let mut changes = world.get_resource_mut::<Vec<OwnedChange>>().unwrap();
        changes.push(OwnedChange {
            controller: Gamepad(1),
            change,
        });
    }

    fn assert_event_is_present(world: &mut World, id: Uuid) {
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 1);

            for (event, _) in r.events.iter() {
                assert_eq!(event, &id);
            }
        }
    }
}
