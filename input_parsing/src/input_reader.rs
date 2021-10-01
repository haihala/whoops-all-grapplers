use std::{collections::VecDeque, time::Instant};

use crate::{
    ButtonUpdate, Diff, Frame, GameButton, InputChange, OwnedChange, SpecialMove, StickPosition,
};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash)]
pub struct InputEvent {
    pub id: Uuid,
    created: Instant,
}

#[derive(PartialEq)]
struct IncompleteMotionInput {
    progress: usize,
    previous_event_time: Instant,
    next: StickPosition,
    done: bool,
}
impl IncompleteMotionInput {
    fn new(next: StickPosition) -> IncompleteMotionInput {
        IncompleteMotionInput {
            next,

            progress: 1,
            previous_event_time: Instant::now(),
            done: false,
        }
    }
}

#[derive(Default)]
/// This is a component and used as an interface
/// Main tells this what Actions to send what events from
pub struct InputReader {
    pub events: HashSet<InputEvent>,

    controller: Option<Gamepad>,
    registered_events: HashMap<Uuid, SpecialMove>,
    head: Frame,
    flipped: bool,
    motions_in_progress: HashMap<Uuid, IncompleteMotionInput>,

    // This is a workaround to dpad inputs
    // Not an elegant one, but the first that came to mind
    temp_stick: StickPosition,
}
impl InputReader {
    pub fn register(&mut self, uuid: Uuid, action: SpecialMove) {
        self.registered_events.insert(uuid, action);
    }

    pub fn unregister(&mut self, uuid: &Uuid) {
        self.registered_events.remove(uuid);
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
    }

    pub fn get_stick_position(&self) -> StickPosition {
        self.head.stick_position.clone()
    }
    pub fn get_temp_stick(&self) -> StickPosition {
        self.temp_stick.clone()
    }
    pub fn set_temp_stick(&mut self, new: StickPosition) {
        self.temp_stick = new;
    }

    fn add_frame(&mut self, diff: Diff) {
        self.head.apply(diff.clone());
        self.temp_stick = self.head.stick_position.clone();

        self.start_motion_inputs();
        self.increment_motion_inputs(&diff);
        self.complete_motion_inputs(&diff);
    }

    fn start_motion_inputs(&mut self) {
        let unstarted: HashMap<&Uuid, &SpecialMove> = self
            .registered_events
            .iter()
            .filter(|(key, _)| !self.motions_in_progress.contains_key(key))
            .collect();

        for (target, action) in unstarted.iter() {
            let first = action.motion.requirements(self.flipped).next().unwrap();

            if first == self.head.stick_position {
                self.motions_in_progress
                    .insert(<Uuid>::clone(target), IncompleteMotionInput::new(first));
            }
        }
    }
    fn increment_motion_inputs(&mut self, diff: &Diff) {
        for (target, motion) in self.motions_in_progress.iter_mut() {
            if !motion.done {
                if let Some(stick) = diff.stick_move.clone() {
                    // Stick position changed
                    if stick == motion.next {
                        // New stick position is what this motion needed

                        if let Some(next) =
                            Self::get_requirements(&self.registered_events, target, self.flipped)
                                .nth(motion.progress)
                        {
                            // Still more to go
                            motion.next = next;
                            motion.progress += 1;
                            motion.previous_event_time = Instant::now();
                        } else {
                            // Motion has no more requirements, so it's done
                            motion.done = true;
                        }
                    }
                }
            }
        }
    }

    fn get_requirements(
        registered_events: &HashMap<Uuid, SpecialMove>,
        target: &Uuid,
        flipped: bool,
    ) -> Box<dyn Iterator<Item = StickPosition>> {
        registered_events
            .get(target)
            .unwrap()
            .motion
            .clone()
            .requirements(flipped)
    }

    fn complete_motion_inputs(&mut self, diff: &Diff) {
        if let Some(pressed) = diff.pressed.clone() {
            for (target, motion) in self.motions_in_progress.iter_mut() {
                if motion.done {
                    let action = self.registered_events.get(target).unwrap();
                    if pressed.contains(&action.button) {
                        self.events.insert(InputEvent {
                            id: *target,
                            created: Instant::now(),
                        });
                    }
                }
            }
        }
    }

    fn remove_old_incomplete_moves(&mut self) {
        self.motions_in_progress.retain(|_, motion| {
            motion.previous_event_time.elapsed().as_secs_f32()
                < crate::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
        });

        self.events
            .retain(|event| event.created.elapsed().as_secs_f32() < crate::EVENT_REPEAT_PERIOD)
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
    // TODO: In cases where multiple dpad inputs land on the same frame, this breaks
    // Assumes that stick was in reader.head, which is true for the first event
    for mut reader in readers.iter_mut() {
        if reader.controller == Some(*id) {
            let mut stick: IVec2 = reader.temp_stick.clone().into();
            if let Some(x) = delta_x {
                stick.x = x * value;
            }
            if let Some(y) = delta_y {
                stick.y = y * value;
            }
            reader.temp_stick = stick.into();
            return Some(InputChange::Stick(reader.temp_stick.clone()));
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
                reader.remove_old_incomplete_moves();
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
    use super::*;

    #[test]
    fn hadouken_recognized() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(fake_parser.system());

        let uuid = Uuid::new_v4();
        let mut reader = InputReader::default();
        reader.register(
            uuid,
            SpecialMove {
                motion: vec![2, 3, 6].into(),
                button: GameButton::Fast,
            },
        );
        reader.controller = Some(Gamepad(1));

        world.spawn().insert(reader);

        let inputs: Vec<OwnedChange> = vec![];
        world.insert_resource(inputs);

        // Initial tick
        update_stage.run(&mut world);

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

        // Check that the event got registered
        for r in world.query::<&InputReader>().iter(&world) {
            assert_eq!(r.events.len(), 1);

            for event in r.events.iter() {
                assert_eq!(event.id, uuid);
            }
        }
    }

    fn fake_parser(readers: Query<&mut InputReader>, events: ResMut<Vec<OwnedChange>>) {
        update_readers(readers, events.to_vec());
    }

    fn add_input(world: &mut World, change: InputChange) {
        let mut changes = world.get_resource_mut::<Vec<OwnedChange>>().unwrap();
        changes.clear();
        changes.push(OwnedChange {
            controller: Gamepad(1),
            change,
        });
    }
}
