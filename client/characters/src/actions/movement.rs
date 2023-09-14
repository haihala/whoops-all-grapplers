use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Movement {
    pub amount: Vec2,
    pub duration: usize,
}
impl Movement {
    pub(crate) fn impulse(amount: Vec2) -> Self {
        Self {
            amount,
            duration: 1,
        }
    }
}
