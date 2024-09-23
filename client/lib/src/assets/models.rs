use bevy::{
    pbr::ExtendedMaterial, prelude::*, render::view::NoFrustumCulling, scene::SceneInstance,
    utils::HashMap,
};
use characters::FlashRequest;
use wag_core::Model;

use super::{ExtendedFlashMaterial, FlashMaterial};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    // TODO: This could use the bevy 0.14 typed asset handles instead of static strings
    // So far, I think that is a waste of effort.
    vec![
        (Model::Dummy, "dummy.glb#Scene0"),
        (Model::Mizku, "mizuki.glb#Scene0"),
        (Model::Fireball, "fireball.glb#Scene0"),
        (Model::Kunai, "kunai.glb#Scene0"),
        (Model::TrainingStage, "stage.glb#Scene0"),
    ]
    .into_iter()
    .collect()
}

#[derive(Component, Debug)]
pub struct PlayerModelHook(pub HashMap<&'static str, Color>);

#[allow(clippy::too_many_arguments)]
pub fn prep_player_gltf(
    unloaded_instances: Query<(Entity, &SceneInstance, &PlayerModelHook)>,
    material_handles: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    mut commands: Commands,
) {
    for (entity, instance, update_material) in &unloaded_instances {
        if scene_manager.instance_is_ready(**instance) {
            commands.entity(entity).remove::<PlayerModelHook>();
        }

        // Iterate over all entities in scene (once it's loaded)
        for (entity, material_handle, name) in
            material_handles.iter_many(scene_manager.iter_instance_entities(**instance))
        {
            commands.entity(entity).insert(NoFrustumCulling);

            let Some(old_material) = pbr_materials.get(material_handle) else {
                continue;
            };

            let mut base_material = old_material.clone();

            if let Some(color) = update_material.0.get(name.as_str()) {
                base_material.base_color = *color;
            }

            let material = materials.add(ExtendedMaterial {
                base: base_material,
                extension: FlashMaterial::from_request(FlashRequest::default(), 0.0),
            });

            commands
                .entity(entity)
                .insert(material.clone())
                .remove::<Handle<StandardMaterial>>();
        }
    }
}
