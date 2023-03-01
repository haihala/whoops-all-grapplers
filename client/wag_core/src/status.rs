use bevy::prelude::*;

#[derive(Reflect, FromReflect, Debug, Clone, Copy, PartialEq, Component)]
pub struct StatusEffect {
    pub walk_speed_multiplier: f32,
    pub max_health: i32,
    // TODO: Add more fields
}

impl Default for StatusEffect {
    fn default() -> Self {
        Self {
            walk_speed_multiplier: 1.0,
            max_health: 0,
        }
    }
}

impl StatusEffect {
    pub fn combine(self, rhs: &Self) -> Self {
        Self {
            walk_speed_multiplier: self.walk_speed_multiplier * rhs.walk_speed_multiplier,
            max_health: self.max_health + rhs.max_health,
        }
    }
}

#[derive(Reflect, FromReflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Default, // Not in use, here to satisfy inspectable
    Dodge,
    Parry,
}

#[derive(Reflect, FromReflect, Debug, Clone, Copy, Default, PartialEq)]
pub struct StatusCondition {
    pub name: Status,
    pub effect: Option<StatusEffect>,
    pub expiration: Option<usize>,
}
