use bevy::prelude::*;

use characters::{
    ActionEvent, ActionTracker, Character, CharacterStateBoxes, Gauges, Inventory, Situation,
};
use foundation::{
    ActionId, AnimationType, CancelType, CharacterFacing, Combo, Facing, SimpleState, Stats,
    StatusCondition, StatusFlag,
};
use input_parsing::InputParser;

use crate::sub_state::{AirState, CrouchState, StandState, Stun};

#[derive(Reflect, Debug, Component, Clone, Hash)]
enum MainState {
    Air(AirState),
    Stand(StandState),
    Crouch(CrouchState),
    Ground(usize),
}

#[derive(Component, Debug, Clone, Hash)]
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
    pub fn start_move(&mut self, action_id: ActionId, start_frame: usize) {
        let tracker = ActionTracker::new(start_frame, self.action_in_progress(), action_id);

        self.main = match &self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Move(tracker)),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Move(tracker)),
            MainState::Air(_) => MainState::Air(AirState::Move(tracker)),
            // TODO: crashes during playtest
            other => panic!("Starting a move while {:?}", other),
        };
        self.free_since = None;
        self.clear_cancel_windows();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn proceed_move(
        &mut self,
        inventory: Inventory,
        character: &Character,
        resources: Gauges,
        parser: InputParser,
        stats: Stats,
        frame: usize,
        player_position: Vec3,
        player_facing: CharacterFacing,
        combo: Combo,
    ) -> Vec<ActionEvent> {
        let situation = self.build_situation(
            inventory,
            resources,
            parser,
            stats,
            frame,
            player_position,
            player_facing,
            combo,
        );

        let action_id = self.get_action_tracker().unwrap().action_id;
        (character.get_move(action_id).unwrap().script)(&situation)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build_situation(
        &self,
        inventory: Inventory,
        resources: Gauges,
        input_parser: InputParser,
        stats: Stats,
        frame: usize,
        player_position: Vec3,
        player_facing: CharacterFacing,
        combo: Combo,
    ) -> Situation {
        Situation {
            inventory,
            frame,
            stats,
            resources: resources.0,
            grounded: self.is_grounded(),
            tracker: self.get_action_tracker().cloned(),
            held_buttons: input_parser.get_pressed(),
            stick_position: input_parser.get_stick_pos(),
            status_flags: self.conditions.iter().map(|c| c.flag.clone()).collect(),
            position: player_position,
            facing: player_facing,
            combo,
            stunned: self.stunned(),
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

    fn get_action_tracker_mut(&mut self) -> Option<&mut ActionTracker> {
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
        self.main = match &self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Stun(Stun::Block(recovery_frame))),
            MainState::Crouch(_) => {
                MainState::Crouch(CrouchState::Stun(Stun::Block(recovery_frame)))
            }
            other => panic!("Blocked while {:?}", other),
        };
        self.free_since = None;
    }
    pub fn hit_stun(&mut self, recovery_frame: usize) {
        self.main = match &self.main {
            MainState::Stand(_) => MainState::Stand(StandState::Stun(Stun::Hit(recovery_frame))),
            MainState::Crouch(_) => MainState::Crouch(CrouchState::Stun(Stun::Hit(recovery_frame))),
            MainState::Air(_) => MainState::Air(AirState::Freefall),
            other => panic!("Stunned while {:?}", other),
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
        self.clear_cancel_windows();
        self.clear_comic_cancels();
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
    pub fn can_block(&self) -> bool {
        matches!(
            self.main,
            MainState::Stand(
                StandState::Idle | StandState::Walk(_) | StandState::Stun(Stun::Block(_))
            ) | MainState::Crouch(CrouchState::Idle | CrouchState::Stun(Stun::Block(_)))
        )
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
            .retain(|cond| cond.flag != StatusFlag::AirActionCooldown);
    }
    pub fn is_grounded(&self) -> bool {
        !matches!(self.main, MainState::Air(_))
    }
    pub fn can_update_visual_facing(&self) -> bool {
        self.is_grounded() && !self.has_flag(StatusFlag::MovementLock)
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

    pub fn force_state(&mut self, simple_state: SimpleState) {
        match simple_state {
            SimpleState::Air => match &self.main {
                MainState::Air(_) => {}
                // Jumps are moves
                MainState::Stand(StandState::Move(tracker))
                | MainState::Crouch(CrouchState::Move(tracker)) => {
                    self.main = MainState::Air(AirState::Move(*tracker))
                }
                // All others are launchers I think?
                _ => self.main = MainState::Air(AirState::Freefall),
            },
            SimpleState::Stand => match &self.main {
                MainState::Stand(_) => {}
                MainState::Air(AirState::Move(tracker))
                | MainState::Crouch(CrouchState::Move(tracker)) => {
                    self.main = MainState::Stand(StandState::Move(*tracker))
                }
                MainState::Crouch(CrouchState::Stun(stun)) => {
                    self.main = MainState::Stand(StandState::Stun(stun.clone()))
                }
                // This allows restand mixups
                _ => self.main = MainState::Stand(StandState::Idle),
            },
            SimpleState::Crouch => match &self.main {
                MainState::Crouch(_) => {}
                MainState::Air(AirState::Move(tracker))
                | MainState::Stand(StandState::Move(tracker)) => {
                    self.main = MainState::Crouch(CrouchState::Move(*tracker))
                }
                MainState::Stand(StandState::Stun(stun)) => {
                    self.main = MainState::Crouch(CrouchState::Stun(stun.clone()))
                }
                // This allows restand mixups
                _ => self.main = MainState::Crouch(CrouchState::Idle),
            },
        }
    }

    pub fn is_crouching(&self) -> bool {
        matches!(self.main, MainState::Crouch(_))
    }

    pub fn get_boxes(&self, character: &Character) -> CharacterStateBoxes {
        match self.main {
            MainState::Stand(_) => character.boxes.standing,
            MainState::Air(_) => character.boxes.airborne,
            // TODO: These could have it's own box
            MainState::Crouch(_) | MainState::Ground(_) => character.boxes.crouching,
        }
    }
    pub fn add_condition(&mut self, condition: StatusCondition) {
        self.conditions.push(condition);
    }
    pub fn clear_conditions(&mut self, flag: StatusFlag) {
        self.conditions.retain(|cond| cond.flag != flag);
    }
    pub fn get_conditions(&self) -> &[StatusCondition] {
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

    pub fn cancels(&self) -> Vec<CancelType> {
        self.conditions
            .clone()
            .iter_mut()
            .filter_map(|cond| {
                if let StatusFlag::Cancel(ct) = &cond.flag {
                    Some(ct.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    // This is called when a move starts OR ends
    pub fn clear_cancel_windows(&mut self) {
        self.conditions
            .retain(|cond| !matches!(cond.flag, StatusFlag::Cancel(_)));
    }

    // This is called when a move recovers naturally
    pub fn clear_comic_cancels(&mut self) {
        self.conditions
            .retain(|cond| !matches!(cond.flag, StatusFlag::ComicCancelCooldown));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generic_animation_mid_move() {
        // TODO: Creating testing states should be easier
        let mut move_state = PlayerState {
            main: MainState::Stand(StandState::Move(ActionTracker::new(
                0,
                false,
                ActionId::TestMove,
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
