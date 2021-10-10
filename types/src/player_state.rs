use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationState {
    Startup(usize),
    Active(usize),
    Recovery(usize),
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
enum GroundedState {
    Ground,
    Air,
}
use bevy::prelude::*;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub struct PlayerState {
    animation: Option<AnimationState>,
    grounded: GroundedState,
    facing: Facing,
    stun: Option<usize>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            animation: None,
            grounded: GroundedState::Ground,
            facing: Facing::Right,
            stun: None,
        }
    }
}
impl PlayerState {
    pub fn can_act(&self) -> bool {
        self.animation.is_none() && !self.stunned()
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

    pub fn start_animation(&mut self, progress_frame: usize) {
        self.animation = Some(AnimationState::Startup(progress_frame));
    }
    pub fn start_active(&mut self, progress_frame: usize) {
        self.animation = Some(AnimationState::Active(progress_frame))
    }
    pub fn start_recovery(&mut self, progress_frame: usize) {
        self.animation = Some(AnimationState::Recovery(progress_frame))
    }
    pub fn recover_animation(&mut self) {
        self.animation = None
    }
    pub fn animation_state(&self) -> Option<AnimationState> {
        self.animation
    }

    pub fn flipped(&self) -> bool {
        self.facing == Facing::Left
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        if flipped {
            self.facing = Facing::Left;
        } else {
            self.facing = Facing::Right;
        }
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            match self.facing {
                Facing::Right => 1.0,
                Facing::Left => -1.0,
            },
            0.0,
            0.0,
        )
    }

    pub fn stun(&mut self, recovery_frame: usize) {
        self.stun = Some(recovery_frame);
        self.animation = None;
    }
    pub fn stunned(&self) -> bool {
        self.stun.is_some()
    }
    pub fn stunned_until(&self) -> Option<usize> {
        self.stun
    }
    pub fn recover_from_hitstun(&mut self) {
        self.stun = None;
    }
}
