use bevy::prelude::*;
use characters::{ActionEvent, ActionEvents, Inventory, WAGResources};

pub fn modify_properties(mut query: Query<(&ActionEvents, &mut WAGResources)>) {
    for (events, mut properties) in &mut query {
        for prop in events.get_matching_events(|action| match action {
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

pub fn manage_item_consumption(mut players: Query<(&ActionEvents, &mut Inventory)>) {
    for (events, mut inventory) in &mut players {
        for item in events
            .get_matching_events(|action| {
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
