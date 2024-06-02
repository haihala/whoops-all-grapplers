use bevy::prelude::*;
use characters::{ActionEvent, Inventory, ResourceType, WAGResources};
use player_state::PlayerState;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (modify_properties, manage_item_consumption));
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

pub fn manage_item_consumption(
    mut players: Query<(&mut PlayerState, &mut Inventory, &mut WAGResources)>,
) {
    for (mut state, mut inventory, mut resources) in &mut players {
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
            resources
                .get_mut(ResourceType::ItemCount(item))
                .unwrap()
                .drain(1);
        }
    }
}
