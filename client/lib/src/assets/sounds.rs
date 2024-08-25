use bevy::asset::AssetPath;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

use wag_core::SoundEffect;

#[derive(Debug, Resource)]
pub struct Sounds {
    handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
    queue: Vec<Handle<AudioSource>>,
}
impl Sounds {
    pub fn new(handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>) -> Sounds {
        Sounds {
            handles,
            queue: vec![],
        }
    }

    pub fn play(&mut self, key: SoundEffect) {
        if let Some(clips) = self.handles.get(&key) {
            let clip = clips[rand::thread_rng().gen_range(0..clips.len())].clone();
            self.queue.push(clip);
        }
    }
}

pub fn play_queued(
    mut commands: Commands,
    mut sounds: ResMut<Sounds>,
    spawned: Query<(Entity, &AudioSink)>,
) {
    for source in sounds.queue.drain(..) {
        commands.spawn(AudioBundle {
            source,
            ..default()
        });
    }
    for (entity, sink) in &spawned {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn get_sound_paths() -> HashMap<SoundEffect, Vec<impl Into<AssetPath<'static>>>> {
    vec![
        (
            SoundEffect::Whoosh,
            vec!["sound_effects/whoosh.ogg".to_string()],
        ),
        (
            SoundEffect::Block,
            vec!["sound_effects/block.ogg".to_string()],
        ),
        (
            SoundEffect::Hit,
            (1..=3)
                .map(|int| format!("sound_effects/hit{}.ogg", int))
                .collect(),
        ),
        (
            SoundEffect::Clash,
            (1..=2)
                .map(|int| format!("sound_effects/clink{}.ogg", int))
                .collect(),
        ),
        (
            SoundEffect::GlassClink,
            (1..=10)
                .map(|int| format!("sound_effects/glass-{:0>2}.ogg", int))
                .collect(),
        ),
        (
            SoundEffect::PlasticCupFlick,
            (1..=23)
                .map(|int| format!("sound_effects/plastic-cup-flick-{:0>2}.ogg", int))
                .collect(),
        ),
        (
            SoundEffect::PotLidGong,
            (1..=4)
                .map(|int| format!("sound_effects/pot-lid-{:0>2}.ogg", int))
                .collect(),
        ),
        (
            SoundEffect::PlasticCupTap,
            (1..=20)
                .map(|int| format!("sound_effects/plastic-cup-tap-{:0>2}.ogg", int))
                .collect(),
        ),
    ]
    .into_iter()
    .collect()
}
