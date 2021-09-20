use bevy::prelude::*;

use std::collections::{HashMap, HashSet, VecDeque};

use crate::clock::Clock;

pub mod special_moves;
mod stick_position;
pub use stick_position::StickPosition;

#[derive(Debug)]
pub struct Controller(Gamepad);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionButton {
    Heavy,
    Fast,
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

#[derive(Debug, Default)]
pub struct InputStore {
    diff_buffer: VecDeque<InputUpdate>,
    pub stick_position: StickPosition,
    pub recently_pressed: HashSet<ActionButton>,
    pub recently_released: HashSet<ActionButton>,
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
    let mut uci = uncontrolled.iter();
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                if let Some(character) = uci.next() {
                    commands.entity(character).insert(Controller(*id));
                } else {
                    match unused_pads {
                        Some(ref mut queue) => {
                            queue.push(Controller(*id));
                        }
                        None => {
                            commands.insert_resource(vec![Controller(*id)]);
                        }
                    };
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
        let last_recent_frame = if clock.frame > crate::constants::RECENT_INPUT_FRAMES {
            clock.frame - crate::constants::RECENT_INPUT_FRAMES
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
                frame: clock.frame,
                stick_move,
                pressed,
                released,
            });
        }
    }
}

pub fn cull_diff_buffer(mut query: Query<&mut InputStore>, clock: Res<Clock>) {
    let oldest_allowed_frame = if clock.frame > crate::constants::INPUT_BUFFER_FRAMES {
        clock.frame - crate::constants::INPUT_BUFFER_FRAMES
    } else {
        0
    };

    for mut inputs in query.iter_mut() {
        inputs
            .diff_buffer
            .retain(|x| x.frame >= oldest_allowed_frame);
    }
}
