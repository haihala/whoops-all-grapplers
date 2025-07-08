use bevy::prelude::*;
use foundation::{HIT_FLASH_COLOR, JACKPOT_COLOR, METER_BAR_FULL_SEGMENT_COLOR};

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Event)]
pub struct FlashRequest {
    pub color: Color,
    pub speed: f32,
    pub depth: f32,
    pub angle_mult: f32,
    pub duration: f32,
}

impl FlashRequest {
    pub fn hit_flash() -> Self {
        Self {
            color: HIT_FLASH_COLOR,
            depth: 0.5,
            duration: 0.2,
            ..default()
        }
    }

    pub fn meter_use() -> Self {
        Self {
            color: METER_BAR_FULL_SEGMENT_COLOR,
            speed: 30.0,
            depth: 0.5,
            duration: 0.5,
            ..default()
        }
    }

    pub fn jackpot(level: i32) -> Self {
        Self {
            color: JACKPOT_COLOR,
            speed: 20.0,
            depth: 0.5,
            angle_mult: 2.0,
            duration: 0.3 * level as f32,
        }
    }
}

impl Default for FlashRequest {
    fn default() -> Self {
        Self {
            color: METER_BAR_FULL_SEGMENT_COLOR,
            speed: 30.0,
            depth: 0.5,
            duration: 0.5,
            angle_mult: 1.0,
        }
    }
}
impl From<Color> for FlashRequest {
    fn from(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
}
