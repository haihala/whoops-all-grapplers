use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{AbsoluteDirection, DashPhase, DashState, RelativeDirection};
use std::fmt::Debug;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum FreedomLevel {
    Stunned,
    Busy,
    LightBusy,
    Free,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationPhase {
    Startup(usize),
    Active(usize),
    Recovery(usize),
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
enum GroundedState {
    Ground,
    Air,
}
impl Default for GroundedState {
    fn default() -> Self {
        GroundedState::Ground
    }
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum JumpDirection {
    Neutral,
    Diagonal(AbsoluteDirection),
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum StunType {
    Basic(usize),
    FreeFall,
}

#[derive(Inspectable, PartialEq, Clone, Copy, Debug, Default)]
pub struct PlayerState {
    animation: Option<AnimationPhase>,
    dash: Option<DashState>,
    facing: AbsoluteDirection,
    grounded: GroundedState,
    jumping: Option<JumpDirection>,
    stun: Option<StunType>,
    walking: Option<RelativeDirection>,
}
impl PlayerState {
    pub fn freedom_level(&self, current_frame: usize) -> FreedomLevel {
        if self.stunned() {
            FreedomLevel::Stunned
        } else if self.animation.is_some()
            || (self.dash.is_some()
                && self.dash.unwrap().get_phase(current_frame) == Some(DashPhase::Start))
        {
            FreedomLevel::Busy
        } else if self.dash.is_some()
            && self.dash.unwrap().get_phase(current_frame) == Some(DashPhase::Recovery)
        {
            FreedomLevel::LightBusy
        } else {
            FreedomLevel::Free
        }
    }

    fn clear(&mut self) {
        // Reset temporary state
        self.animation = None;
        self.dash = None;
        self.jumping = None;
        self.stun = None;
        self.walking = None;
    }

    // Animation
    pub fn start_animation(&mut self, progress_frame: usize) {
        self.clear();
        self.animation = Some(AnimationPhase::Startup(progress_frame));
    }
    pub fn start_active(&mut self, progress_frame: usize) {
        self.animation = Some(AnimationPhase::Active(progress_frame))
    }
    pub fn start_recovery(&mut self, progress_frame: usize) {
        self.animation = Some(AnimationPhase::Recovery(progress_frame))
    }
    pub fn recover_animation(&mut self) {
        self.animation = None
    }
    pub fn animation_state(&self) -> Option<AnimationPhase> {
        self.animation
    }

    // Dash
    pub fn start_dash(&mut self, direction: RelativeDirection, current_frame: usize) {
        self.clear();
        self.dash = Some(DashState::new(direction, current_frame))
    }
    pub fn get_dash_phase(&mut self, current_frame: usize) -> Option<DashPhase> {
        if let Some(dash) = self.dash {
            if let Some(phase) = dash.get_phase(current_frame) {
                Some(phase)
            } else {
                self.dash = None;
                None
            }
        } else {
            None
        }
    }
    pub fn get_dash_direction(&mut self) -> Option<Vec3> {
        if let Some(dash) = self.dash {
            Some(dash.direction.handle_mirroring(self.forward()))
        } else {
            None
        }
    }

    // Facing
    pub fn flipped(&self) -> bool {
        self.facing == AbsoluteDirection::Left
    }
    pub fn set_flipped(&mut self, flipped: bool) {
        if flipped {
            self.facing = AbsoluteDirection::Left;
        } else {
            self.facing = AbsoluteDirection::Right;
        }
    }
    pub fn forward(&self) -> Vec3 {
        self.facing.to_vec3()
    }

    // Jumping
    pub fn land(&mut self) {
        self.grounded = GroundedState::Ground;
    }
    pub fn register_jump(&mut self, direction: Option<RelativeDirection>) {
        self.clear();
        self.grounded = GroundedState::Air;
        self.jumping = Some(match direction {
            Some(relative_direction) => {
                JumpDirection::Diagonal(relative_direction.as_absolute(self.facing))
            }
            None => JumpDirection::Neutral,
        })
    }
    pub fn get_jump_impulse(&mut self) -> Option<Vec3> {
        match self.jumping {
            Some(jump_direction) => {
                self.jumping = None;

                Some(match jump_direction {
                    JumpDirection::Neutral => constants::NEUTRAL_JUMP_VECTOR.into(),
                    JumpDirection::Diagonal(direction) => {
                        direction.handle_mirroring(constants::DIAGONAL_JUMP_VECTOR.into())
                    }
                })
            }
            None => None,
        }
    }
    pub fn clear_jump(&mut self) {
        self.jumping = None;
    }
    pub fn is_grounded(&self) -> bool {
        self.grounded == GroundedState::Ground
    }

    // Stun
    pub fn hit(&mut self, recovery_frame: usize, launching_hit: bool) {
        self.clear();
        self.stun = Some(if !launching_hit && self.is_grounded() {
            StunType::Basic(recovery_frame)
        } else {
            self.grounded = GroundedState::Air;
            StunType::FreeFall
        });
    }
    pub fn stunned(&self) -> bool {
        self.stun.is_some()
    }
    pub fn stunned_until(&self) -> Option<usize> {
        match self.stun {
            Some(kind) => match kind {
                StunType::Basic(frames) => Some(frames),
                StunType::FreeFall => None,
            },
            None => None,
        }
    }
    pub fn recover_from_hitstun(&mut self) {
        self.stun = None;
    }

    // Walking
    pub fn walk(&mut self, direction: RelativeDirection) {
        self.walking = Some(direction);
    }
    pub fn stop_walking(&mut self) {
        self.walking = None;
    }
    pub fn get_walk_direction(&self) -> Option<Vec3> {
        if let Some(direction) = self.walking {
            Some(direction.handle_mirroring(self.forward()))
        } else {
            None
        }
    }
}
