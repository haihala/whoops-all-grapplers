use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Debug, Clone, Default)]
pub struct StatusEffect {
    pub animation_speed_multiplier: f32,
    pub walk_speed_multiplier: f32,
    pub damage_multiplier: f32,
    pub projectile_speed_multiplier: f32,
    pub max_health: i32,
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

#[derive(Inspectable, Debug, Clone, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Default, // Not in use, here to satisfy inspectable
}

#[derive(Inspectable, Debug, Clone, Default)]
pub struct StatusCondition {
    pub name: Status,
    pub effect: Option<StatusEffect>,
    pub expiration: Option<usize>,
}
