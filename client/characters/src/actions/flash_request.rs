use bevy::prelude::*;
use wag_core::{HIT_FLASH_COLOR, METER_BAR_FULL_SEGMENT_COLOR};

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct FlashRequest {
    pub color: Color,
    pub speed: f32,
    pub depth: f32,
    pub duration: f32,
}

impl FlashRequest {
    pub fn hit_flash() -> Self {
        Self {
            color: HIT_FLASH_COLOR,
            depth: 1.0,
            duration: 0.2,
            ..default()
        }
    }
}

impl Default for FlashRequest {
    fn default() -> Self {
        Self {
            color: METER_BAR_FULL_SEGMENT_COLOR,
            speed: 30.0,
            depth: 1.0,
            duration: 0.5,
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
