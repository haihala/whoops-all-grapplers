use bevy::prelude::*;

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::clock::Clock;

pub mod special_moves;

#[derive(Debug)]
pub struct Controller(Gamepad);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionButton {
    Heavy,
    Fast,
}

#[derive(EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StickPosition {
    NW,
    N,
    NE,
    W,
    Neutral,
    E,
    SW,
    S,
    SE,
}
impl Default for StickPosition {
    fn default() -> Self {
        StickPosition::Neutral
    }
}
impl From<i32> for StickPosition {
    fn from(numpad: i32) -> Self {
        match numpad {
            1 => StickPosition::SW,
            2 => StickPosition::S,
            3 => StickPosition::SE,
            4 => StickPosition::W,
            5 => StickPosition::Neutral,
            6 => StickPosition::E,
            7 => StickPosition::NW,
            8 => StickPosition::N,
            9 => StickPosition::NE,
            _ => panic!("Invalid numpad to StickPosition conversion"),
        }
    }
}
impl From<IVec2> for StickPosition {
    fn from(item: IVec2) -> Self {
        let matrix = vec![
            vec![StickPosition::SW, StickPosition::S, StickPosition::SE],
            vec![StickPosition::W, StickPosition::Neutral, StickPosition::E],
            vec![StickPosition::NW, StickPosition::N, StickPosition::NE],
        ];

        matrix[(item.y + 1) as usize][(item.x + 1) as usize].clone()
    }
}
// Can't implement traits for bevy types
#[allow(clippy::from_over_into)]
impl Into<IVec2> for StickPosition {
    fn into(self) -> IVec2 {
        match self {
            StickPosition::NW => (-1, 1).into(),
            StickPosition::N => (0, 1).into(),
            StickPosition::NE => (1, 1).into(),
            StickPosition::W => (-1, 0).into(),
            StickPosition::Neutral => (0, 0).into(),
            StickPosition::E => (1, 0).into(),
            StickPosition::SW => (-1, -1).into(),
            StickPosition::S => (0, -1).into(),
            StickPosition::SE => (1, -1).into(),
        }
    }
}

