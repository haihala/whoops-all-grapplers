use bevy::prelude::*;
use characters::{Properties, SpecialProperty};
use input_parsing::InputParser;
use wag_core::Clock;

pub fn manage_charge(mut query: Query<(&mut Properties, &InputParser)>, clock: Res<Clock>) {
    for (mut properties, parser) in &mut query {
        for property in &mut properties.special_properties {
            let mut clear = false;
            let mut gain = 0;

            if let Some(SpecialProperty::Charge(ref mut charge_props)) = property.special {
                let direction_held = !charge_props.directions.is_empty()
                    && charge_props
                        .directions
                        .contains(&parser.get_relative_stick_position());

                let buttons_pressed = !charge_props.buttons.is_empty()
                    && charge_props
                        .buttons
                        .iter()
                        .all(|button| parser.get_pressed().contains(button));

                let frames_since = clock.frame - charge_props.last_gain_frame;

                if direction_held || buttons_pressed {
                    gain = frames_since as i32;
                    charge_props.last_gain_frame = clock.frame;
                } else if frames_since > charge_props.clear_time {
                    clear = true;
                }
            }

            // Moved here to avoid a double mutable borrow.
            // The elements being borrowed would be mutually exclusive but rust can't see it
            if gain > 0 {
                property.gain(if property.is_empty() {
                    // First frame of charge
                    // if this isn't hear, it will charge to full always after about a second in the round.
                    1
                } else {
                    gain
                });
            } else if clear {
                property.clear();
            }
        }
    }
}
