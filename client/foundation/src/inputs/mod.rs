use bevy::{prelude::*, utils::HashMap};

use strum_macros::EnumIter;

use crate::Player;

mod stick_position;
pub use stick_position::StickPosition;

mod input_state;
pub use input_state::InputState;

pub const STICK_DEAD_ZONE: f32 = 0.3;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Reflect, Default)]
/// Buttons of the game
/// The name 'Button' is in prelude
/// This is for in match inputs
pub enum GameButton {
    #[default]
    Default, // To satisfy Inspectable

    Start,
    Select,

    Fast,
    Strong,
    Wrestling,
    Gimmick,
}

impl GameButton {
    pub fn to_dsl(self) -> String {
        match self {
            GameButton::Default => panic!("Default can't be converted to dsl"),
            GameButton::Start => ".",
            GameButton::Select => ",",
            GameButton::Fast => "f",
            GameButton::Strong => "s",
            GameButton::Wrestling => "w",
            GameButton::Gimmick => "g",
        }
        .into()
    }
}

impl TryFrom<NetworkInputButton> for GameButton {
    type Error = (); // This could be improved

    fn try_from(value: NetworkInputButton) -> Result<Self, ()> {
        Ok(match value {
            // This is where keybindings are sort of defined
            NetworkInputButton::South => GameButton::Fast,
            NetworkInputButton::West => GameButton::Gimmick,
            NetworkInputButton::North => GameButton::Wrestling,
            NetworkInputButton::East => GameButton::Strong,
            NetworkInputButton::Start => GameButton::Start,
            NetworkInputButton::Select => GameButton::Select,
            _ => return Err(()),
        })
    }
}

// Game runs with strictly digital input, this is an abstraction
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
pub enum NetworkInputButton {
    Up,
    Down,
    Left,
    Right,
    South,
    West,
    North,
    East,
    Start,
    Select,
    R1,
    R2,
    R3,
    L1,
    L2,
    L3,
}

impl NetworkInputButton {
    pub fn from_gamepad_button_type(value: GamepadButton) -> Option<Self> {
        Some(match value {
            GamepadButton::South => NetworkInputButton::South,
            GamepadButton::East => NetworkInputButton::East,
            GamepadButton::North => NetworkInputButton::North,
            GamepadButton::West => NetworkInputButton::West,
            GamepadButton::LeftTrigger => NetworkInputButton::L1,
            GamepadButton::LeftTrigger2 => NetworkInputButton::L2,
            GamepadButton::RightTrigger => NetworkInputButton::R1,
            GamepadButton::RightTrigger2 => NetworkInputButton::R2,
            GamepadButton::Select => NetworkInputButton::Select,
            GamepadButton::Start => NetworkInputButton::Start,
            GamepadButton::LeftThumb => NetworkInputButton::L3,
            GamepadButton::RightThumb => NetworkInputButton::R3,
            GamepadButton::DPadUp => NetworkInputButton::Up,
            GamepadButton::DPadDown => NetworkInputButton::Down,
            GamepadButton::DPadLeft => NetworkInputButton::Left,
            GamepadButton::DPadRight => NetworkInputButton::Right,
            _ => return None,
        })
    }

    pub fn from_key(value: KeyCode) -> Option<Self> {
        Some(match value {
            KeyCode::KeyJ => NetworkInputButton::South,
            KeyCode::KeyK => NetworkInputButton::East,
            KeyCode::KeyI => NetworkInputButton::North,
            KeyCode::KeyU => NetworkInputButton::West,
            KeyCode::KeyY => NetworkInputButton::L1,
            KeyCode::KeyH => NetworkInputButton::L2,
            KeyCode::KeyO => NetworkInputButton::R1,
            KeyCode::KeyL => NetworkInputButton::R2,
            KeyCode::KeyV => NetworkInputButton::Select,
            KeyCode::KeyB => NetworkInputButton::Start,
            KeyCode::KeyN => NetworkInputButton::L3,
            KeyCode::KeyM => NetworkInputButton::R3,
            KeyCode::KeyW => NetworkInputButton::Up,
            KeyCode::KeyS => NetworkInputButton::Down,
            KeyCode::KeyA => NetworkInputButton::Left,
            KeyCode::KeyD => NetworkInputButton::Right,
            _ => return None,
        })
    }

