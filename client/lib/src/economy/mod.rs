use bevy::prelude::*;
use characters::{ActionEvent, WAGResources};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, modify_properties);
    }
}

fn modify_properties(mut query: Query<(&mut PlayerState, &mut WAGResources)>) {
    for (mut state, mut properties) in &mut query {
        for prop in state.drain_matching_actions(|action| match action {
            ActionEvent::ModifyProperty(prop, amount) => {
                Some(ActionEvent::ModifyProperty(*prop, *amount))
            }
            ActionEvent::ClearProperty(prop) => Some(ActionEvent::ClearProperty(*prop)),
            _ => None,
        }) {
            // Moved outside to avoid double borrow
            match prop {
                ActionEvent::ModifyProperty(prop, amount) => {
                    properties.get_mut(&prop).unwrap().change(amount);
                }
                ActionEvent::ClearProperty(prop) => {
                    properties.get_mut(&prop).unwrap().clear();
                }
                _ => panic!("Filter failed"),
            }
        }
    }
}
