use bevy::prelude::*;

use characters::{
    Action, ActionEvent, ActionTracker, Character, Inventory, Situation, WAGResources,
};
use input_parsing::InputParser;
use wag_core::{ActionId, AnimationType, Area, Facing, Stats, StatusCondition, StatusFlag};

use crate::sub_state::{AirState, CrouchState, StandState, Stun};

#[derive(Reflect, Debug, Component, Clone)]
enum MainState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
    Ground(usize),
}

#[derive(Reflect, Debug, Component, Clone)]
#[reflect(Component)]
pub struct PlayerState {
    main: MainState,
    pub free_since: Option<usize>,
    conditions: Vec<StatusCondition>,
    unprocessed_events: Vec<ActionEvent>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            main: MainState::Stand(StandState::default()),
            free_since: Some(0),
            conditions: vec![],
            unprocessed_events: vec![],
        }
    }
}
impl PlayerState {
    pub fn reset(&mut self) {
        *self = PlayerState::default();
    }

    pub fn drain_matching_actions<T>(
        &mut self,
        predicate: impl Fn(&mut ActionEvent) -> Option<T>,
    ) -> Vec<T> {
        self.unprocessed_events
            .extract_if(|action| (predicate)(action).is_some())
            .map(|mut action| (predicate)(&mut action).unwrap())
            .collect()
    }

    pub fn last_breakpoint_frame(&self) -> Option<usize> {
        match self.main {
            MainState::Stand(StandState::Move(ref tracker))
            | MainState::Crouch(CrouchState::Move(ref tracker))
            | MainState::Air(AirState::Move(ref tracker)) => tracker.last_breakpoint_frame(),
            _ => None,
        }
    }

    pub fn add_actions(&mut self, mut actions: Vec<ActionEvent>) {
        self.unprocessed_events.append(&mut actions);
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
    #[allow(clippy::too_many_arguments)]
    pub fn start_move(
        &mut self,
        action_id: ActionId,
        action: Action,
        start_frame: usize,
        offset: usize,
        inventory: Inventory,
        resources: WAGResources,
        parser: InputParser,
        stats: Stats,
    ) {
        let events = if let Some(mutator) = action.script[0].mutator {
            let situation =
                self.build_situation(inventory, resources, parser, stats, start_frame + offset);
            mutator(action.script[0].clone(), &situation).events
        } else {
            action.script[0].clone().events
        };

        let initial_events = events.into_iter().map(|x| x.add_offset(offset));

        self.unprocessed_events.extend(initial_events); // This can't be the best way to merge Vecs
        let tracker = ActionTracker::new(action_id, action, start_frame);

        self.main = match self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Move(tracker)),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Move(tracker)),
            MainState::Air(_) => MainState::Air(AirState::Move(tracker)),
            MainState::Ground(_) => panic!("Starting a move on the ground"),
        };
        self.free_since = None;
    }
    pub fn proceed_move(
        &mut self,
        inventory: Inventory,
        resources: WAGResources,
        parser: InputParser,
        stats: Stats,
        frame: usize,
    ) {
        let situation = self.build_situation(inventory, resources, parser, stats, frame);
        let tracker = self.get_action_tracker_mut().unwrap();

        if tracker.blocker.fulfilled(&situation) {
            if let Some(next_block) = tracker.pop_next(frame) {
                let mutated = next_block.apply_mutator(&situation);
                tracker.blocker = mutated.exit_requirement;
                tracker.cancel_policy = mutated.cancel_policy;
                self.unprocessed_events.extend(mutated.events);
            } else {
                self.recover(frame);
            }
        }
    }
    pub fn build_situation(
        &self,
        inventory: Inventory,
        resources: WAGResources,
        input_parser: InputParser,
        stats: Stats,
        frame: usize,
    ) -> Situation {
        Situation {
            inventory,
            frame,
            stats,
            resources: resources.0,
            grounded: self.is_grounded(),
            tracker: self.get_action_tracker().cloned(),
            held_buttons: input_parser.get_pressed(),
            status_flags: self.conditions.iter().map(|c| c.flag).collect(),
        }
    }
    pub fn get_action_tracker(&self) -> Option<&ActionTracker> {
        match self.main {
            MainState::Stand(StandState::Move(ref history))
            | MainState::Crouch(CrouchState::Move(ref history))
            | MainState::Air(AirState::Move(ref history)) => Some(history),
            _ => None,
        }
    }
    pub fn get_action_tracker_mut(&mut self) -> Option<&mut ActionTracker> {
        match self.main {
            MainState::Stand(StandState::Move(ref mut history))
            | MainState::Crouch(CrouchState::Move(ref mut history))
            | MainState::Air(AirState::Move(ref mut history)) => Some(history),
            _ => None,
        }
    }
    pub fn action_in_progress(&self) -> bool {
        self.get_action_tracker().is_some()
    }

    pub fn register_hit(&mut self) {
        if let Some(ref mut tracker) = self.get_action_tracker_mut() {
            tracker.has_hit = true;
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
            // Launch
            MainState::Air(AirState::Freefall) => {}
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

        self.conditions
            .retain(|cond| cond.flag != StatusFlag::DoubleJumped);
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

    pub fn get_pushbox(&self, character: &Character) -> Area {
        match self.main {
            MainState::Stand(_) => character.standing_pushbox,
            MainState::Crouch(_) => character.crouching_pushbox,
            MainState::Air(_) => character.air_pushbox,
            MainState::Ground(_) => character.crouching_pushbox, // TODO: This could have it's own box
        }
    }
    pub fn add_condition(&mut self, condition: StatusCondition) {
        self.conditions.push(condition);
    }
    pub fn get_conditions(&self) -> &[StatusCondition] {
        &self.conditions
    }
    pub fn has_flag(&self, condition: StatusFlag) -> bool {
        self.conditions.iter().any(|cond| cond.flag == condition)
    }
    pub fn combined_status_effects(&self) -> Stats {
        // TODO: Cache for later
        self.conditions.iter().fold(Stats::identity(), |acc, cond| {
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
    use characters::Action;

    use super::*;

    #[test]
    fn generic_animation_mid_move() {
        // TODO: Creating testing states should be easier
        let mut move_state = PlayerState {
            main: MainState::Stand(StandState::Move(ActionTracker::new(
                ActionId::TestMove,
                Action::default(),
                0,
            ))),
            ..default()
        };

        assert_eq!(move_state.get_generic_animation(Facing::Left), None);

        move_state.main = MainState::Stand(StandState::Idle);

        assert_eq!(
            move_state.get_generic_animation(Facing::Left),
            Some(AnimationType::StandIdle)
        );
    }
}
