use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationState {
    Startup,
    Active,
    Recovery,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
enum GroundedState {
    Ground,
    Air,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub struct PlayerState {
    animation: Option<AnimationState>,
    grounded: GroundedState,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            animation: None,
            grounded: GroundedState::Ground,
        }
    }
}
impl PlayerState {
    pub fn can_act(&self) -> bool {
        self.animation.is_none()
    }

    pub fn land(&mut self) {
        self.grounded = GroundedState::Ground;
    }
    pub fn jump(&mut self) {
        self.grounded = GroundedState::Air;
    }
    pub fn is_grounded(&self) -> bool {
        self.grounded == GroundedState::Ground
    }

    pub fn start_animation(&mut self) {
        self.animation = Some(AnimationState::Startup);
    }
    pub fn start_active(&mut self) {
        self.animation = Some(AnimationState::Active);
    }
    pub fn start_recovery(&mut self) {
        self.animation = Some(AnimationState::Recovery);
    }
    pub fn recover_animation(&mut self) {
        self.animation = None;
    }
    pub fn animation_state(&self) -> Option<AnimationState> {
        self.animation
    }
}
