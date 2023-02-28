use bevy::prelude::*;

#[derive(Reflect, FromReflect, Debug, Clone, Copy, PartialEq)]
pub struct StatusEffect {
    // TODO: These do nothing
    pub animation_speed_multiplier: f32,
    pub damage_multiplier: f32,
    pub projectile_speed_multiplier: f32,

    // These work
    pub walk_speed_multiplier: f32,
    pub max_health: i32,
}

impl Default for StatusEffect {
    fn default() -> Self {
        Self {
            animation_speed_multiplier: 1.0,
            walk_speed_multiplier: 1.0,
            damage_multiplier: 1.0,
            projectile_speed_multiplier: 1.0,
            max_health: 0,
        }
    }
}

impl StatusEffect {
    pub fn combine(self, rhs: &Self) -> Self {
        Self {
            animation_speed_multiplier: self.animation_speed_multiplier
                + rhs.animation_speed_multiplier,
            walk_speed_multiplier: self.walk_speed_multiplier + rhs.walk_speed_multiplier,
            damage_multiplier: self.damage_multiplier + rhs.damage_multiplier,
            projectile_speed_multiplier: self.projectile_speed_multiplier
                + rhs.projectile_speed_multiplier,
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
