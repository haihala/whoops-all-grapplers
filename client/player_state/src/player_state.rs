use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use characters::{Action, AttackHeight, FlowControl, MoveHistory, Situation};
use types::{AnimationType, Area, Facing, Status, StatusCondition, StatusEffect, StickPosition};

use crate::sub_state::{AirState, CrouchState, StandState, Stun};

#[derive(Inspectable, Debug, Component, Clone)]
enum MainState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
    Ground(usize),
}

#[derive(Inspectable, Debug, Component, Clone)]
pub struct PlayerState {
    main: MainState,
    pub free_since: Option<usize>,
    conditions: Vec<StatusCondition>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            main: MainState::Stand(StandState::default()),
            free_since: Some(0),
            conditions: vec![],
        }
    }
}
impl PlayerState {
    pub fn reset(&mut self) {
        *self = PlayerState::default();
    }

    pub fn proceed_move(&mut self, situation: Situation) {
        if let Some(ref mut history) = self.get_move_history_mut() {
            if !history.unprocessed_events.is_empty() {
                warn!("Leftover events");
                dbg!(&history.unprocessed_events);
            }

            let new_fcs = situation.new_actions();
            history
                .unprocessed_events
                .extend(new_fcs.clone().into_iter().filter_map(|fc| {
                    if let FlowControl::Action(action) = fc {
                        Some(action)
                    } else {
                        None
                    }
                }));
            history.past.extend(new_fcs.into_iter());
        }
    }

    pub fn current_move_fully_handled(&self) -> Option<bool> {
        self.get_move_history().map(|history| history.is_done())
    }

    pub fn drain_matching_actions<T>(
        &mut self,
        predicate: impl Fn(&mut Action) -> Option<T>,
    ) -> Vec<T> {
        if let Some(ref mut history) = self.get_move_history_mut() {
            history
                .unprocessed_events
                .drain_filter(|action| (predicate)(action).is_some())
                .map(|mut action| (predicate)(&mut action).unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_generic_animation(&self, facing: Facing) -> Option<AnimationType> {
        match self.main {
            MainState::Air(AirState::Idle) => Some(AnimationType::AirIdle),
            MainState::Air(AirState::Freefall) => Some(AnimationType::AirStun),

            MainState::Stand(StandState::Idle) => Some(AnimationType::StandIdle),
            MainState::Stand(StandState::Stun(Stun::Block(_))) => Some(AnimationType::StandBlock),
            MainState::Stand(StandState::Stun(Stun::Hit(_))) => Some(AnimationType::StandStun),
            MainState::Stand(StandState::Walk(dir)) => Some(if facing == dir {
                AnimationType::WalkForward
            } else {
                AnimationType::WalkBack
            }),

            MainState::Crouch(CrouchState::Idle) => Some(AnimationType::CrouchIdle),
            MainState::Crouch(CrouchState::Stun(Stun::Block(_))) => {
                Some(AnimationType::CrouchBlock)
            }
            MainState::Crouch(CrouchState::Stun(Stun::Hit(_))) => Some(AnimationType::CrouchStun),
            MainState::Ground(_) => Some(AnimationType::Getup),
            _ => None,
        }
    }

    // Moves
    pub fn start_move(&mut self, history: MoveHistory) {
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Move(history)),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Move(history)),
            MainState::Air(_) => MainState::Air(AirState::Move(history)),
            MainState::Ground(_) => panic!("Starting a move on the ground"),
        };
        self.free_since = None;
    }
    pub fn get_move_history(&self) -> Option<&MoveHistory> {
        match self.main {
            MainState::Stand(StandState::Move(ref history))
            | MainState::Crouch(CrouchState::Move(ref history))
            | MainState::Air(AirState::Move(ref history)) => Some(history),
            _ => None,
        }
    }
    pub fn get_move_history_mut(&mut self) -> Option<&mut MoveHistory> {
        match self.main {
            MainState::Stand(StandState::Move(ref mut history))
            | MainState::Crouch(CrouchState::Move(ref mut history))
            | MainState::Air(AirState::Move(ref mut history)) => Some(history),
            _ => None,
        }
    }

    pub fn register_hit(&mut self) {
        if let Some(ref mut history) = self.get_move_history_mut() {
            history.has_hit = true;
        }
    }

    // Stun
    pub fn block(&mut self, recovery_frame: usize) {
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Stun(Stun::Block(recovery_frame))),
            MainState::Crouch(_) => {
                MainState::Crouch(CrouchState::Stun(Stun::Block(recovery_frame)))
            }
            MainState::Air(_) => MainState::Air(AirState::Freefall),
            MainState::Ground(_) => panic!("Blocked on the ground"),
        };
        self.free_since = None;
    }
    pub fn stun(&mut self, recovery_frame: usize) {
        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Stun(Stun::Hit(recovery_frame))),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Stun(Stun::Hit(recovery_frame))),
            MainState::Air(_) => MainState::Air(AirState::Freefall),
            MainState::Ground(_) => panic!("Stunned on the ground"),
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
            MainState::Ground(_) => MainState::Crouch(CrouchState::Idle),
        };
        self.free_since = Some(frame);
    }
    pub fn unstun_frame(&self) -> Option<usize> {
        match self.main {
            MainState::Stand(StandState::Stun(ref stun))
            | MainState::Crouch(CrouchState::Stun(ref stun)) => Some(stun.get_frame()),
            _ => None,
        }
    }
    pub fn stunned(&self) -> bool {
        matches!(
            self.main,
            MainState::Stand(StandState::Stun(_))
                | MainState::Crouch(CrouchState::Stun(_))
                | MainState::Air(AirState::Freefall)
                | MainState::Ground(_)
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
            _ => {
                panic!("Jumping while {:?}", self.main)
            }
        };
    }
    pub fn launch(&mut self) {
        self.main = MainState::Air(AirState::Freefall);
        self.free_since = None;
    }
    pub fn land(&mut self, frame: usize) {
        self.main = if matches!(self.main, MainState::Air(AirState::Freefall)) {
            MainState::Ground(frame)
        } else {
            MainState::Stand(StandState::Idle)
        };
    }
    pub fn is_grounded(&self) -> bool {
        !matches!(self.main, MainState::Air(_))
    }
    pub fn otg_since(&self) -> Option<usize> {
        if let MainState::Ground(landing_frame) = self.main {
            Some(landing_frame)
        } else {
            None
        }
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
        self.get_move_history().is_none() && self.is_grounded()
    }

    pub fn add_condition(&mut self, condition: StatusCondition) {
        self.conditions.push(condition);
    }

    pub fn has_condition(&mut self, condition: Status) -> bool {
        self.conditions.iter().any(|cond| cond.name == condition)
    }

    pub fn combined_status_effects(&self) -> StatusEffect {
        // TODO: Cache for later
        self.conditions
            .iter()
            .fold(StatusEffect::default(), |acc, cond| {
                if let Some(effect) = &cond.effect {
                    acc.combine(effect)
                } else {
                    acc
                }
            })
    }
}
