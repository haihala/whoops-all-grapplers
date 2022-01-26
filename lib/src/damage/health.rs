use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use player_state::{PlayerState, StateEvent};
use types::Player;

use crate::meter::Meter;

const METER_GAINED_PER_DAMAGE: f32 = 0.5;

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    value: i32,
    max: i32,
    combo_damage: i32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            value: 100,
            max: 100,
            combo_damage: 0,
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

    fn drain_meter_gain(&mut self) -> i32 {
        let temp = self.combo_damage;
        self.combo_damage = 0;
        (temp as f32 * METER_GAINED_PER_DAMAGE) as i32
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.value -= amount;
        self.combo_damage += amount;
        self.ratio = self.value as f32 / self.max as f32;
    }
}

pub fn refill_meter(mut query: Query<(&mut Meter, &mut Health, &mut PlayerState, &Player)>) {
    let mut gains = vec![];

    for (_, mut health, mut state, player) in query.iter_mut() {
        if let Some(free_event) = state
            .get_events()
            .iter()
            .find(|event| matches!(event, StateEvent::Recovery))
        {
            state.consume_event(*free_event);
            gains.push((player.other(), health.drain_meter_gain()));
        }
    }

    for (mut meter, _, _, player) in query.iter_mut() {
        for (_, amount) in gains.iter().filter(|(recipient, _)| recipient == player) {
            meter.gain(*amount);
        }
    }
}