    pub fn to_gamepad_button_type(&self) -> GamepadButton {
        match self {
            NetworkInputButton::South => GamepadButton::South,
            NetworkInputButton::East => GamepadButton::East,
            NetworkInputButton::North => GamepadButton::North,
            NetworkInputButton::West => GamepadButton::West,
            NetworkInputButton::L1 => GamepadButton::LeftTrigger,
            NetworkInputButton::L2 => GamepadButton::LeftTrigger2,
            NetworkInputButton::R1 => GamepadButton::RightTrigger,
            NetworkInputButton::R2 => GamepadButton::RightTrigger2,
            NetworkInputButton::Select => GamepadButton::Select,
            NetworkInputButton::Start => GamepadButton::Start,
            NetworkInputButton::L3 => GamepadButton::LeftThumb,
            NetworkInputButton::R3 => GamepadButton::RightThumb,
            NetworkInputButton::Up => GamepadButton::DPadUp,
            NetworkInputButton::Down => GamepadButton::DPadDown,
            NetworkInputButton::Left => GamepadButton::DPadLeft,
            NetworkInputButton::Right => GamepadButton::DPadRight,
        }
    }

    pub fn to_keycode(&self) -> KeyCode {
        match self {
            NetworkInputButton::South => KeyCode::KeyJ,
            NetworkInputButton::East => KeyCode::KeyK,
            NetworkInputButton::North => KeyCode::KeyI,
            NetworkInputButton::West => KeyCode::KeyU,
            NetworkInputButton::L1 => KeyCode::KeyY,
            NetworkInputButton::L2 => KeyCode::KeyH,
            NetworkInputButton::R1 => KeyCode::KeyO,
            NetworkInputButton::R2 => KeyCode::KeyL,
            NetworkInputButton::Select => KeyCode::KeyV,
            NetworkInputButton::Start => KeyCode::KeyB,
            NetworkInputButton::L3 => KeyCode::KeyN,
            NetworkInputButton::R3 => KeyCode::KeyM,
            NetworkInputButton::Up => KeyCode::KeyW,
            NetworkInputButton::Down => KeyCode::KeyS,
            NetworkInputButton::Left => KeyCode::KeyA,
            NetworkInputButton::Right => KeyCode::KeyD,
        }
    }

