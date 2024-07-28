use bevy::prelude::*;
use characters::ActionEvent;
use player_state::PlayerState;

use crate::assets::{ExtendedFlashMaterial, FlashMaterial};

pub fn handle_flash_events(
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    handles: Query<(Entity, &Handle<ExtendedFlashMaterial>)>,
    parents: Query<&Parent>,
    mut players: Query<(Entity, &mut PlayerState)>,
    time: Res<Time>,
) {
    for (root, mut state) in &mut players {
        for flash_request in state.drain_matching_actions(|action| {
            if let ActionEvent::Flash(flash_request) = action {
                Some(flash_request.to_owned())
            } else {
                None
            }
        }) {
            for (material_entity, handle) in &handles {
                let mut parent = parents.get(material_entity).unwrap();

                while let Ok(next) = parents.get(**parent) {
                    parent = next;
                }

                // Root level parent ought to be the player
                if **parent != root {
                    continue;
                }

                let material = materials.get_mut(handle).unwrap();
                material.extension =
                    FlashMaterial::from_request(flash_request, time.elapsed_seconds());
            }
        }
    }
}
