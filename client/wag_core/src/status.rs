use bevy::prelude::*;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Component)]
pub struct Stats {
    pub walk_speed: f32,
    pub max_health: i32,
    pub flat_damage: i32,
    // Opener
    pub opener_damage_multiplier: f32,
    pub opener_meter_gain: i32,
    pub opener_stun_frames: i32,
    // Actions
    pub action_speed_multiplier: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            walk_speed: 3.0,
            max_health: 250,
            opener_damage_multiplier: 1.5,
            opener_meter_gain: 50,
            opener_stun_frames: 5,
            ..Self::identity()
        }
    }
}

impl Stats {
    pub fn identity() -> Self {
        Self {
            // These are meant to be identity values, you should be able to
            // combine them with another Stats instance and get the other instance out.
            // Useful for folding and stuff.
            walk_speed: 0.0,
            max_health: 0,
            flat_damage: 0,
            opener_damage_multiplier: 1.0,
            opener_meter_gain: 0,
            opener_stun_frames: 0,
            action_speed_multiplier: 1.0,
        }
    }

    pub fn combine(mut self, rhs: &Self) -> Self {
        self.walk_speed += rhs.walk_speed;
        self.max_health += rhs.max_health;
        self.flat_damage += rhs.flat_damage;
        self.opener_damage_multiplier *= rhs.opener_damage_multiplier;
        self.opener_meter_gain += rhs.opener_meter_gain;
        self.opener_stun_frames += rhs.opener_stun_frames;
        self.action_speed_multiplier *= rhs.action_speed_multiplier;

        self
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusFlag {
    #[default]
    Default, // Not in use, here to satisfy inspectable
    Intangible,
    Parry,
}

#[derive(Reflect, Debug, Clone, Copy, Default, PartialEq)]
pub struct StatusCondition {
    pub flag: StatusFlag,
    pub effect: Option<Stats>,
    pub expiration: Option<usize>,
}
