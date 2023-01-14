use bevy::prelude::*;
use characters::{Character, Resources};
use input_parsing::InputParser;
use wag_core::Clock;

const CHARGE_EXPIRATION_TIME: f32 = 0.2;
const CHARGE_EXPIRATION_FRAMES: usize = (CHARGE_EXPIRATION_TIME * wag_core::FPS) as usize;

pub fn manage_charge(
    mut query: Query<(&mut Resources, &InputParser, &Character)>,
    clock: Res<Clock>,
) {
    for (mut resources, parser, character) in &mut query {
        let charge = &mut resources.charge;

        let player_charging = character
            .charge_directions
            .contains(&parser.get_relative_stick_position());

        if player_charging {
            // Bump charge
            charge.progress += 1;
            charge.last_update = clock.frame;
        } else if charge.last_update + CHARGE_EXPIRATION_FRAMES < clock.frame {
            // Charge expiration
            charge.reset();
        }
    }
}
