use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::collections::{HashMap, HashSet, VecDeque};

use crate::player::PlayerState;

#[derive(Debug)]
pub struct Controller(Gamepad);

#[derive(EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    QuarterCircle,
    BackwardQuarterCircle,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionButton {
    Vicious,
    Fast,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct InputFrame {
    #[allow(dead_code)]
    frame: i32,
    stick_move: Option<StickPosition>,
    pressed: HashSet<ActionButton>,
    #[allow(dead_code)]
    released: HashSet<ActionButton>,
}

pub struct InputBuffer {
    frames: VecDeque<InputFrame>,
    pub interpreted: Vec<SpecialMove>,
    pub stick_position: StickPosition,
    pub recently_pressed: HashSet<ActionButton>,
    pub recently_released: HashSet<ActionButton>,
}
impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            frames: Default::default(),
            interpreted: Default::default(),
            stick_position: Default::default(),
            recently_pressed: Default::default(),
            recently_released: Default::default(),
        }
    }
}
impl InputBuffer {
    fn contains(&self, input: &SpecialMove) -> bool {
        let mut requirements = match input {
            SpecialMove::QuarterCircle => {
                vec![StickPosition::S, StickPosition::SE, StickPosition::E]
            }
            SpecialMove::BackwardQuarterCircle => {
                vec![StickPosition::S, StickPosition::SW, StickPosition::W]
            }
        }
        .into_iter();

        let mut requirement = requirements.next().unwrap();

        for event in self.frames.iter() {
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
    mut players: Query<(&Controller, &mut InputBuffer, &mut PlayerState)>,
    clock: Res<crate::Clock>,
    button_mappings: Res<HashMap<GamepadButtonType, ActionButton>>,
) {
    for (controller, mut buffer, mut player_state) in players.iter_mut() {
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

        player_state.decelerating = stick_position == StickPosition::Neutral;

        let stick_move = if stick_position != buffer.stick_position {
            Some(stick_position.clone())
        } else {
            None
        };

        buffer.stick_position = stick_position;

        let no_longer_recent_frame_index =
            if buffer.frames.len() > crate::constants::RECENT_INPUT_FRAMES {
                buffer.frames.len() - crate::constants::RECENT_INPUT_FRAMES
            } else {
                0
            };

        buffer.recently_pressed = buffer
            .recently_pressed
            .clone()
            .into_iter()
            .filter(|x| {
                buffer.frames[no_longer_recent_frame_index]
                    .pressed
                    .contains(x)
            })
            .chain(pressed.clone().into_iter())
            .collect();

        buffer.recently_released = buffer
            .recently_released
            .clone()
            .into_iter()
            .filter(|x| {
                buffer.frames[no_longer_recent_frame_index]
                    .released
                    .contains(x)
            })
            .chain(released.clone().into_iter())
            .collect();

        buffer.frames.push_back(InputFrame {
            frame: clock.0,
            stick_move,
            pressed,
            released,
        });
    }
}

pub fn interpret_stick_inputs(mut query: Query<&mut InputBuffer>) {
    for mut buffer in query.iter_mut() {
        buffer.interpreted.clear();
        // TODO: Check if player is free to act
        for input in SpecialMove::iter() {
            if buffer.contains(&input) {
                buffer.interpreted.push(input.clone());
            }
        }
    }
}

pub fn cull_stick_input_buffer(mut query: Query<&mut InputBuffer>) {
    for mut buffer in query.iter_mut() {
        while buffer.frames.len() > crate::constants::INPUT_BUFFER_LENGTH {
            buffer.frames.pop_front();
        }
    }
}
