use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use characters::{AttackHeight, MoveSituation};
use types::{Area, Facing, StickPosition};

use crate::sub_state::{AirState, CrouchState, StandState};

#[derive(Inspectable, Debug, Component, Clone)]
enum MainState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
}

#[derive(Inspectable, Debug, Component, Clone)]
pub struct PlayerState {
    main: MainState,
    pub free_since: Option<usize>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            main: MainState::Stand(StandState::default()),
            free_since: None,
        }
    }
}
impl PlayerState {
    pub fn reset(&mut self) {
        *self = PlayerState::default();
    }

    // Moves
    pub fn start_move(&mut self, situation: MoveSituation) {
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Move(situation)),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Move(situation)),
            MainState::Air(_) => MainState::Air(AirState::Move(situation)),
        };
        self.free_since = None;
    }
    pub fn get_move_state(&self) -> Option<&MoveSituation> {
        match self.main {
            MainState::Stand(StandState::Move(ref situation))
            | MainState::Crouch(CrouchState::Move(ref situation))
            | MainState::Air(AirState::Move(ref situation)) => Some(situation),
            _ => None,
        }
    }
    pub fn get_move_state_mut(&mut self) -> Option<&mut MoveSituation> {
        match self.main {
            MainState::Stand(StandState::Move(ref mut situation))
            | MainState::Crouch(CrouchState::Move(ref mut situation))
            | MainState::Air(AirState::Move(ref mut situation)) => Some(situation),
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
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Stun(recovery_frame)),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Stun(recovery_frame)),
            MainState::Air(_) => MainState::Air(AirState::Freefall),
        };
        self.free_since = None;
    }
    pub fn throw(&mut self) {
        self.main = MainState::Air(AirState::Freefall);
        self.free_since = None;
    }
    pub fn recover(&mut self, frame: usize) {
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Idle),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Idle),
            MainState::Air(_) => MainState::Air(AirState::Idle),
        };
        self.free_since = Some(frame);
    }
    pub fn unstun_frame(&self) -> Option<usize> {
        match self.main {
            MainState::Stand(StandState::Stun(frame))
            | MainState::Crouch(CrouchState::Stun(frame)) => Some(frame.to_owned()),
            _ => None,
        }
    }
    pub fn stunned(&self) -> bool {
        matches!(
            self.main,
            MainState::Stand(StandState::Stun(_))
                | MainState::Crouch(CrouchState::Stun(_))
                | MainState::Air(AirState::Freefall)
        )
    }

    // Jumping
    pub fn jump(&mut self) {
        match self.main.clone() {
            MainState::Stand(StandState::Move(situation))
            | MainState::Crouch(CrouchState::Move(situation)) => {
                self.main = MainState::Air(AirState::Move(situation))
            }
            MainState::Crouch(_) | MainState::Stand(_) => {
                self.main = MainState::Air(AirState::Idle)
            }
            _ => {}
        };
    }
    pub fn launch(&mut self) {
        self.main = MainState::Air(AirState::Freefall);
        self.free_since = None;
    }
    pub fn land(&mut self) {
        if matches!(self.main, MainState::Air(AirState::Freefall)) {
            // TODO: Better handling of what happens on landing
            // Recovery?
            // Put the player in a groundactivity otg or something and pick up on that in a recovery system
        }
        self.main = MainState::Crouch(CrouchState::Idle);
    }
    pub fn is_grounded(&self) -> bool {
        matches!(self.main, MainState::Stand(_) | MainState::Crouch(_))
    }

    // Walking
    pub fn walk(&mut self, direction: Facing) {
        self.main = MainState::Stand(StandState::Walk(direction));
    }
    pub fn get_walk_direction(&self) -> Option<Facing> {
        if let MainState::Stand(StandState::Walk(direction)) = self.main {
            Some(direction.to_owned())
        } else {
            None
        }
    }

    pub fn crouch(&mut self) {
        self.main = MainState::Crouch(CrouchState::Idle);
    }
    pub fn stand(&mut self) {
        self.main = MainState::Stand(StandState::Idle);
    }
    pub fn is_crouching(&self) -> bool {
        matches!(self.main, MainState::Crouch(_))
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
