use bevy::prelude::*;
use characters::{SpecialProperty, WAGResources};
use input_parsing::InputParser;
use wag_core::{Clock, Facing};

pub fn manage_charge(
    mut query: Query<(&mut WAGResources, &InputParser, &Facing)>,
    clock: Res<Clock>,
) {
    for (mut properties, parser, facing) in &mut query {
        for (_, prop) in &mut properties.iter_mut() {
            let mut clear = false;
            let mut gain = 0;

            if let Some(SpecialProperty::Charge(ref mut charge_props)) = prop.special {
                let direction_held = !charge_props.directions.is_empty()
                    && charge_props
                        .directions
                        .contains(&facing.mirror_stick_pos(parser.get_stick_pos()));

                let buttons_pressed = !charge_props.buttons.is_empty()
                    && charge_props
                        .buttons
                        .iter()
                        .all(|button| parser.get_pressed().contains(button));

                let charging = direction_held || buttons_pressed;
                let frames_since_last_gain = if clock.frame > charge_props.last_gain_frame {
                    clock.frame - charge_props.last_gain_frame
                } else {
                    0
                };

                // Done this way to normalize frame skips but not allow repeatedly tapping back to build charge at the same pace as holding back
                if charging {
                    if charge_props.charging {
                        gain = frames_since_last_gain;
                    }
                    charge_props.last_gain_frame = clock.frame;
                } else if frames_since_last_gain > charge_props.clear_time {
                    clear = true;
                }
                charge_props.charging = charging;
            }

            // Moved here to avoid a double mutable borrow.
            // The elements being borrowed would be mutually exclusive but rust can't see it
            if gain > 0 {
                prop.gain(gain as i32);
            } else if clear {
                prop.clear();
            }
        }
    }
}
