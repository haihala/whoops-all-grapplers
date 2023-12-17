use bevy::prelude::*;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Component)]
pub struct Stats {
    // Resources
    pub max_health: i32,
    pub starting_meter: i32,

    // Damage
    pub flat_damage: i32,
    pub chip_damage: bool,
    pub backdash_invuln: i32,

    // Movement
    pub walk_speed: f32,
    pub gravity: f32,
    pub jump_force_multiplier: f32,

    // Opener
    pub opener_damage_multiplier: f32,
    pub opener_meter_gain: i32,
    pub opener_stun_frames: i32,

    // Actions
    pub action_speed_multiplier: f32,
    pub link_bonus_multiplier: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            walk_speed: 3.0,
            max_health: 250,
            opener_damage_multiplier: 1.5,
            opener_meter_gain: 15,
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
            max_health: 0,
            starting_meter: 0,

            flat_damage: 0,
            chip_damage: true,
            backdash_invuln: 0,

            walk_speed: 0.0,
            gravity: 0.0,
            jump_force_multiplier: 1.0,

            opener_damage_multiplier: 1.0,
            opener_meter_gain: 0,
            opener_stun_frames: 0,

            action_speed_multiplier: 1.0,
            link_bonus_multiplier: 1.0,
        }
    }

    pub fn combine(mut self, rhs: &Self) -> Self {
        self.max_health += rhs.max_health;
        self.starting_meter += rhs.starting_meter;

        self.flat_damage += rhs.flat_damage;
        self.chip_damage &= rhs.chip_damage; // If a source disables chip it's disabled forever
        self.backdash_invuln += rhs.backdash_invuln;

        self.walk_speed += rhs.walk_speed;
        self.gravity += rhs.gravity;
        self.jump_force_multiplier *= rhs.jump_force_multiplier;

        self.opener_damage_multiplier *= rhs.opener_damage_multiplier;
        self.opener_meter_gain += rhs.opener_meter_gain;
        self.opener_stun_frames += rhs.opener_stun_frames;

        self.action_speed_multiplier *= rhs.action_speed_multiplier;
        self.link_bonus_multiplier *= rhs.link_bonus_multiplier;

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
