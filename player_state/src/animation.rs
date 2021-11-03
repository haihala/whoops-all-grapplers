use bevy_inspector_egui::Inspectable;
use moves::FrameData;

use crate::{events::AnimationEvent, StateEvent};

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationPhase {
    Startup(usize),
    Active(usize),
    Recovery(usize),
    Null,
}

impl Default for AnimationPhase {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        AnimationPhase::Null
    }
}

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub struct Animation {
    pub phase: AnimationPhase,
    pub frame_data: FrameData,
    pub start_frame: usize,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            phase: Default::default(),
            frame_data: FrameData::new(0, 0, 0),
            start_frame: Default::default(),
        }
    }
}
impl Animation {
    pub fn new(start_frame: usize, frame_data: FrameData) -> Animation {
        Animation {
            start_frame,
            frame_data,
            phase: AnimationPhase::Startup(start_frame + frame_data.active_start),
        }
    }

    pub fn tick(&mut self, current_frame: usize) -> Option<StateEvent> {
        match self.phase {
            AnimationPhase::Startup(progress_frame) => {
                if progress_frame <= current_frame {
                    self.phase =
                        AnimationPhase::Active(self.start_frame + self.frame_data.recovery_start);

                    Some(StateEvent::AnimationUpdate(AnimationEvent::StartActive))
                } else {
                    None
                }
            }
            AnimationPhase::Active(progress_frame) => {
                if progress_frame <= current_frame {
                    self.phase =
                        AnimationPhase::Recovery(self.start_frame + self.frame_data.recovered);

                    Some(StateEvent::AnimationUpdate(AnimationEvent::EndActive))
                } else {
                    None
                }
            }
            AnimationPhase::Recovery(progress_frame) => {
                if progress_frame <= current_frame {
                    Some(StateEvent::AnimationUpdate(AnimationEvent::Recovered))
                } else {
                    None
                }
            }
            AnimationPhase::Null => panic!("Null animation state"),
        }
    }
}
