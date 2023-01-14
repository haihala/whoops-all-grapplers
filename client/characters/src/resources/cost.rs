use bevy::prelude::*;

#[derive(Debug, Default, Reflect, FromReflect, Clone, Eq, PartialEq, Copy)]
pub struct Cost {
    pub meter: i32,
    pub charge: bool,
    pub bullet: bool,
}
impl Cost {
    pub fn bullet() -> Self {
        Self {
            bullet: true,
            ..default()
        }
    }
    pub fn charge() -> Self {
        Self {
            charge: true,
            ..default()
        }
    }

    pub fn meter(amount: i32) -> Self {
        Self {
            meter: amount,
            ..default()
        }
    }
}
