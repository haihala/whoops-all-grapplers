use bevy::prelude::*;
use characters::{Action, Resources};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pay_resources).add_system(gain_meter);
    }
}

fn pay_resources(mut query: Query<(&mut PlayerState, &mut Resources)>) {
    for (mut state, mut resources) in &mut query {
        for bill in state.drain_matching_actions(|action| {
            if let Action::Pay(cost) = action {
                Some(*cost)
            } else {
                None
            }
        }) {
            resources.pay(bill);
        }
    }
}

fn gain_meter(mut query: Query<(&mut PlayerState, &mut Resources)>) {
    for (mut state, mut resources) in &mut query {
        for amount in state.drain_matching_actions(|action| {
            if let Action::GainMeter(amount) = action {
                Some(*amount)
            } else {
                None
            }
        }) {
            resources.meter.gain(amount);
        }
    }
}
