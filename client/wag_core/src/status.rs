use bevy::prelude::*;

#[derive(Reflect, FromReflect, Debug, Clone, Copy, PartialEq, Component)]
pub struct StatusEffect {
    pub walk_speed_multiplier: f32,
    pub max_health: i32,
    // Opener
    pub opener_damage_multiplier: f32,
    pub opener_meter_gain: i32,
    // TODO: Add more fields
}

impl Default for StatusEffect {
    fn default() -> Self {
        Self {
            walk_speed_multiplier: 1.0,
            max_health: 0,
            opener_damage_multiplier: 1.0,
            opener_meter_gain: 0,
        }
    }
}

impl StatusEffect {
    pub fn combine(mut self, rhs: &Self) -> Self {
        self.walk_speed_multiplier *= rhs.walk_speed_multiplier;
        self.max_health += rhs.max_health;
        self.opener_damage_multiplier *= rhs.opener_damage_multiplier;
        self.opener_meter_gain += rhs.opener_meter_gain;

        self
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
