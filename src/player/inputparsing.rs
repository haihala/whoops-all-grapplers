use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Controller(Gamepad, pub StickPosition);

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
pub struct InputFrame {
    #[allow(dead_code)]
    pub frame: i32,
    pub stick_move: Option<StickPosition>,
    pub pressed: HashSet<ActionButton>,
    #[allow(dead_code)]
    pub released: HashSet<ActionButton>,
}

pub struct InputBuffer {
    pub frames: VecDeque<InputFrame>,
    pub interpreted: Vec<SpecialMove>,
}
impl InputBuffer {
    fn contains(&self, input: &SpecialMove) -> bool {
        let requirements = match input {
            SpecialMove::QuarterCircle => {
                vec![StickPosition::S, StickPosition::SE, StickPosition::E]
            }
            SpecialMove::BackwardQuarterCircle => {
                vec![StickPosition::S, StickPosition::SW, StickPosition::W]
            }
        };

        let mut requirements_iter = requirements.iter();
        let mut requirement = requirements_iter.next().unwrap().clone();

        for event in self.frames.iter() {
            if let Some(position) = &event.stick_move {
                if position == &requirement {
                    if let Some(next) = requirements_iter.next() {
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
    uncontrolled: Query<Entity, (With<super::Player>, Without<Controller>)>,
    mut unused_pads: Option<ResMut<Vec<Controller>>>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);
                match uncontrolled.single() {
                    Ok(entity) => {
                        commands
                            .entity(entity)
                            .insert(Controller(*id, StickPosition::Neutral));
                    }
                    Err(_) => {
                        let new_controller = Controller(*id, StickPosition::Neutral);
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
    mut players: Query<(&mut Controller, &mut InputBuffer)>,
    clock: Res<crate::Clock>,
    button_mappings: Res<HashMap<GamepadButtonType, ActionButton>>,
) {
    for (mut controller, mut buffer) in players.iter_mut() {
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

        buffer.frames.push_back(InputFrame {
            frame: clock.0,
            stick_move: if stick_position != controller.1 {
                Some(stick_position.clone())
            } else {
                None
            },
            pressed,
            released,
        });

        controller.1 = stick_position.clone();
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
