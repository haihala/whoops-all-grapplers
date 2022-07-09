use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_hanabi::ParticleEffect;

use types::VisualEffect;

#[derive(Debug)]
pub struct ParticleRequest {
    pub effect: VisualEffect,
    pub position: Vec3,
}

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
    mut transforms: Query<(&mut ParticleEffect, &mut Transform)>,
    particles: Option<ResMut<Particles>>,
) {
    if let Some(mut particles) = particles {
        for ParticleRequest { effect, position } in
            particles.queue.drain(..).collect::<Vec<_>>().into_iter()
        {
            let emitter = particles.get(effect);
            if let Ok((mut effect, mut tf)) = transforms.get_mut(emitter) {
                tf.translation = position;
                effect.maybe_spawner().unwrap().reset();
            }
        }
    }
}
