use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use input_parsing::InputReader;
use types::{Hit, PlayerState};

use crate::{clock::Clock, physics::PhysicsObject};

#[derive(Inspectable)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    ratio: f32,
    defense: f32,
    hits: Vec<Hit>,
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

    pub fn hit(&mut self, hit: Hit) {
        self.hits.push(hit);
    }

    fn drain_hits(&mut self) -> Vec<Hit> {
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
        &mut PhysicsObject,
        &InputReader,
    )>,
    clock: Res<Clock>,
) {
    for (mut health, mut state, mut physics_object, reader) in query.iter_mut() {
        for hit in health.drain_hits() {
            // Todo high/low
            let stick: IVec2 = reader.get_relative_stick_position().into();
            let blocked = stick.x == -1; // Holding back

            let (damage, stun, knockback) = if blocked {
                (
                    hit.damage * crate::CHIP_DAMAGE_MULTIPLIER,
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
            state.stun(stun + clock.frame);
            physics_object.add_impulse(knockback);
            if knockback.y > 0.0 {
                state.jump();
            }
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

pub fn recover_from_hitstun(mut query: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in query.iter_mut() {
        if let Some(stun_end_frame) = state.stunned_until() {
            if clock.frame >= stun_end_frame {
                state.recover_from_hitstun();
            }
        }
    }
}
