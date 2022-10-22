use bevy::prelude::*;
use core::Model;
use std::collections::HashMap;

#[derive(Debug, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    vec![
        (Model::Dummy, "dummy.glb#Scene0"),
        (Model::Fireball, "fireball.glb#Scene0"),
    ]
    .into_iter()
    .collect()
}
