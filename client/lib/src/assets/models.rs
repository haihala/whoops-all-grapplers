use bevy::prelude::*;
use std::collections::HashMap;
use wag_core::Model;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    vec![
        (Model::Dummy, "dummy.glb#Scene0"),
        (Model::Fireball, "fireball.glb#Scene0"),
    ]
    .into_iter()
    .collect()
}
