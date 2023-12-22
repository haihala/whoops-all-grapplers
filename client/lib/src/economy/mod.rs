mod item_costs;
use bevy::prelude::*;
use characters::{ActionEvent, WAGResources};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (modify_properties, item_costs::manage_item_consumption),
        );
    }
}

fn modify_properties(mut query: Query<(&mut PlayerState, &mut WAGResources)>) {
    for (mut state, mut properties) in &mut query {
        for prop in state.drain_matching_actions(|action| match action {
            ActionEvent::ModifyResource(prop, amount) => {
                Some(ActionEvent::ModifyResource(*prop, *amount))
            }
            ActionEvent::ClearResource(prop) => Some(ActionEvent::ClearResource(*prop)),
            _ => None,
        }) {
            // Moved outside to avoid double borrow
            match prop {
                ActionEvent::ModifyResource(prop, amount) => {
                    dbg!("mod");
                    properties.get_mut(prop).unwrap().change(amount);
                }
                ActionEvent::ClearResource(prop) => {
                    dbg!("clear");
                    properties.get_mut(prop).unwrap().clear();
                }
                _ => panic!("Filter failed"),
            }
        }
    }
}
