use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use player_state::{PlayerState, StateEvent};

use crate::meter::Meter;

#[derive(Inspectable, Component, Clone, Copy)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    value: i32,
    max: i32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            value: 100,
            max: 100,
        }
    }
}
impl Health {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    pub fn reset(&mut self) {
        *self = Health::default();
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.value -= amount;
        self.ratio = self.value as f32 / self.max as f32;
    }
}

pub fn refill_meter(mut query: Query<(&mut Meter, &mut PlayerState)>) {
    for (mut meter, mut state) in query.iter_mut() {
        if let Some(free_event) = state
            .get_events()
            .iter()
            .find(|event| matches!(event, StateEvent::Recovery))
        {
            state.consume_event(*free_event);
            meter.flush_combo();
        }
    }
}
