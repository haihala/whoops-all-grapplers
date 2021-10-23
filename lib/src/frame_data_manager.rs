use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::HashMap;

use moves::FrameData;
use types::{AnimationPhase, MoveType, PlayerState};

use crate::damage::HitboxManager;
use crate::Clock;

#[derive(Default)]
pub struct FrameDataManager {
    registered: HashMap<MoveType, FrameData>,
    active: Option<MoveType>,
    start_frame: usize,
}
impl FrameDataManager {
    pub fn load(target: HashMap<MoveType, FrameData>) -> FrameDataManager {
        FrameDataManager {
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
                .with_run_criteria(FixedTimestep::steps_per_second(constants::FPS_F64))
                .with_system(animation.system()),
        );
    }
}

fn animation(
    clock: Res<Clock>,
    mut query: Query<(&mut FrameDataManager, &mut HitboxManager, &mut PlayerState)>,
) {
    for (mut bank, mut hurtbox_generator, mut state) in query.iter_mut() {
        if let Some(active_id) = bank.active {
            let active_move = bank.registered.get(&active_id).unwrap();
            if state.animation_state().is_none() {
                state.start_animation(clock.frame + active_move.active_start)
            } else {
                match state.animation_state().unwrap() {
                    AnimationPhase::Startup(progress_frame) => {
                        if clock.frame >= progress_frame {
                            hurtbox_generator.spawn(active_id);
                            state.start_active(clock.frame + active_move.recovery_start);
                        }
                    }
                    AnimationPhase::Active(progress_frame) => {
                        if clock.frame >= progress_frame {
                            hurtbox_generator.despawn(active_id);
                            state.start_recovery(clock.frame + active_move.recovery_start);
                        }
                    }
                    AnimationPhase::Recovery(progress_frame) => {
                        if clock.frame >= progress_frame {
                            bank.active = None;
                            state.recover_animation();
                        }
                    }
                }
            }
        }
    }
}
