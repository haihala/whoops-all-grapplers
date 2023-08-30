use bevy::prelude::*;
use bevy_hanabi::EffectSpawner;
use std::collections::HashMap;

use wag_core::VisualEffect;

#[derive(Debug)]
pub struct ParticleRequest {
    pub effect: VisualEffect,
    pub position: Vec3,
}

#[derive(Debug, Resource)]
pub struct Particles {
    handles: HashMap<VisualEffect, Entity>,
    queue: Vec<ParticleRequest>,
}
impl Particles {
    pub fn new(handles: HashMap<VisualEffect, Entity>) -> Particles {
        Particles {
            handles,
            queue: vec![],
        }
    }

    fn get(&self, key: VisualEffect) -> Entity {
        self.handles.get(&key).unwrap().to_owned()
    }

    pub fn spawn(&mut self, request: ParticleRequest) {
        self.queue.push(request);
    }
}

pub fn handle_requests(
    mut transforms: Query<(&mut EffectSpawner, &mut Transform)>,
    mut particles: ResMut<Particles>,
) {
    for ParticleRequest { effect, position } in
        particles.queue.drain(..).collect::<Vec<_>>().into_iter()
    {
        let emitter = particles.get(effect);
        if let Ok((mut spawner, mut tf)) = transforms.get_mut(emitter) {
            tf.translation = position;
            spawner.reset();
        }
    }
}
