use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::HashMap;
use uuid::Uuid;

use crate::character::PlayerState;
use crate::clock::Clock;
use crate::damage::HitboxManager;

// For now, this is mostly just to invalidate infinite hurtbox spam

pub struct Animation {
    active_start: usize,
    recovery_start: usize,
    recovered: usize,
}
impl Animation {
    pub fn new(startup: usize, active: usize, recovery: usize) -> Self {
        Self {
            active_start: startup,
            recovery_start: startup + active,
            recovered: startup + active + recovery,
        }
    }
}

#[derive(Default)]
pub struct AnimationBank {
    registered: HashMap<Uuid, Animation>,
    active: Option<Uuid>,
    start_frame: usize,
}

impl AnimationBank {
    pub fn register(&mut self, id: Uuid, animation: Animation) {
        self.registered.insert(id, animation);
    }

    pub fn start(&mut self, id: Uuid, frame: usize) {
        // Not strictly necessary but may cause an oopsie in the future if left out
        self.interrupt();

        self.active = Some(id);
        self.start_frame = frame;
    }

    pub fn interrupt(&mut self) {
        self.active = None;
    }
}
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animation.system()).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(crate::FPS as f64))
                .with_system(animation.system()),
        );
    }
}

fn animation(
    clock: Res<Clock>,
    mut query: Query<(&mut AnimationBank, &mut HitboxManager, &mut PlayerState)>,
) {
    for (mut bank, mut hurtbox_generator, mut state) in query.iter_mut() {
        if let Some(active_id) = bank.active {
            let active_animation = bank.registered.get(&active_id).unwrap();
            match *state {
                PlayerState::Startup => {
                    if clock.frame >= active_animation.active_start + bank.start_frame {
                        hurtbox_generator.spawn(active_id);
                        *state = PlayerState::Active;
                    }
                }
                PlayerState::Active => {
                    if clock.frame >= active_animation.recovery_start + bank.start_frame {
                        hurtbox_generator.despawn(active_id);
                        *state = PlayerState::Recovery;
                    }
                }
                PlayerState::Recovery => {
                    if clock.frame >= active_animation.recovered + bank.start_frame {
                        bank.active = None;
                        state.recover();
                    }
                }
                _ => {}
            }
        }
    }
}
