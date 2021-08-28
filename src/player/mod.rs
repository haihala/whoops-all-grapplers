use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Materials;
mod character;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(detect_new_pads.system())
            .add_system(collect_input.system())
            .add_system(parse_input.system())
            .add_system_to_stage(CoreStage::Last, cull_stick_input_buffer.system())
            .add_system(character::ryan.system());
    }
}

struct Player;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ActionButton {
    Vicious,
    Fast,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum StickPosition {
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

impl From<Vec2> for StickPosition {
    fn from(item: Vec2) -> Self {
        match item.y as i32 {
            -1 => match item.x as i32 {
                -1 => StickPosition::SW,
                0 => StickPosition::S,
                1 => StickPosition::SE,
                _ => panic!("Weird Vec2 to Button conversion"),
            },
            0 => match item.x as i32 {
                -1 => StickPosition::W,
                0 => StickPosition::Neutral,
                1 => StickPosition::E,
                _ => panic!("Weird Vec2 to Button conversion"),
            },
            1 => match item.x as i32 {
                -1 => StickPosition::NW,
                0 => StickPosition::N,
                1 => StickPosition::NE,
                _ => panic!("Weird Vec2 to Button conversion"),
            },
            _ => panic!("Weird Vec2 to Button conversion"),
        }
    }
}

#[derive(Debug)]
struct Controller(Gamepad, StickPosition);

#[derive(EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    QuarterCircle,
    BackwardQuarterCircle,
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
    interpreted: Vec<SpecialMove>,
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

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let width = 10.;
    let height = 15.;

    let button_mappings: HashMap<GamepadButtonType, ActionButton> = [
        (GamepadButtonType::South, ActionButton::Fast),
        (GamepadButtonType::West, ActionButton::Vicious),
    ]
    .iter()
    .cloned()
    .collect();

    commands.insert_resource(button_mappings);

    commands
        .spawn_bundle(SpriteBundle {
            material: assets.collision_box_color.clone(),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(Player)
        .insert(character::Ryan)
        .insert(InputBuffer {
            frames: VecDeque::new(),
            interpreted: Vec::new(),
        });
}

fn detect_new_pads(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut controlled: Query<(Entity, &mut Controller)>,
    uncontrolled: Query<Entity, (With<Player>, Without<Controller>)>,
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

static DEAD_ZONE: f32 = 0.5;
fn collect_input(
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

        let stick_sum = Vec2::new(
            xsum.signum() * ((xsum.abs() > DEAD_ZONE) as i32 as f32),
            ysum.signum() * ((ysum.abs() > DEAD_ZONE) as i32 as f32),
        );

        let stick_position: StickPosition = stick_sum.clone().into();
        dbg!(&stick_position, &stick_sum);

        let mut pressed: HashSet<ActionButton> = HashSet::new();
        for btn in buttons.get_just_pressed() {
            if let Some(event) = button_mappings.get(&btn.1) {
                pressed.insert(event.clone());
            }
        }

        let mut released: HashSet<ActionButton> = HashSet::new();
        for btn in buttons.get_just_released() {
            if let Some(event) = button_mappings.get(&btn.1) {
                released.insert(event.clone());
            }
        }

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

fn parse_input(mut query: Query<&mut InputBuffer>) {
    for mut buffer in query.iter_mut() {
        buffer.interpreted.clear();
        // TODO: Check if player is free to act
        for input in SpecialMove::iter() {
            // dbg!(&buffer.frames);
            if buffer.contains(&input) {
                buffer.interpreted.push(input.clone());
            }
        }
    }
}

static INPUT_BUFFER_LENGTH: usize = 60;
fn cull_stick_input_buffer(mut query: Query<&mut InputBuffer>) {
    for mut buffer in query.iter_mut() {
        while buffer.frames.len() > INPUT_BUFFER_LENGTH {
            buffer.frames.pop_front();
        }
    }
}
