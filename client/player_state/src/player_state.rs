use bevy::prelude::*;

use characters::{Action, MoveHistory, Situation};
use wag_core::{AnimationType, Facing, Stats, StatusCondition, StatusFlag};

use crate::sub_state::{AirState, CrouchState, StandState, Stun};

#[derive(Reflect, FromReflect, Debug, Component, Clone)]
enum MainState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
    Ground(usize),
}

#[derive(Reflect, Debug, Component, Clone)]
pub struct PlayerState {
    main: MainState,
    pub free_since: Option<usize>,
    conditions: Vec<StatusCondition>,
    external_actions: Vec<Action>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            main: MainState::Stand(StandState::default()),
            free_since: Some(0),
            conditions: vec![],
            external_actions: vec![],
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

            let new_fcs = situation.new_fcs();
            history.add_actions_from(new_fcs.clone());
            history.past.extend(new_fcs);
        }
    }

    pub fn current_move_fully_handled(&self) -> Option<bool> {
        self.get_move_history().map(|history| history.is_done())
    }

    pub fn drain_matching_actions<T>(
        &mut self,
        predicate: impl Fn(&mut Action) -> Option<T>,
    ) -> Vec<T> {
        let mut actions: Vec<T> = self
            .external_actions
            .drain_filter(|action| (predicate)(action).is_some())
            .map(|mut action| (predicate)(&mut action).unwrap())
            .collect();

        if let Some(ref mut history) = self.get_move_history_mut() {
            let history_actions = history
                .unprocessed_events
                .drain_filter(|action| (predicate)(action).is_some())
                .map(|mut action| (predicate)(&mut action).unwrap());
            actions.extend(history_actions);
        }
        actions
    }

    pub fn add_actions(&mut self, mut actions: Vec<Action>) {
        self.external_actions.append(&mut actions);
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
    pub fn is_free(&self) -> bool {
        self.get_move_history().is_none()
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
            self.free_since = Some(frame);
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
    pub fn force_stand(&mut self) {
        match self.main {
            MainState::Stand(_) => {
                // Already standing, everything is great
            }
            MainState::Crouch(ref cs) => {
                self.main = match cs {
                    CrouchState::Stun(stun) => MainState::Stand(StandState::Stun(stun.clone())),
                    CrouchState::Move(move_history) => {
                        MainState::Stand(StandState::Move(move_history.clone()))
                    }
                    CrouchState::Idle => MainState::Stand(StandState::Idle),
                }
            }
            MainState::Air(_) => panic!("Forcing to stand in the air"),
            MainState::Ground(_) => panic!("Forcing to stand on the ground"),
        };
    }
    pub fn is_crouching(&self) -> bool {
        matches!(self.main, MainState::Crouch(_))
    }

    pub fn add_condition(&mut self, condition: StatusCondition) {
        self.conditions.push(condition);
    }
    pub fn get_conditions(&self) -> &Vec<StatusCondition> {
        &self.conditions
    }
    pub fn has_flag(&self, condition: StatusFlag) -> bool {
        self.conditions.iter().any(|cond| cond.flag == condition)
    }
    pub fn combined_status_effects(&self) -> Stats {
        // TODO: Cache for later
        self.conditions.iter().fold(Stats::default(), |acc, cond| {
            if let Some(effect) = &cond.effect {
                acc.combine(effect)
            } else {
                acc
            }
        })
    }
    pub fn expire_conditions(&mut self, frame: usize) {
        self.conditions
            .retain(|cond| cond.expiration.is_none() || cond.expiration.unwrap() > frame);
    }
    pub fn is_intangible(&self) -> bool {
        self.otg_since().is_some() || self.has_flag(StatusFlag::Intangible)
    }
}

#[cfg(test)]
mod test {
    use characters::Move;
    use wag_core::MoveId;

    use super::*;

    #[test]
    fn generic_animation_mid_move() {
        // TODO: Creating testing states should be easier
        let mut move_state = PlayerState {
            main: MainState::Stand(StandState::Move(MoveHistory {
                move_id: MoveId::TestMove,
                move_data: Move::default(),
                frame_skip: 0,
                started: 0,
                past: vec![],
                unprocessed_events: vec![],
                has_hit: false,
            })),
            free_since: Some(2),
            conditions: vec![],
            external_actions: vec![],
        };

        assert_eq!(move_state.get_generic_animation(Facing::Left), None);

        move_state.main = MainState::Stand(StandState::Idle);

        assert_eq!(
            move_state.get_generic_animation(Facing::Left),
            Some(AnimationType::StandIdle)
        );
    }
}
