use bevy::prelude::*;
use characters::{Action, Properties};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, modify_properties);
    }
}

fn modify_properties(mut query: Query<(&mut PlayerState, &mut Properties)>) {
    for (mut state, mut properties) in &mut query {
        for prop in state.drain_matching_actions(|action| match action {
            Action::ModifyProperty(prop, amount) => Some(Action::ModifyProperty(*prop, *amount)),
            Action::ClearProperty(prop) => Some(Action::ClearProperty(*prop)),
            _ => None,
        }) {
            // Moved outside to avoid double borrow
            match prop {
                Action::ModifyProperty(prop, amount) => {
                    properties.get_mut(&prop).unwrap().change(amount);
                }
                Action::ClearProperty(prop) => {
                    properties.get_mut(&prop).unwrap().clear();
                }
                _ => panic!("Filter failed"),
            }
        }
    }
}
