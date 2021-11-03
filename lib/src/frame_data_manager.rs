use bevy::prelude::*;
use bevy::utils::HashMap;

use moves::FrameData;
use player_state::{AnimationEvent, PlayerState, StateEvent};
use types::MoveType;

use crate::{clock::run_max_once_per_combat_frame, damage::HitboxManager};

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
                .with_run_criteria(run_max_once_per_combat_frame.system())
                .with_system(animation.system()),
        );
    }
}

fn animation(mut query: Query<(&mut FrameDataManager, &mut HitboxManager, &mut PlayerState)>) {
    for (mut bank, mut hurtbox_generator, mut state) in query.iter_mut() {
        if let Some(active_id) = bank.active {
            let active_move = bank.registered.get(&active_id).unwrap();
            let events = state.get_events();
            if events.is_empty() && !state.animation_in_progress() {
                state.start_animation(*active_move);
            } else {
                for event in events {
                    match event {
                        StateEvent::AnimationUpdate(new_phase) => match new_phase {
                            AnimationEvent::StartActive => {
                                hurtbox_generator.spawn(active_id);
                                state.consume_event(event);
                            }
                            AnimationEvent::EndActive => {
                                hurtbox_generator.despawn(active_id);
                                state.consume_event(event);
                            }
                            AnimationEvent::Recovered => {
                                bank.active = None;
                                state.consume_event(event);
                            }
                            AnimationEvent::Null => panic!("Null animation event"),
                        },
                        StateEvent::Null => panic!("Null event"),
                        _ => {}
                    }
                }
            }
        }
    }
}
