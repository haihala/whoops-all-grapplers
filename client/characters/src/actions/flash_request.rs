use bevy::prelude::*;

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
            color: Color::rgb(0.14, 0.7, 0.8),
            speed: 30.0,
            depth: 0.5,
            duration: 0.5,
        }
    }
}
