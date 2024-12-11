use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, Event)]
pub struct Movement {
    pub amount: Vec2,
    pub duration: usize,
}
impl Movement {
    pub fn impulse(amount: Vec2) -> Self {
        Self {
            amount,
            duration: 1,
        }
    }
}
