use bevy::prelude::*;
use characters::{ActionEvent, Inventory, WAGResources};
use player_state::PlayerState;

pub fn modify_properties(mut query: Query<(&mut PlayerState, &mut WAGResources)>) {
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
                    properties.get_mut(prop).unwrap().change(amount);
                }
                ActionEvent::ClearResource(prop) => {
                    properties.get_mut(prop).unwrap().clear();
                }
                _ => panic!("Filter failed"),
            }
        }
    }
}

pub fn manage_item_consumption(mut players: Query<(&mut PlayerState, &mut Inventory)>) {
    for (mut state, mut inventory) in &mut players {
        for item in state
            .drain_matching_actions(|action| {
                if let ActionEvent::Consume(id) = action {
                    Some(*id)
                } else {
                    None
                }
            })
            .into_iter()
        {
            inventory.remove(item);
        }
    }
}