    pub fn to_input_event(
        &self,
        writer: &mut InputStream,
        pad_id: usize,
        pressed: bool,
    ) -> Option<InputEvent> {
        match self {
            NetworkInputButton::Up
            | NetworkInputButton::Down
            | NetworkInputButton::Left
            | NetworkInputButton::Right => Some(InputEvent::Point(
                writer.update_dpad(pad_id, *self, pressed),
            )),

            NetworkInputButton::South
            | NetworkInputButton::West
            | NetworkInputButton::North
            | NetworkInputButton::East
            | NetworkInputButton::Start
            | NetworkInputButton::Select => {
                let game_button = GameButton::try_from(*self).unwrap();
                Some(if pressed {
                    InputEvent::Press(game_button)
                } else {
                    InputEvent::Release(game_button)
                })
            }

            NetworkInputButton::R1
            | NetworkInputButton::R2
            | NetworkInputButton::R3
            | NetworkInputButton::L1
            | NetworkInputButton::L2
            | NetworkInputButton::L3 => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Copy, Hash)]
pub enum InputEvent {
    Point(StickPosition),
    Press(GameButton),
    Release(GameButton),
}

impl From<char> for InputEvent {
    fn from(ch: char) -> InputEvent {
        if let Ok(number_token) = ch.to_string().parse::<i32>() {
            InputEvent::Point(number_token.into())
        } else {
            match ch {
                'f' => InputEvent::Press(GameButton::Fast),
                'F' => InputEvent::Release(GameButton::Fast),
                's' => InputEvent::Press(GameButton::Strong),
                'S' => InputEvent::Release(GameButton::Strong),
                'w' => InputEvent::Press(GameButton::Wrestling),
                'W' => InputEvent::Release(GameButton::Wrestling),
                'g' => InputEvent::Press(GameButton::Gimmick),
                'G' => InputEvent::Release(GameButton::Gimmick),
                // There is no need for negative edge on start, this whole thing is mighty sus so let's not get caught up on that shall we
                '.' => InputEvent::Press(GameButton::Start),
                ',' => InputEvent::Press(GameButton::Select),
                _ => panic!("Invalid character {ch}"),
            }
        }
    }
}

// TODO: Rename to owned input event etc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnedInput {
    pub event: InputEvent,
    pub player_handle: InputDevice,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct InputStream {
    pub events: Vec<OwnedInput>,
    dpads: HashMap<usize, StickPosition>,
    analog_sticks: HashMap<usize, StickPosition>,
    pub input_states: HashMap<InputDevice, InputState>,
}

impl InputStream {
    pub fn update_analog_stick(
        &mut self,
        pad_id: usize,
        axis: GamepadAxis,
        value: f32,
    ) -> StickPosition {
        let mut old_stick: IVec2 = self
            .analog_sticks
            .get(&pad_id)
            .map(|sp| sp.to_owned())
            .unwrap_or_default()
            .into();

        let snap_value = if value.abs() < STICK_DEAD_ZONE {
            0
        } else {
            value.signum() as i32
        };

        match axis {
            GamepadAxis::LeftStickX => old_stick.x = snap_value,
            GamepadAxis::LeftStickY => old_stick.y = snap_value,
            _ => {}
        };

        let new_stick = old_stick.into();
        self.analog_sticks.insert(pad_id, new_stick);
        new_stick
    }

    pub fn update_dpad(
        &mut self,
        pad_id: usize,
        button: NetworkInputButton,
        pressed: bool,
    ) -> StickPosition {
        let mut old_dpad: IVec2 = self
            .dpads
            .get(&pad_id)
            .map(|sp| sp.to_owned())
            .unwrap_or_default()
            .into();

        // This is cumbersome on devices that don't clean socd
        // Need to figure out a solution where socd and repeated events work
        let val = pressed as i32;
        match button {
            NetworkInputButton::Up => old_dpad.y = val,
            NetworkInputButton::Down => old_dpad.y = -val,
            NetworkInputButton::Left => old_dpad.x = -val,
            NetworkInputButton::Right => old_dpad.x = val,
            _ => panic!(),
        };

        debug_assert!(old_dpad.x.abs() <= 1, "dpad x is greater than 1");
        debug_assert!(old_dpad.y.abs() <= 1, "dpad y is greater than 1");

        old_dpad.x = old_dpad.x.signum();
        old_dpad.y = old_dpad.y.signum();

        let new_stick = old_dpad.into();
        self.dpads.insert(pad_id, new_stick);
        new_stick
    }
}

#[derive(Debug, Resource, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDevice {
    Controller(Entity),
    Keyboard,
    Online(usize),
}

#[derive(Debug, Resource, Clone, Copy)]
pub struct Controllers {
    pub p1: InputDevice,
    pub p2: InputDevice,
}

impl Controllers {
    pub fn get_handle(&self, player: Player) -> InputDevice {
        match player {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn get_player(&self, handle: InputDevice) -> Option<Player> {
        if handle == self.p1 {
            Some(Player::One)
        } else if handle == self.p2 {
            Some(Player::Two)
        } else {
            None
        }
    }
}

#[derive(Debug, Resource, Clone, Copy)]
pub struct LocalController(pub InputDevice);
