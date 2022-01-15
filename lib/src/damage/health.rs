use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use input_parsing::InputParser;
use player_state::PlayerState;
use types::{HeightWindow, Hit};

use crate::{clock::Clock, physics::PlayerVelocity};

const CHIP_DAMAGE_MULTIPLIER: f32 = 0.01;

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    defense: f32,
    hits: Vec<(Hit, HeightWindow)>,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            defense: 100.0,
            hits: Vec::new(),
        }
    }
}
impl Health {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    pub fn reset(&mut self) {
        self.ratio = 1.0;
    }

    pub fn hit(&mut self, hit: Hit, height_window: HeightWindow) {
        self.hits.push((hit, height_window));
    }

    fn drain_hits(&mut self) -> Vec<(Hit, HeightWindow)> {
        self.hits.drain(..).collect()
    }

    fn apply_damage(&mut self, amount: f32) {
        self.ratio -= amount / self.defense;
    }
}

pub fn apply_hits(
    mut query: Query<(
        &mut Health,
        &mut PlayerState,
        &mut PlayerVelocity,
        &InputParser,
    )>,
    clock: Res<Clock>,
) {
    for (mut health, mut state, mut velocity, reader) in query.iter_mut() {
        for (hit, height_window) in health.drain_hits() {
            let stick: IVec2 = reader.get_relative_stick_position().into();
            let holding_back = stick.x == -1;
            let holding_down = stick.y == -1;
            let blocked =
                holding_back && state.blocked(hit.fixed_height, height_window, holding_down);

            let (damage, stun, knockback) = if blocked {
                (
                    hit.damage * CHIP_DAMAGE_MULTIPLIER,
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
            velocity.add_impulse(knockback);
            state.hit(stun + clock.frame, knockback.y > 0.0);
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
