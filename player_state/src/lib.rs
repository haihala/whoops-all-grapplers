use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use moves::{CancelLevel, Move, MoveMobility, Phase, PhaseKind};

use std::fmt::Debug;

use types::{AttackHeight, HeightWindow, LRDirection, MoveId, StickPosition};

mod primary_state;
use primary_state::*;

mod events;

pub use events::StateEvent;

pub const PLAYER_SPRITE_WIDTH: f32 = 0.80;
pub const PLAYER_SPRITE_STANDING_HEIGHT: f32 = 1.80;
const PLAYER_SPRITE_CROUCHING_HEIGHT_MULTIPLIER: f32 = 0.6;
const PLAYER_LOW_BLOCK_THRESHOLD_RATIO: f32 = 0.25;
const PLAYER_HIGH_BLOCK_THRESHOLD_RATIO: f32 = 0.75;

pub const PLAYER_SPRITE_CROUCHING_HEIGHT: f32 =
    PLAYER_SPRITE_STANDING_HEIGHT * PLAYER_SPRITE_CROUCHING_HEIGHT_MULTIPLIER;
pub const PLAYER_CROUCHING_OFFSET: f32 = PLAYER_SPRITE_STANDING_HEIGHT / 2.0;
pub const PLAYER_STANDING_OFFSET: f32 = PLAYER_SPRITE_CROUCHING_HEIGHT / 2.0;
pub const PLAYER_CROUCHING_SHIFT: f32 = PLAYER_STANDING_OFFSET - PLAYER_CROUCHING_OFFSET;
pub const PLAYER_STANDING_SHIFT: f32 = -PLAYER_CROUCHING_SHIFT;

#[derive(Debug, Default, Inspectable, Clone)]
struct MoveTracker {
    move_data: Move,
    start_frame: usize,
    previous_phase: Phase,
    id: MoveId,
}
impl MoveTracker {
    fn get_phase(&self, frame: usize) -> Option<&Phase> {
        self.move_data.get_phase(self.start_frame, frame)
    }
}

#[derive(Inspectable, Debug)]
pub struct PlayerState {
    primary: PrimaryState,
    move_tracker: Option<MoveTracker>,
    frame: usize,
    events: Vec<(usize, StateEvent)>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            primary: PrimaryState::Ground(GroundActivity::Standing),
            move_tracker: Default::default(),
            frame: Default::default(),
            events: Default::default(),
        }
    }
}
impl PlayerState {
    pub fn tick(&mut self, current_frame: usize) {
        self.frame = current_frame;

        match self.primary {
            PrimaryState::Ground(activity) => {
                match activity {
                    GroundActivity::Stun(unstun_frame) => {
                        if unstun_frame <= self.frame {
                            self.primary = PrimaryState::Ground(GroundActivity::Standing);
                            self.add_event(StateEvent::Recovery);
                        }
                    }
                    GroundActivity::Walk(last_input_frame, _) => {
                        // Implicitly stop walking if no events have come in for a few frames
                        // The delay of 5 frames is a bit much, but hopefully gives the character some weight
                        if last_input_frame + 5 < self.frame {
                            self.primary = PrimaryState::Ground(GroundActivity::Standing);
                        }
                    }
                    GroundActivity::Move(id) => {
                        self.move_tick(id, PrimaryState::Ground(GroundActivity::Standing))
                    }
                    _ => {}
                }
            }
            PrimaryState::Air(AirActivity::Move(id)) => {
                self.move_tick(id, PrimaryState::Air(AirActivity::Idle))
            }
            _ => {}
        }

        for late_event in self
            .events
            .iter()
            .filter(|(frame, _)| frame + 300 < self.frame)
        {
            panic!("Late event {:?} wasn't processed", late_event);
        }
    }
    fn move_tick(&mut self, move_id: MoveId, return_state: PrimaryState) {
        let mut tracker = self.move_tracker.clone().unwrap();
        let phase = tracker.get_phase(self.frame);

        if let Some(new_phase) = phase {
            if *new_phase != tracker.previous_phase {
                self.add_event(StateEvent::PhaseChange);

                if let PhaseKind::Attack(descriptor) = new_phase.kind {
                    self.add_event(StateEvent::Attack(move_id, descriptor));
                } else if let PhaseKind::Grab(descriptor) = new_phase.kind {
                    self.add_event(StateEvent::Grab(descriptor));
                }

                tracker.previous_phase = new_phase.to_owned();
                self.move_tracker = Some(tracker);
            }
        } else {
            // Move is over
            self.move_tracker = None;
            self.primary = return_state;
        }
    }

    fn add_event(&mut self, event: StateEvent) {
        self.events.push((self.frame, event));
    }

    pub fn get_events(&self) -> Vec<StateEvent> {
        self.events
            .clone()
            .into_iter()
            .map(|(_, event)| event)
            .collect()
    }

    pub fn consume_event(&mut self, event: StateEvent) {
        self.events.retain(|(_, e)| *e != event);
    }

