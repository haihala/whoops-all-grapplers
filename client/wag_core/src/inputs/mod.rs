mod stick_position;
use bevy::{prelude::*, utils::HashMap};
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

use crate::Player;

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

impl From<NetworkInputButton> for GameButton {
    fn from(value: NetworkInputButton) -> Self {
        match value {
            // This is where keybindings are sort of defined
            NetworkInputButton::South => GameButton::Fast,
            NetworkInputButton::West => GameButton::Gimmick,
            NetworkInputButton::North => GameButton::Wrestling,
            NetworkInputButton::East => GameButton::Strong,
            NetworkInputButton::Start => GameButton::Start,
            NetworkInputButton::Select => GameButton::Select,
            _ => panic!(),
        }
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
    pub fn from_gamepad_button_type(value: GamepadButtonType) -> Option<Self> {
        Some(match value {
            GamepadButtonType::South => NetworkInputButton::South,
            GamepadButtonType::East => NetworkInputButton::East,
            GamepadButtonType::North => NetworkInputButton::North,
            GamepadButtonType::West => NetworkInputButton::West,
            GamepadButtonType::LeftTrigger => NetworkInputButton::L1,
            GamepadButtonType::LeftTrigger2 => NetworkInputButton::L2,
            GamepadButtonType::RightTrigger => NetworkInputButton::R1,
            GamepadButtonType::RightTrigger2 => NetworkInputButton::R2,
            GamepadButtonType::Select => NetworkInputButton::Select,
            GamepadButtonType::Start => NetworkInputButton::Start,
            GamepadButtonType::LeftThumb => NetworkInputButton::L3,
            GamepadButtonType::RightThumb => NetworkInputButton::R3,
            GamepadButtonType::DPadUp => NetworkInputButton::Up,
            GamepadButtonType::DPadDown => NetworkInputButton::Down,
            GamepadButtonType::DPadLeft => NetworkInputButton::Left,
            GamepadButtonType::DPadRight => NetworkInputButton::Right,
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

    pub fn to_gamepad_button_type(&self) -> GamepadButtonType {
        match self {
            NetworkInputButton::South => GamepadButtonType::South,
            NetworkInputButton::East => GamepadButtonType::East,
            NetworkInputButton::North => GamepadButtonType::North,
            NetworkInputButton::West => GamepadButtonType::West,
            NetworkInputButton::L1 => GamepadButtonType::LeftTrigger,
            NetworkInputButton::L2 => GamepadButtonType::LeftTrigger2,
            NetworkInputButton::R1 => GamepadButtonType::RightTrigger,
            NetworkInputButton::R2 => GamepadButtonType::RightTrigger2,
            NetworkInputButton::Select => GamepadButtonType::Select,
            NetworkInputButton::Start => GamepadButtonType::Start,
            NetworkInputButton::L3 => GamepadButtonType::LeftThumb,
            NetworkInputButton::R3 => GamepadButtonType::RightThumb,
            NetworkInputButton::Up => GamepadButtonType::DPadUp,
            NetworkInputButton::Down => GamepadButtonType::DPadDown,
            NetworkInputButton::Left => GamepadButtonType::DPadLeft,
            NetworkInputButton::Right => GamepadButtonType::DPadRight,
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
                let game_button = GameButton::from(*self);
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
    pub player_handle: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct InputStream {
    pub frame: usize,
    pub events: Vec<OwnedInput>,
    dpads: HashMap<usize, StickPosition>,
    analog_sticks: HashMap<usize, StickPosition>,
}

impl InputStream {
    pub fn update_analog_stick(
        &mut self,
        pad_id: usize,
        axis: GamepadAxisType,
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
            GamepadAxisType::LeftStickX => old_stick.x = snap_value,
            GamepadAxisType::LeftStickY => old_stick.y = snap_value,
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

#[derive(Debug, Resource, Clone, Copy)]
pub struct Controllers {
    pub p1: usize,
    pub p2: usize,
}

impl Default for Controllers {
    fn default() -> Self {
        Controllers { p1: 0, p2: 1 }
    }
}

impl Controllers {
    pub fn get_handle(&self, player: Player) -> usize {
        match player {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn get_player(&self, handle: usize) -> Option<Player> {
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
pub struct LocalController(pub usize);
