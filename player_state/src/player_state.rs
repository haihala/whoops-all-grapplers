use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use characters::{AttackHeight, MoveSituation};
use types::{Area, Facing, StickPosition};

use crate::sub_state::{AirState, CrouchState, StandState};

#[derive(Inspectable, Debug, Component, Clone)]
pub enum PlayerState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Stand(StandState::default())
    }
}
impl PlayerState {
    pub fn reset(&mut self) {
        *self = PlayerState::default();
    }

    // Moves
    pub fn start_move(&mut self, situation: MoveSituation) {
        *self = match self {
            Self::Stand(_) => Self::Stand(StandState::Move(situation)),
            Self::Crouch(_) => Self::Crouch(CrouchState::Move(situation)),
            Self::Air(_) => Self::Air(AirState::Move(situation)),
        };
    }
    pub fn get_move_state(&self) -> Option<&MoveSituation> {
        match self {
            Self::Stand(StandState::Move(ref situation))
            | Self::Crouch(CrouchState::Move(ref situation))
            | Self::Air(AirState::Move(ref situation)) => Some(situation),
            _ => None,
        }
    }
    pub fn get_move_state_mut(&mut self) -> Option<&mut MoveSituation> {
        match self {
            Self::Stand(StandState::Move(ref mut situation))
            | Self::Crouch(CrouchState::Move(ref mut situation))
            | Self::Air(AirState::Move(ref mut situation)) => Some(situation),
            _ => None,
        }
    }

    pub fn register_hit(&mut self) {
        if let Some(ref mut situation) = self.get_move_state_mut() {
            situation.hit_registered = true;
        }
    }

    // Stun
    pub fn stun(&mut self, recovery_frame: usize) {
        *self = match self {
            Self::Stand(_) => Self::Stand(StandState::Stun(recovery_frame)),
            Self::Crouch(_) => Self::Crouch(CrouchState::Stun(recovery_frame)),
            Self::Air(_) => Self::Air(AirState::Freefall),
        }
    }
    pub fn throw(&mut self) {
        *self = Self::Air(AirState::Freefall);
    }
    pub fn recover(&mut self) {
        *self = match self {
            Self::Stand(_) => Self::Stand(StandState::Idle),
            Self::Crouch(_) => Self::Crouch(CrouchState::Idle),
            Self::Air(_) => Self::Air(AirState::Idle),
        }
    }
    pub fn unstun_frame(&self) -> Option<usize> {
        match self {
            Self::Stand(StandState::Stun(frame)) | Self::Crouch(CrouchState::Stun(frame)) => {
                Some(frame.to_owned())
            }
            _ => None,
        }
    }
    pub fn stunned(&self) -> bool {
        matches!(
            self,
            Self::Stand(StandState::Stun(_))
                | Self::Crouch(CrouchState::Stun(_))
                | Self::Air(AirState::Freefall)
        )
    }

    // Jumping
    pub fn jump(&mut self) {
        match self {
            Self::Stand(StandState::Move(situation))
            | Self::Crouch(CrouchState::Move(situation)) => {
                *self = Self::Air(AirState::Move(situation.to_owned()))
            }
            Self::Crouch(_) | Self::Stand(_) => *self = Self::Air(AirState::Idle),
            _ => {}
        };
    }
    pub fn launch(&mut self) {
        *self = Self::Air(AirState::Freefall);
    }
    pub fn land(&mut self) {
        if matches!(self, Self::Air(AirState::Freefall)) {
            // TODO: Better handling of what happens on landing
            // Recovery?
            // Put the player in a groundactivity otg or something and pick up on that in a recovery system
        }
        *self = Self::Crouch(CrouchState::Idle);
    }
    pub fn is_grounded(&self) -> bool {
        matches!(self, Self::Stand(_) | Self::Crouch(_))
    }

    // Walking
    pub fn walk(&mut self, direction: Facing) {
        *self = Self::Stand(StandState::Walk(direction));
    }
    pub fn get_walk_direction(&self) -> Option<Facing> {
        if let Self::Stand(StandState::Walk(direction)) = self {
            Some(direction.to_owned())
        } else {
            None
        }
    }

    pub fn crouch(&mut self) {
        *self = Self::Crouch(CrouchState::Idle);
    }
    pub fn stand(&mut self) {
        *self = Self::Stand(StandState::Idle);
    }
    pub fn is_crouching(&self) -> bool {
        matches!(self, Self::Crouch(_))
    }

    pub fn blocked(
        &self,
        fixed_height: Option<AttackHeight>,
        hitbox: Area,
        low_threshold: f32,
        high_threshold: f32,
        stick: StickPosition,
    ) -> bool {
        if !self.can_block_now() {
            return false;
        }

        let blocking_high = stick == StickPosition::W;
        let blocking_low = stick == StickPosition::SW;

        let height = fixed_height.unwrap_or(if hitbox.bottom() > high_threshold {
            AttackHeight::High
        } else if hitbox.top() > low_threshold {
            AttackHeight::Mid
        } else {
            AttackHeight::Low
        });

        match height {
            AttackHeight::Low => blocking_low,
            AttackHeight::Mid => blocking_low || blocking_high,
            AttackHeight::High => blocking_high,
        }
    }
    fn can_block_now(&self) -> bool {
        self.get_move_state().is_none() && self.is_grounded()
    }
}
