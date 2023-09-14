use bevy::prelude::*;
use std::collections::HashMap;
use wag_core::{Joint, Joints, Model};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    vec![
        (Model::Dummy, "dummy.glb#Scene0"),
        (Model::Fireball, "fireball.glb#Scene0"),
        (Model::TrainingStage, "stage.glb#Scene0"),
    ]
    .into_iter()
    .collect()
}

pub(super) fn find_joints(
    mut joints: Query<(Entity, &mut Joints)>,
    named_nodes: Query<(Entity, &Name)>,
    parents: Query<&Parent>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    let loaded_joints: Vec<_> = named_nodes
        .into_iter()
        .filter_map(|(entity, name)| Joint::from_model_string(name).map(|joint| (entity, joint)))
        .collect();

    let mut all_done = true;
    for (root_entity, mut joints) in &mut joints {
        joints.nodes.extend(
            loaded_joints
                .clone()
                .into_iter()
                .filter_map(|(entity, joint)| {
                    if is_child_of(&parents, entity, root_entity) {
                        Some((joint, entity))
                    } else {
                        None
                    }
                }),
        );
        if joints.nodes.is_empty() {
            all_done = false;
        }
    }

    if all_done {
        *done = true;
    }
}

fn is_child_of(query: &Query<&Parent>, start: Entity, target: Entity) -> bool {
    if start == target {
        true
    } else {
        let mut cursor = start;
        while let Ok(parent) = query.get(cursor) {
            if **parent == target {
                return true;
            }
            cursor = **parent;
        }
        false
    }
}
