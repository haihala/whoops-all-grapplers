use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use characters::{AttackHeight, MoveSituation};
use types::{Area, Facing, StickPosition};

use crate::sub_state::{AirActivity, GroundActivity};

#[derive(Inspectable, Debug, Component)]
pub enum PlayerState {
    Air(AirActivity),
    Ground(GroundActivity),
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Ground(GroundActivity::Standing)
    }
}
impl PlayerState {
    pub fn reset(&mut self) {
        *self = PlayerState::default();
    }

    // Moves
    pub fn start_move(&mut self, situation: MoveSituation) {
        *self = match self {
            Self::Ground(_) => Self::Ground(GroundActivity::Move(situation)),
            Self::Air(_) => Self::Air(AirActivity::Move(situation)),
        };
    }
    pub fn get_move_state(&self) -> Option<&MoveSituation> {
        match self {
            Self::Ground(GroundActivity::Move(ref situation))
            | Self::Air(AirActivity::Move(ref situation)) => Some(situation),
            _ => None,
        }
    }
    pub fn get_move_state_mut(&mut self) -> Option<&mut MoveSituation> {
        match self {
            Self::Ground(GroundActivity::Move(ref mut situation))
            | Self::Air(AirActivity::Move(ref mut situation)) => Some(situation),
            _ => None,
        }
    }

    pub fn register_hit(&mut self) {
        if let Self::Ground(GroundActivity::Move(ref mut situation))
        | Self::Air(AirActivity::Move(ref mut situation)) = self
        {
            situation.hit_registered = true;
        }
    }

    // Stun
    pub fn stun(&mut self, recovery_frame: usize) {
        match self {
            Self::Ground(_) => {
                *self = Self::Ground(GroundActivity::Stun(recovery_frame));
            }
            Self::Air(_) => {
                *self = Self::Air(AirActivity::Freefall);
            }
        }
    }
    pub fn throw(&mut self) {
        *self = Self::Air(AirActivity::Freefall);
    }
    pub fn recover(&mut self) {
        if self.is_grounded() {
            *self = Self::Ground(GroundActivity::Standing);
        } else {
            *self = Self::Air(AirActivity::Idle);
        }
    }
    pub fn unstun_frame(&self) -> Option<usize> {
        if let Self::Ground(GroundActivity::Stun(frame)) = self {
            Some(frame.to_owned())
        } else {
            None
        }
    }
    pub fn stunned(&self) -> bool {
        matches!(
            self,
            Self::Ground(GroundActivity::Stun(_)) | Self::Air(AirActivity::Freefall)
        )
    }

    // Jumping
    pub fn jump(&mut self) {
        if let Self::Ground(GroundActivity::Move(situation)) = self {
            *self = Self::Air(AirActivity::Move(situation.to_owned()));
        } else if self.is_grounded() {
            // was grounded doing something else, now in air without a move
            *self = Self::Air(AirActivity::Idle);
        }
    }
    pub fn launch(&mut self) {
        *self = Self::Air(AirActivity::Freefall);
    }
    pub fn land(&mut self) {
        if matches!(self, Self::Air(AirActivity::Freefall)) {
            // TODO: Better handling of what happens on landing
            // Recovery?
            // Put the player in a groundactivity otg or something and pick up on that in a recovery system
        }
        *self = Self::Ground(GroundActivity::Standing);
    }
    pub fn is_grounded(&self) -> bool {
        matches!(self, Self::Ground(_))
    }

    // Walking
    pub fn walk(&mut self, direction: Facing) {
        *self = Self::Ground(GroundActivity::Walk(direction));
    }
    pub fn get_walk_direction(&self) -> Option<Facing> {
        if let Self::Ground(GroundActivity::Walk(direction)) = self {
            Some(direction.to_owned())
        } else {
            None
        }
    }

    pub fn crouch(&mut self) {
        *self = Self::Ground(GroundActivity::Crouching);
    }
    pub fn stand(&mut self) {
        *self = Self::Ground(GroundActivity::Standing);
    }
    pub fn is_crouching(&self) -> bool {
        matches!(self, Self::Ground(GroundActivity::Crouching))
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
