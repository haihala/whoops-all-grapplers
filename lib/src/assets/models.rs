use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use types::Model;

#[derive(Debug, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Gltf>>);

#[derive(Debug, Component, Deref, DerefMut)]
pub struct ModelRequest(pub Model);

pub fn model_spawner(
    mut commands: Commands,
    models: Res<Models>,
    assets: Res<Assets<Gltf>>,
    query: Query<(Entity, &ModelRequest)>,
) {
    for (entity, request) in query.iter() {
        let model_handle = models[request].clone();
        if let Some(gltf) = assets.get(&model_handle) {
            // Asset has been loaded

            // Spawn the model as a child
            let mut e = commands.entity(entity);
            e.with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
            e.remove::<ModelRequest>(); // Prevent multiple spawns by removing the spawn request
        }
    }
}

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    vec![(Model::Dummy, "dummy-character.glb")]
        .into_iter()
        .collect()
}
