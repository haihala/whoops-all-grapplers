use bevy::prelude::*;

use crate::{
    assets::{ExtendedFlashMaterial, FlashMaterial},
    event_spreading::FlashPlayer,
};

pub fn handle_flash_events(
    trigger: Trigger<FlashPlayer>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    handles: Query<(Entity, &Handle<ExtendedFlashMaterial>)>,
    parents: Query<&Parent>,
    time: Res<Time>,
) {
    for (material_entity, handle) in &handles {
        let mut parent = parents.get(material_entity).unwrap();

        while let Ok(next) = parents.get(**parent) {
            parent = next;
        }

        // Root level parent ought to be the player
        if **parent != trigger.entity() {
            continue;
        }

        let material = materials.get_mut(handle).unwrap();
        material.extension = FlashMaterial::from_request(trigger.event().0, time.elapsed_seconds());
    }
}
