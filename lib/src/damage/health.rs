use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use input_parsing::InputParser;
use player_state::{PlayerState, StateEvent};
use types::{HeightWindow, Hit, Player};

use crate::{clock::Clock, meter::Meter, physics::PlayerVelocity};

const CHIP_DAMAGE_MULTIPLIER: f32 = 0.01;
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
    hits: Vec<(Hit, HeightWindow)>,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            value: 100,
            max: 100,
            combo_damage: 0,
            hits: Vec::new(),
        }
    }
}
impl Health {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    pub fn reset(&mut self) {
        self.value = self.max;
        self.ratio = 1.0;
    }

    pub fn hit(&mut self, hit: Hit, height_window: HeightWindow) {
        self.hits.push((hit, height_window));
    }

    fn drain_hits(&mut self) -> Vec<(Hit, HeightWindow)> {
        self.hits.drain(..).collect()
    }

    fn drain_meter_gain(&mut self) -> i32 {
        let temp = self.combo_damage;
        self.combo_damage = 0;
        (temp as f32 * METER_GAINED_PER_DAMAGE) as i32
    }

    fn apply_damage(&mut self, amount: i32) {
        self.value -= amount;
        self.combo_damage += amount;
        self.ratio = self.value as f32 / self.max as f32;
    }
}

pub fn apply_hits(
    mut players: Query<(
        &mut Health,
        &mut PlayerState,
        &mut PlayerVelocity,
        &InputParser,
        &Player,
    )>,
    clock: Res<Clock>,
) {
    let mut attacker_knockbacks = vec![];

    for (mut health, mut state, mut velocity, reader, player) in players.iter_mut() {
        for (hit, height_window) in health.drain_hits() {
            let stick = reader.get_relative_stick_position();

            let (damage, stun, defender_knockback) =
                if state.blocked(hit.fixed_height, height_window, stick) {
                    attacker_knockbacks.push((
                        player.other(),
                        mirror_knockback(hit.block_knockback, !state.flipped()),
                    ));

                    (
                        (hit.damage as f32 * CHIP_DAMAGE_MULTIPLIER).ceil() as i32,
                        hit.block_stun,
                        mirror_knockback(hit.block_knockback, state.flipped()),
                    )
                } else {
                    (
                        hit.damage,
                        hit.hit_stun,
                        mirror_knockback(hit.hit_knockback, state.flipped()),
                    )
                };

            health.apply_damage(damage);
            velocity.add_impulse(defender_knockback);
            state.hit(stun + clock.frame, defender_knockback.y > 0.0);
        }
    }
    for (_, _, mut velocity, _, player) in players.iter_mut() {
        for (_, amount) in attacker_knockbacks
            .iter()
            .filter(|(target, _)| *target == *player)
        {
            velocity.add_impulse(*amount);
        }
    }
}

fn mirror_knockback(knockback: Vec3, flipped: bool) -> Vec3 {
    // Flipped is from the target's perspective
    if flipped {
        knockback
    } else {
        Vec3::new(-knockback.x, knockback.y, knockback.z)
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
