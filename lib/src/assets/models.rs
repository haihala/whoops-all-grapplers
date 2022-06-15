use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

use super::AnimationRequest;

#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Model {
    Dummy,
}

#[derive(Debug, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Gltf>>);

#[derive(Debug, Component)]
pub struct ModelRequest {
    pub model: Model,
    pub animation: Option<(&'static str, bool)>,
}

pub fn model_spawner(
    mut commands: Commands,
    models: Res<Models>,
    assets: Res<Assets<Gltf>>,
    query: Query<(Entity, &ModelRequest)>,
) {
    for (entity, request) in query.iter() {
        let model_handle = models[&request.model].clone();
        if let Some(gltf) = assets.get(&model_handle) {
            // Asset has been loaded

            // Spawn the model as a child
            let mut e = commands.entity(entity);
            e.with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
            e.remove::<ModelRequest>(); // Prevent multiple spawns by removing the spawn request

            // If an animation was requested, put that on.
            if let Some((animation_name, looping)) = request.animation {
                e.insert(AnimationRequest {
                    looping,
                    animation: gltf.named_animations[animation_name].clone(),
                });
            }
        }
    }
}
