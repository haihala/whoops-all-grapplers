use bevy::prelude::*;
use wag_core::Animation;

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct AnimationRequest {
    pub animation: Animation,
    pub position_offset: Vec2,
    pub invert: bool,
    pub looping: bool,
    pub ignore_action_speed: bool,
}
impl From<Animation> for AnimationRequest {
    fn from(animation: Animation) -> Self {
        Self {
            animation,
            ..default()
        }
    }
}
