use bevy::prelude::*;

use crate::WEAKEN_STATUS_COLOR;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Component)]
pub struct Stats {
    // Resources
    pub max_health: i32,
    pub starting_meter: i32,

    // Damage
    pub damage_multiplier: f32,
    pub chip_damage: bool,
    pub backdash_invuln: i32,
    pub defense_meter: i32,

    // Movement
    pub walk_speed: f32,
    pub gravity: f32,
    pub gravity_scaling: f32,
    pub jump_force_multiplier: f32,

    // Opener
    pub opener_damage_multiplier: f32,
    pub opener_meter_gain: i32,
    pub opener_stun_frames: i32,

    // Actions
    pub action_speed_multiplier: f32,
    pub meter_per_second: f32,

    // Direct Influence
    pub direct_influence: f32,

    // Samurai
    pub kunais: i32,
    pub auto_sharpen: i32,
    pub retain_sharpness: bool,
}

impl std::hash::Hash for Stats {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.max_health.hash(state);
        self.starting_meter.hash(state);

        self.damage_multiplier.to_bits().hash(state);
        self.chip_damage.hash(state);
        self.backdash_invuln.hash(state);
        self.defense_meter.hash(state);

        self.walk_speed.to_bits().hash(state);
        self.gravity.to_bits().hash(state);
        self.gravity_scaling.to_bits().hash(state);
        self.jump_force_multiplier.to_bits().hash(state);

        self.opener_damage_multiplier.to_bits().hash(state);
        self.opener_meter_gain.hash(state);
        self.opener_stun_frames.hash(state);

        self.action_speed_multiplier.to_bits().hash(state);
        self.meter_per_second.to_bits().hash(state);

        self.direct_influence.to_bits().hash(state);

        self.kunais.hash(state);
        self.auto_sharpen.hash(state);
        self.retain_sharpness.hash(state);
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            walk_speed: 3.0,
            max_health: 250,
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
            max_health: 0,
            starting_meter: 0,

            damage_multiplier: 1.0,
            chip_damage: true,
            backdash_invuln: 0,
            defense_meter: 0,

            walk_speed: 0.0,
            gravity: 0.0,
            gravity_scaling: 0.0,
            jump_force_multiplier: 1.0,

            opener_damage_multiplier: 1.0,
            opener_meter_gain: 0,
            opener_stun_frames: 0,

            action_speed_multiplier: 1.0,
            meter_per_second: 0.0,

            direct_influence: 0.0,

            kunais: 0,
            auto_sharpen: 0,
            retain_sharpness: false,
        }
    }

    pub fn combine(mut self, rhs: &Self) -> Self {
        self.max_health += rhs.max_health;
        self.starting_meter += rhs.starting_meter;

        self.damage_multiplier *= rhs.damage_multiplier;
        self.chip_damage &= rhs.chip_damage; // If a source disables chip it's disabled forever
        self.backdash_invuln += rhs.backdash_invuln;
        self.defense_meter += rhs.defense_meter;

        self.walk_speed += rhs.walk_speed;
        self.gravity += rhs.gravity;
        self.gravity_scaling += rhs.gravity_scaling;
        self.jump_force_multiplier *= rhs.jump_force_multiplier;

        self.opener_damage_multiplier *= rhs.opener_damage_multiplier;
        self.opener_meter_gain += rhs.opener_meter_gain;
        self.opener_stun_frames += rhs.opener_stun_frames;

        self.action_speed_multiplier *= rhs.action_speed_multiplier;
        self.meter_per_second += rhs.meter_per_second;

        self.direct_influence += rhs.direct_influence;

        self.kunais += rhs.kunais;
        self.auto_sharpen += rhs.auto_sharpen;
        self.retain_sharpness |= rhs.retain_sharpness;

        self
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum StatusFlag {
    #[default]
    Default, // Not in use, here to satisfy inspectable
    Intangible,
    Parry,
    DoubleJumped,
    MovementLock,
    Weaken,
}

impl StatusFlag {
    pub fn display_color(&self) -> Option<Color> {
        match self {
            StatusFlag::Weaken => Some(WEAKEN_STATUS_COLOR),
            _ => None,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, Default, PartialEq, Hash)]
pub struct StatusCondition {
    pub flag: StatusFlag,
    pub effect: Option<Stats>,
    pub expiration: Option<usize>,
}
