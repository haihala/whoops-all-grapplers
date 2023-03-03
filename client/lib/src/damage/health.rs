use bevy::prelude::*;

use characters::Action;
use player_state::PlayerState;

#[derive(Reflect, Component, Clone, Copy)]
pub struct Health {
    value: usize,

    // As this is also stored elsewhere (Stats), could maybe be removed from here in the future
    // TODO: Think about it
    max: usize,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            value: 100,
            max: 100,
        }
    }
}
impl Health {
    pub fn get_percentage(&self) -> f32 {
        (self.value as f32 / self.max as f32) * 100.0
    }

    pub fn reset(&mut self, max_health: i32) {
        self.max = max_health as usize;
        self.value = max_health as usize;
    }

    pub fn apply_damage(&mut self, amount: usize) {
        if amount > self.value {
            // Prevent underflow
            self.value = 0;
        } else {
            self.value -= amount;
        }
    }
}

pub(super) fn take_damage(mut query: Query<(&mut PlayerState, &mut Health)>) {
    for (mut state, mut health) in &mut query {
        for amount in state.drain_matching_actions(|action| {
            if let Action::TakeDamage(amount) = action {
                Some(*amount)
            } else {
                None
            }
        }) {
            health.apply_damage(amount);
        }
    }
}
