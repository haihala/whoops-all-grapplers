use bevy::{gltf::Gltf, prelude::*};

use super::AnimationRequest;

/// You need to wait for gltf models to be loaded before they can be spawned.
/// Instead of doing that in sync, you can add this component
/// Component is removed when the spawning is done, so it's an easy way to see if all models have been loaded.
#[derive(Debug, Component)]
pub struct ModelRequest {
    pub model: Handle<Gltf>,
    pub animation: Option<(&'static str, bool)>,
}

pub fn model_spawner(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    query: Query<(Entity, &ModelRequest)>,
) {
    for (entity, request) in query.iter() {
        if let Some(gltf) = assets.get(&request.model) {
            let mut e = commands.entity(entity);
            e.with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
            e.remove::<ModelRequest>();

            if let Some((animation_name, looping)) = request.animation {
                e.insert(AnimationRequest {
                    looping,
                    animation: gltf.named_animations[animation_name].clone(),
                });
            }
        }
    }
}