#[test]
fn test_ivec_stickposition_conversions() {
    for sp1 in StickPosition::iter() {
        let ivec: IVec2 = sp1.clone().into();
        let sp2: StickPosition = ivec.into();
        assert!(sp1 == sp2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InputUpdate {
    #[allow(dead_code)]
    frame: usize,
    stick_move: Option<StickPosition>,
    pressed: HashSet<ActionButton>,
    #[allow(dead_code)]
    released: HashSet<ActionButton>,
}

pub struct InputStore {
    diff_buffer: VecDeque<InputUpdate>,
    pub stick_position: StickPosition,
    pub recently_pressed: HashSet<ActionButton>,
    pub recently_released: HashSet<ActionButton>,
}
impl Default for InputStore {
    fn default() -> Self {
        Self {
            diff_buffer: Default::default(),
            stick_position: Default::default(),
            recently_pressed: Default::default(),
            recently_released: Default::default(),
        }
    }
}
impl InputStore {
    pub fn contains(&self, mut requirements: Box<dyn Iterator<Item = StickPosition>>) -> bool {
        let mut requirement = requirements.next().unwrap();

        for event in self.diff_buffer.iter() {
            if let Some(position) = &event.stick_move {
                if position == &requirement {
                    if let Some(next) = requirements.next() {
                        // Get the next requirement
                        requirement = next.clone();
                    } else {
                        // We've gone through all the inputs, so it matches
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn detect_new_pads(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut controlled: Query<(Entity, &mut Controller)>,
    uncontrolled: Query<Entity, (With<crate::Player>, Without<Controller>)>,
    mut unused_pads: Option<ResMut<Vec<Controller>>>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);
                match uncontrolled.single() {
                    Ok(entity) => {
                        commands.entity(entity).insert(Controller(*id));
                    }
                    Err(_) => {
                        let new_controller = Controller(*id);
                        match unused_pads {
                            Some(ref mut queue) => {
                                queue.push(new_controller);
                            }
                            None => {
                                let mut queue = VecDeque::new();
                                queue.push_back(new_controller);
                                commands.insert_resource(queue);
                            }
                        };
                    }
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);
                for (entity, mut controller) in controlled.iter_mut() {
                    if controller.0 == *id {
                        match unused_pads {
                            Some(ref mut queue) => {
                                if queue.len() > 0 {
                                    controller.0 = queue.pop().unwrap().0;
                                } else {
                                    commands.entity(entity).remove::<Controller>();
                                }
                            }
                            None => {
                                commands.entity(entity).remove::<Controller>();
                            }
                        };
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

pub fn collect_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut players: Query<(&Controller, &mut InputStore)>,
    clock: Res<crate::Clock>,
    button_mappings: Res<HashMap<GamepadButtonType, ActionButton>>,
) {
    for (controller, mut inputs) in players.iter_mut() {
        let lstick_x_axis = GamepadAxis(controller.0, GamepadAxisType::LeftStickX);
        let lstick_y_axis = GamepadAxis(controller.0, GamepadAxisType::LeftStickY);

        let (stick_x, stick_y) = match (axes.get(lstick_x_axis), axes.get(lstick_y_axis)) {
            (Some(stick_x), Some(stick_y)) => (stick_x, stick_y),
            _ => (0.0, 0.0),
        };

        let up_button = GamepadButton(controller.0, GamepadButtonType::DPadUp);
        let down_button = GamepadButton(controller.0, GamepadButtonType::DPadDown);
        let left_button = GamepadButton(controller.0, GamepadButtonType::DPadLeft);
        let right_button = GamepadButton(controller.0, GamepadButtonType::DPadRight);
        let dpad_x = (buttons.pressed(right_button) as i32) - (buttons.pressed(left_button) as i32);
        let dpad_y = (buttons.pressed(up_button) as i32) - (buttons.pressed(down_button) as i32);

        let xsum = dpad_x as f32 + stick_x;
        let ysum = dpad_y as f32 + stick_y;

        let stick_sum = IVec2::new(xsum.round() as i32, ysum.round() as i32);

        let stick_position: StickPosition = stick_sum.into();

        let pressed: HashSet<ActionButton> = buttons
            .get_just_pressed()
            .into_iter()
            .filter_map(|btn| button_mappings.get(&btn.1))
            .map(|btn| btn.to_owned())
            .collect();

        let released: HashSet<ActionButton> = buttons
            .get_just_released()
            .into_iter()
            .filter_map(|btn| button_mappings.get(&btn.1))
            .map(|btn| btn.to_owned())
            .collect();

        let stick_move = if stick_position != inputs.stick_position {
            Some(stick_position.clone())
        } else {
            None
        };

        inputs.stick_position = stick_position;
        let last_recent_frame = if clock.0 > crate::constants::RECENT_INPUT_FRAMES {
            clock.0 - crate::constants::RECENT_INPUT_FRAMES
        } else {
            0
        };

        let recent_frames: Vec<InputUpdate> = inputs
            .diff_buffer
            .iter()
            .filter(|x| x.frame >= last_recent_frame)
            .cloned()
            .collect();

        inputs.recently_pressed = recent_frames
            .iter()
            .flat_map(|x| x.pressed.clone())
            .collect();

        inputs.recently_released = recent_frames
            .iter()
            .flat_map(|x| x.released.clone())
            .collect();

        if !pressed.is_empty() || !released.is_empty() || stick_move.is_some() {
            inputs.diff_buffer.push_back(InputUpdate {
                frame: clock.0,
                stick_move,
                pressed,
                released,
            });
        }
    }
}

pub fn cull_diff_buffer(mut query: Query<&mut InputStore>, clock: Res<Clock>) {
    let oldest_allowed_frame = if clock.0 > crate::constants::INPUT_BUFFER_FRAMES {
        clock.0 - crate::constants::INPUT_BUFFER_FRAMES
    } else {
        0
    };

    for mut inputs in query.iter_mut() {
        inputs
            .diff_buffer
            .retain(|x| x.frame >= oldest_allowed_frame);
    }
}
