use bevy::prelude::*;
use wag_core::METER_BAR_FULL_SEGMENT_COLOR;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct FlashRequest {
    pub color: Color,
    pub speed: f32,
    pub depth: f32,
    pub duration: f32,
}

impl Default for FlashRequest {
    fn default() -> Self {
        Self {
            color: METER_BAR_FULL_SEGMENT_COLOR,
            speed: 30.0,
            depth: 0.5,
            duration: 0.5,
        }
    }
}