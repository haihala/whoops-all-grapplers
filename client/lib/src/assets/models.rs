use bevy::{pbr::ExtendedMaterial, prelude::*, scene::SceneInstance, utils::HashMap};
use characters::FlashRequest;
use wag_core::{Joint, Joints, Model};

use crate::player::{ExtendedFlashMaterial, FlashMaterial};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
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

// From https://github.com/bevyengine/bevy/discussions/8533
#[allow(clippy::too_many_arguments)]
pub fn prep_player_gltf(
    unloaded_instances: Query<(
        Entity,
        &Parent,
        Option<&Name>,
        &SceneInstance,
        &PlayerModelHook,
    )>,
    material_handles: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    mut cmds: Commands,

    mut joints: Query<&mut Joints>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for (entity, parent, name, instance, update_material) in &unloaded_instances {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<PlayerModelHook>();
            dbg!("Scene is ready");
            assign_joints(
                name.cloned().unwrap_or_default().as_str(),
                entity,
                **parent,
                &mut joints,
                &children,
                &names,
            );
            dbg!("Joints are ready");
        }

        // Iterate over all entities in scene (once it's loaded)
        for (entity, material_handle, name) in
            material_handles.iter_many(scene_manager.iter_instance_entities(**instance))
        {
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

            cmds.entity(entity)
                .insert(material.clone())
                .remove::<Handle<StandardMaterial>>();
        }
    }
}

fn assign_joints(
    name: &str,
    entity: Entity,
    root: Entity,
    joints: &mut Query<&mut Joints>,
    children: &Query<&Children>,
    names: &Query<&Name>,
) {
    if let Some(joint) = Joint::from_model_string(name) {
        if let Ok(mut joints) = joints.get_mut(root) {
            joints.nodes.insert(joint, entity);
        }
    }

    if let Ok(direct_children) = children.get(entity) {
        for child in direct_children {
            let child_name = names.get(*child).cloned().unwrap_or_default();

            assign_joints(child_name.as_str(), *child, root, joints, children, names);
        }
    }
}
