use bevy::prelude::*;
use characters::{Action, Properties};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(modify_properties);
    }
}

fn modify_properties(mut query: Query<(&mut PlayerState, &mut Properties)>) {
    for (mut state, mut properties) in &mut query {
        for prop in state.drain_matching_actions(|action| match action {
            Action::ModifyResource(resource_index, amount) => {
                Some(Action::ModifyResource(*resource_index, *amount))
            }
            Action::ClearResource(resource_index) => Some(Action::ClearResource(*resource_index)),
            Action::ModifyMeter(amount) => Some(Action::ModifyMeter(*amount)),
            Action::ModifyHealth(amount) => Some(Action::ModifyHealth(*amount)),
            _ => None,
        }) {
            // Moved outside to avoid double borrow
            match prop {
                Action::ModifyResource(resource_index, amount) => {
                    properties.special_properties[resource_index].change(amount);
                }
                Action::ClearResource(resource_index) => {
                    properties.special_properties[resource_index].clear();
                }
                Action::ModifyMeter(amount) => {
                    properties.meter.change(amount);
                }
                Action::ModifyHealth(amount) => {
                    properties.health.change(amount);
                }
                _ => panic!("Filter failed"),
            }
        }
    }
}
