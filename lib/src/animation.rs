use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::HashMap;

use moves::{FrameData, MoveType};

use crate::damage::HitboxManager;
use crate::{Clock, PlayerState};

#[derive(Default)]
pub struct AnimationBank {
    registered: HashMap<MoveType, FrameData>,
    active: Option<MoveType>,
    start_frame: usize,
}
impl AnimationBank {
    pub fn load(target: HashMap<MoveType, FrameData>) -> AnimationBank {
        AnimationBank {
            registered: target,
            ..Default::default()
        }
    }

    pub fn start(&mut self, id: MoveType, frame: usize) {
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
