use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use moves::{MoveState, PhaseCondition};
use types::{AttackHeight, LRDirection, MoveId, StickPosition};

use crate::{
    primary_state::{AirActivity, GroundActivity, PrimaryState},
    PLAYER_HIGH_BLOCK_THRESHOLD_RATIO, PLAYER_LOW_BLOCK_THRESHOLD_RATIO,
    PLAYER_SPRITE_CROUCHING_HEIGHT, PLAYER_SPRITE_STANDING_HEIGHT, PLAYER_SPRITE_WIDTH,
};

#[derive(Inspectable, Debug, Component)]
pub struct PlayerState {
    primary: PrimaryState,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            primary: PrimaryState::Ground(GroundActivity::Standing),
        }
    }
}
impl PlayerState {
    // Moves
    pub fn start_move(&mut self, move_id: MoveId, start_frame: usize, situation: PhaseCondition) {
        let move_state = MoveState {
            start_frame,
            move_id,
            situation,
            ..Default::default()
        };

        self.primary = match self.primary {
            PrimaryState::Ground(_) => PrimaryState::Ground(GroundActivity::Move(move_state)),
            PrimaryState::Air(_) => PrimaryState::Air(AirActivity::Move(move_state)),
        };
    }
    pub fn set_move_phase_index(&mut self, phase_index: usize) {
        if let PrimaryState::Ground(GroundActivity::Move(ref mut move_state))
        | PrimaryState::Air(AirActivity::Move(ref mut move_state)) = self.primary
        {
            move_state.phase_index = phase_index;
        } else {
            panic!("Setting phase index without an active move");
        }
    }
    pub fn get_move_state(&self) -> Option<MoveState> {
        match self.primary {
            PrimaryState::Ground(GroundActivity::Move(move_state))
            | PrimaryState::Air(AirActivity::Move(move_state)) => Some(move_state),
            _ => None,
        }
    }
    pub fn register_hit(&mut self) {
        if let PrimaryState::Ground(GroundActivity::Move(ref mut move_state))
        | PrimaryState::Air(AirActivity::Move(ref mut move_state)) = self.primary
        {
            move_state.register_hit();
        }
    }

    // Stun
    pub fn stun(&mut self, recovery_frame: usize) {
        match self.primary {
            PrimaryState::Ground(_) => {
                self.primary = PrimaryState::Ground(GroundActivity::Stun(recovery_frame));
            }
            PrimaryState::Air(_) => {
                self.primary = PrimaryState::Air(AirActivity::Freefall);
            }
        }
    }
    pub fn throw(&mut self) {
        self.primary = PrimaryState::Air(AirActivity::Freefall);
    }
    pub fn recover(&mut self) {
        if self.is_grounded() {
            self.primary = PrimaryState::Ground(GroundActivity::Standing);
        } else {
            self.primary = PrimaryState::Air(AirActivity::Idle);
        }
    }
    pub fn unstun_frame(&self) -> Option<usize> {
        if let PrimaryState::Ground(GroundActivity::Stun(frame)) = self.primary {
            Some(frame)
        } else {
            None
        }
    }
    pub fn stunned(&self) -> bool {
        matches!(
            self.primary,
            PrimaryState::Ground(GroundActivity::Stun(_))
                | PrimaryState::Air(AirActivity::Freefall)
        )
    }

    // Jumping
    pub fn jump(&mut self) {
        if let PrimaryState::Ground(GroundActivity::Move(id)) = self.primary {
            self.primary = PrimaryState::Air(AirActivity::Move(id));
        } else if self.is_grounded() {
            // was grounded doing something else, now in air without a move
            self.primary = PrimaryState::Air(AirActivity::Idle);
        }
    }
    pub fn launch(&mut self) {
        self.primary = PrimaryState::Air(AirActivity::Freefall);
    }
    pub fn land(&mut self) {
        if matches!(self.primary, PrimaryState::Air(AirActivity::Freefall)) {
            // TODO: Better handling of what happens on landing
            // Recovery?
            // Put the player in a groundactivity otg or something and pick up on that in a recovery system
        }
        self.primary = PrimaryState::Ground(GroundActivity::Standing);
    }
    pub fn is_grounded(&self) -> bool {
        matches!(self.primary, PrimaryState::Ground(_))
    }

    // Walking
    pub fn walk(&mut self, direction: LRDirection) {
        self.primary = PrimaryState::Ground(GroundActivity::Walk(direction));
    }
    pub fn get_walk_direction(&self) -> Option<LRDirection> {
        if let PrimaryState::Ground(GroundActivity::Walk(direction)) = self.primary {
            Some(direction)
        } else {
            None
        }
    }

    pub fn crouch(&mut self) {
        self.primary = PrimaryState::Ground(GroundActivity::Crouching);
    }
    pub fn stand(&mut self) {
        self.primary = PrimaryState::Ground(GroundActivity::Standing);
    }
    pub fn is_crouching(&self) -> bool {
        matches!(
            self.primary,
            PrimaryState::Ground(GroundActivity::Crouching)
        )
    }
    pub fn get_height(&self) -> f32 {
        if matches!(self.primary, PrimaryState::Air(AirActivity::Freefall)) {
            PLAYER_SPRITE_WIDTH
        } else if self.is_crouching() {
            PLAYER_SPRITE_CROUCHING_HEIGHT
        } else {
            PLAYER_SPRITE_STANDING_HEIGHT
        }
    }
    pub fn get_width(&self) -> f32 {
        if matches!(self.primary, PrimaryState::Air(AirActivity::Freefall)) {
            PLAYER_SPRITE_STANDING_HEIGHT
        } else {
            PLAYER_SPRITE_WIDTH
        }
    }
    pub fn get_collider_size(&self) -> Vec2 {
        Vec2::new(self.get_width(), self.get_height())
    }

    pub fn blocked(
        &self,
        fixed_height: Option<AttackHeight>,
        attack_height: f32,
        stick: StickPosition,
    ) -> bool {
        if !self.can_block_now() {
            return false;
        }

        let blocking_high = stick == StickPosition::E;
        let blocking_low = stick == StickPosition::SE;

        let height = fixed_height.unwrap_or(if attack_height > self.high_block_threshold() {
            AttackHeight::High
        } else if attack_height > self.low_block_threshold() {
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
    fn low_block_threshold(&self) -> f32 {
        self.get_height() * PLAYER_LOW_BLOCK_THRESHOLD_RATIO
    }
    fn high_block_threshold(&self) -> f32 {
        self.get_height() * PLAYER_HIGH_BLOCK_THRESHOLD_RATIO
    }
}
