use bevy::prelude::*;
use characters::FlashRequest;
use foundation::FPS;

use crate::{assets::ExtendedFlashMaterial, event_spreading::ColorShift};

pub fn handle_flash_events(
    trigger: Trigger<FlashRequest>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    handles: Query<(Entity, &MeshMaterial3d<ExtendedFlashMaterial>)>,
    parents: Query<&ChildOf>,
    time: Res<Time>,
) {
    for (material_entity, handle) in &handles {
        let mut parent = parents.get(material_entity).unwrap().parent();

        while let Ok(next) = parents.get(parent) {
            parent = next.parent();
        }

        // Root level parent ought to be the player
        if parent != trigger.target() {
            continue;
        }

        let material = materials.get_mut(handle).unwrap();
        material.extension.flash_start = time.elapsed_secs();
        let req = trigger.event();
        material.extension.color = req.color.into();
        material.extension.speed = req.speed;
        material.extension.depth = req.depth;
        material.extension.duration = req.duration;
    }
}

pub fn handle_color_shift(
    trigger: Trigger<ColorShift>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    handles: Query<(Entity, &MeshMaterial3d<ExtendedFlashMaterial>)>,
    parents: Query<&ChildOf>,
    time: Res<Time>,
) {
    for (material_entity, handle) in &handles {
        let mut parent = parents.get(material_entity).unwrap().parent();

        while let Ok(next) = parents.get(parent) {
            parent = next.parent();
        }

        // Root level parent ought to be the player
        if parent != trigger.target() {
            continue;
        }

        let material = materials.get_mut(handle).unwrap();
        let ColorShift(color, duration) = trigger.event();
        material.extension.color_shift = (*color).into();
        material.extension.color_shift_end = time.elapsed_secs() + *duration as f32 / FPS;
    }
}
