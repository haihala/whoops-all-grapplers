use bevy::prelude::*;

use characters::{ActionEvent, Inventory, ResourceType, WAGResources};
use player_state::PlayerState;

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
            inventory.consume(item);
            resources
                .get_mut(ResourceType::ItemCount(item))
                .unwrap()
                .drain(1);
        }
    }
}
