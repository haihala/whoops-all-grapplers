use bevy::prelude::*;
use input_parsing::InputParser;
use kits::Resources;
use time::Clock;

const CHARGE_EXPIRATION_TIME: f32 = 0.2;
const CHARGE_EXPIRATION_FRAMES: usize = (CHARGE_EXPIRATION_TIME * constants::FPS) as usize;

pub fn manage_charge(mut query: Query<(&mut Resources, &InputParser)>, clock: Res<Clock>) {
    for (mut resources, parser) in query.iter_mut() {
        let charge = &mut resources.charge;
        let stick: IVec2 = parser.get_relative_stick_position().into();
        let holding_down = stick.y == -1;
        let holding_back = stick.x == -1;

        let player_charging = holding_back && holding_down;
        let player_maintaining_charge = holding_back || holding_down;

        if player_charging {
            // Bump charge
            charge.progress += 1;
            charge.last_update = clock.frame;
        } else if player_maintaining_charge {
            // Refresh so it doesn't expire
            charge.last_update = clock.frame;
        } else if charge.last_update + CHARGE_EXPIRATION_FRAMES < clock.frame {
            // Charge expiration
            charge.reset();
        }
    }
}
