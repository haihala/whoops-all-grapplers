use bevy::prelude::*;
use map_macro::map;
use std::collections::HashMap;
use types::Model;

#[derive(Debug, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    map! {
        Model::Dummy => "dummy.glb#Scene0",
    }
}
