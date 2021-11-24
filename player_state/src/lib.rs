use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use std::fmt::Debug;

use moves::FrameData;
use types::{AbsoluteDirection, RelativeDirection};

mod animation;
use animation::Animation;

mod primary_state;
use primary_state::*;

mod events;
use events::*;

pub use events::{AnimationEvent, StateEvent};

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum FreedomLevel {
    // TODO: This probably needs to adapt when cancelling complexity comes
    Stunned,
    Busy,
    LightBusy,
    Free,
}

#[derive(Inspectable, PartialEq, Clone, Debug)]
pub struct PlayerState {
    facing: AbsoluteDirection,
    primary: PrimaryState,
    frame: usize,
    events: Vec<(usize, StateEvent)>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            facing: Default::default(),
            primary: PrimaryState::Ground(GroundActivity::Standing),
            frame: 0,
            events: Default::default(),
        }
    }
}
impl PlayerState {
    pub fn tick(&mut self, current_frame: usize) {
        // TODO: change dash and
        self.frame = current_frame;

        match self.primary {
            PrimaryState::Ground(activity) => {
                match activity {
                    GroundActivity::Stun(unstun_frame) => {
                        if unstun_frame <= self.frame {
                            self.primary = PrimaryState::Ground(GroundActivity::Standing);
                        }
                    }
                    GroundActivity::Animation(mut animation) => {
                        if let Some(event) = animation.tick(self.frame) {
                            if matches!(
                                event,
                                StateEvent::AnimationUpdate(AnimationEvent::Recovered)
                            ) {
                                // TODO: Returning to neutral can't always go standing
                                self.primary = PrimaryState::Ground(GroundActivity::Standing);
                            } else {
                                self.primary =
                                    PrimaryState::Ground(GroundActivity::Animation(animation));
                            }
                            self.events.push((self.frame, event));
                        }
                    }
                    GroundActivity::Movement(movement) => match movement {
                        Movement::Dash(mut dash) => {
                            if dash.tick_check_expiration(self.frame) {
                                // Dash has ended
                                self.primary = PrimaryState::Ground(GroundActivity::Standing);
                            } else {
                                self.primary = PrimaryState::Ground(GroundActivity::Movement(
                                    Movement::Dash(dash),
                                ));
                            }
                        }
                        Movement::Walk((last_input_frame, _)) => {
                            // Implicitly stop walking if no events have come in for a few frames
                            if last_input_frame + 5 < self.frame {
                                self.primary = PrimaryState::Ground(GroundActivity::Standing);
                            }
                        }
                        Movement::Null => panic!("Null movement state"),
                    },
                    _ => {}
                }
            }
            PrimaryState::Air(AirActivity::Animation(mut animation)) => {
                if let Some(event) = animation.tick(self.frame) {
                    if matches!(
                        event,
                        StateEvent::AnimationUpdate(AnimationEvent::Recovered)
                    ) {
                        self.primary = PrimaryState::Air(AirActivity::Idle);
                    } else {
                        self.primary = PrimaryState::Air(AirActivity::Animation(animation));
                    }
                    self.events.push((self.frame, event));
                }
            }
            _ => {}
        }

        for late_event in self
            .events
            .iter()
            .filter(|(frame, _)| frame + 300 < self.frame)
        {
            dbg!(late_event);
            panic!("Event wasn't processed");
        }
    }

    pub fn freedom_level(&self) -> FreedomLevel {
        match self.primary {
            PrimaryState::Ground(activity) => activity.freedom_level(),
            PrimaryState::Air(_) => FreedomLevel::Busy, // TODO: This is temporary and prohibits attacking while in the air
        }
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

    // Animation
    pub fn start_animation(&mut self, frame_data: FrameData) {
        let animation = Animation::new(self.frame, frame_data);
        match self.primary {
            PrimaryState::Ground(_) => {
                self.primary = PrimaryState::Ground(GroundActivity::Animation(animation));
            }
            PrimaryState::Air(_) => {
                self.primary = PrimaryState::Air(AirActivity::Animation(animation));
            }
        }
    }
    pub fn animation_in_progress(&self) -> bool {
        matches!(
            self.primary,
            PrimaryState::Ground(GroundActivity::Animation(_))
        ) || matches!(self.primary, PrimaryState::Air(AirActivity::Animation(_)))
    }

    // Stun
    pub fn hit(&mut self, recovery_frame: usize, launching_hit: bool) {
        match self.primary {
            PrimaryState::Ground(_) => {
                if launching_hit {
                    self.primary = PrimaryState::Air(AirActivity::Freefall);
                } else {
                    self.primary = PrimaryState::Ground(GroundActivity::Stun(recovery_frame));
                }
            }
            PrimaryState::Air(_) => {
                self.primary = PrimaryState::Air(AirActivity::Freefall);
            }
        }
    }

    // Dash
    pub fn start_dash(&mut self, direction: RelativeDirection) {
        self.primary = PrimaryState::Ground(GroundActivity::Movement(Movement::Dash(
            DashState::new(direction, self.frame),
        )));
    }
    pub fn get_dash(&mut self) -> Option<DashState> {
        match self.primary {
            PrimaryState::Ground(GroundActivity::Movement(Movement::Dash(dash_state))) => {
                Some(dash_state)
            }
            _ => None,
        }
    }

    // Jumping
    pub fn land(&mut self) {
        self.primary = PrimaryState::Ground(GroundActivity::Standing);
    }
    pub fn register_jump(&mut self, direction: Option<RelativeDirection>) {
        dbg!("Jump");
        self.primary = PrimaryState::Air(AirActivity::Idle);
        self.events.push((
            self.frame,
            StateEvent::Jump(match direction {
                Some(relative_direction) => {
                    JumpDirection::Diagonal(relative_direction.as_absolute(self.facing))
                }
                None => JumpDirection::Neutral,
            }),
        ));
    }
    pub fn jump_direction_to_impulse(&mut self, jump_direction: JumpDirection) -> Vec3 {
        match jump_direction {
            JumpDirection::Neutral => constants::NEUTRAL_JUMP_VECTOR.into(),
            JumpDirection::Diagonal(direction) => {
                direction.handle_mirroring(constants::DIAGONAL_JUMP_VECTOR.into())
            }
            JumpDirection::Null => panic!("Null jump direction"),
        }
    }
    pub fn is_grounded(&self) -> bool {
        matches!(self.primary, PrimaryState::Ground(_))
    }

    // Walking
    pub fn walk(&mut self, direction: RelativeDirection) {
        self.primary = PrimaryState::Ground(GroundActivity::Movement(Movement::Walk((
            self.frame, direction,
        ))));
    }
    pub fn get_walk_direction(&self) -> Option<AbsoluteDirection> {
        if let PrimaryState::Ground(GroundActivity::Movement(Movement::Walk((_, direction)))) =
            self.primary
        {
            Some(direction.as_absolute(self.facing))
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

    pub fn get_collider_size(&self) -> Vec2 {
        if self.primary == PrimaryState::Ground(GroundActivity::Crouching) {
            Vec2::new(
                constants::PLAYER_SPRITE_WIDTH,
                constants::PLAYER_SPRITE_CROUCHING_HEIGHT,
            )
        } else {
            Vec2::new(
                constants::PLAYER_SPRITE_WIDTH,
                constants::PLAYER_SPRITE_STANDING_HEIGHT,
            )
        }
    }
}