    // Moves
    pub fn start_move(&mut self, id: MoveId, move_data: Move) {
        self.primary = match self.primary {
            PrimaryState::Ground(_) => PrimaryState::Ground(GroundActivity::Move(id.to_owned())),
            PrimaryState::Air(_) => PrimaryState::Air(AirActivity::Move(id.to_owned())),
        };
        self.move_tracker = Some(MoveTracker {
            previous_phase: move_data
                .get_phase(self.frame, self.frame)
                .unwrap()
                .to_owned(),
            move_data,
            id,
            start_frame: self.frame,
        });
    }
    pub fn cancel_requirement(&self) -> CancelLevel {
        if let Some(tracker) = &self.move_tracker {
            if let Some(phase) = tracker.get_phase(self.frame) {
                return phase.cancel_requirement;
            }
        }
        match self.primary {
            PrimaryState::Ground(GroundActivity::Walk(_, _)) => CancelLevel::Walk,
            _ => CancelLevel::Anything,
        }
    }
    pub fn get_move_mobility(&self) -> Option<(MoveId, MoveMobility)> {
        if let Some(tracker) = self.move_tracker.as_ref() {
            if let Some(mobility) = tracker.get_phase(self.frame).map(|phase| phase.mobility) {
                if !matches!(mobility, MoveMobility::None) {
                    return Some((tracker.id, mobility));
                }
            }
        }
        None
    }

    // Stun
    pub fn hit(&mut self, recovery_frame: usize) {
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

    // Jumping
    pub fn land(&mut self) {
        if matches!(self.primary, PrimaryState::Air(AirActivity::Move(_))) {
            self.add_event(StateEvent::PhaseChange);
        }
        if matches!(self.primary, PrimaryState::Air(AirActivity::Freefall)) {
            // TODO: Better handling of what happens on landing
            self.add_event(StateEvent::Recovery);
        }
        self.primary = PrimaryState::Ground(GroundActivity::Standing);
    }
    pub fn jump(&mut self) {
        if let PrimaryState::Ground(GroundActivity::Move(id)) = self.primary {
            self.primary = PrimaryState::Air(AirActivity::Move(id));
        } else if self.is_grounded() {
            // was grounded doing something else, now in air without a move
            self.primary = PrimaryState::Air(AirActivity::Freefall);
        }
    }
    pub fn launch(&mut self) {
        self.primary = PrimaryState::Air(AirActivity::Freefall);
    }

    pub fn is_grounded(&self) -> bool {
        matches!(self.primary, PrimaryState::Ground(_))
    }

    // Walking
    pub fn walk(&mut self, direction: LRDirection) {
        if self.cancel_requirement() > CancelLevel::Walk {
            return;
        }
        self.primary = PrimaryState::Ground(GroundActivity::Walk(self.frame, direction));
    }
    pub fn get_walk_direction(&self) -> Option<LRDirection> {
        if let PrimaryState::Ground(GroundActivity::Walk(_, direction)) = self.primary {
            Some(direction)
        } else {
            None
        }
    }

    pub fn crouch(&mut self) {
        if self.cancel_requirement() > CancelLevel::Anything {
            return;
        }
        self.primary = PrimaryState::Ground(GroundActivity::Crouching);
    }
    pub fn stand(&mut self) {
        if self.cancel_requirement() > CancelLevel::Anything {
            return;
        }
        self.primary = PrimaryState::Ground(GroundActivity::Standing);
    }
    pub fn is_crouching(&self) -> bool {
        matches!(
            self.primary,
            PrimaryState::Ground(GroundActivity::Crouching)
        )
    }
    pub fn get_height(&self) -> f32 {
        if self.is_crouching() {
            PLAYER_SPRITE_CROUCHING_HEIGHT
        } else {
            PLAYER_SPRITE_STANDING_HEIGHT
        }
    }
    pub fn get_width(&self) -> f32 {
        PLAYER_SPRITE_WIDTH
    }
    pub fn get_collider_size(&self) -> Vec2 {
        Vec2::new(self.get_width(), self.get_height())
    }

    pub fn blocked(
        &self,
        fixed_height: Option<&AttackHeight>,
        height_window: HeightWindow,
        stick: StickPosition,
    ) -> bool {
        if !self.can_block_now() {
            return false;
        }

        let blocking_high = stick == StickPosition::E;
        let blocking_low = stick == StickPosition::SE;

        let height = fixed_height.unwrap_or(if self.low_block_threshold() > height_window.top {
            &AttackHeight::Low
        } else if self.high_block_threshold() > height_window.bottom {
            &AttackHeight::High
        } else {
            &AttackHeight::Mid
        });

        match height {
            AttackHeight::Low => blocking_low,
            AttackHeight::Mid => blocking_low || blocking_high,
            AttackHeight::High => blocking_high,
        }
    }
    fn can_block_now(&self) -> bool {
        self.cancel_requirement() < CancelLevel::LightNormal && self.is_grounded()
    }
    fn low_block_threshold(&self) -> f32 {
        self.get_height() * PLAYER_LOW_BLOCK_THRESHOLD_RATIO
    }
    fn high_block_threshold(&self) -> f32 {
        self.get_height() * PLAYER_HIGH_BLOCK_THRESHOLD_RATIO
    }
}
