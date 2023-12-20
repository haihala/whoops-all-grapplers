use bevy::prelude::*;
use wag_core::HIT_FLASH_COLOR;

use crate::{ActionEvent, FlashRequest, Movement, ResourceType, ToHit};

#[derive(Debug, Clone, PartialEq, Component, Reflect)]
pub struct Attack {
    pub to_hit: ToHit,
    pub self_on_hit: Vec<ActionEvent>,
    pub self_on_block: Vec<ActionEvent>,
    pub target_on_hit: Vec<ActionEvent>,
    pub target_on_block: Vec<ActionEvent>,
}

impl Default for Attack {
    fn default() -> Self {
        Attack::new(ToHit::default(), CommonAttackProps::default())
    }
}
impl Attack {
    pub fn new(to_hit: ToHit, cab: CommonAttackProps) -> Attack {
        Attack {
            to_hit,
            self_on_hit: cab.self_on_hit(),
            self_on_block: cab.self_on_block(),
            target_on_hit: cab.target_on_hit(),
            target_on_block: cab.target_on_block(),
        }
    }

    pub fn with_to_self_on_hit(self, additional_actions: Vec<ActionEvent>) -> Attack {
        Attack {
            self_on_hit: [self.self_on_hit.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_self_on_block(self, additional_actions: Vec<ActionEvent>) -> Attack {
        Attack {
            self_on_block: [self.self_on_block.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_target_on_hit(self, additional_actions: Vec<ActionEvent>) -> Attack {
        Attack {
            target_on_hit: [self.target_on_hit.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_target_on_block(self, additional_actions: Vec<ActionEvent>) -> Attack {
        Attack {
            target_on_block: [self.target_on_block.clone(), additional_actions].concat(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum StunType {
    Launcher(f32),
    Roller(Vec2),
    Stun(usize),
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct CommonAttackProps {
    pub damage: i32,
    pub knock_back: f32,
    pub push_back: f32,
    pub on_hit: StunType,
    pub on_block: StunType,
}

impl Default for CommonAttackProps {
    fn default() -> Self {
        Self {
            damage: 5,
            knock_back: 5.0,
            push_back: 3.0,
            on_hit: StunType::Stun(20),
            on_block: StunType::Stun(10),
        }
    }
}

impl CommonAttackProps {
    pub fn self_on_hit(self) -> Vec<ActionEvent> {
        vec![
            Movement::impulse(-Vec2::X * self.push_back).into(),
            ActionEvent::CameraTilt(Vec2::X * self.push_back * 0.008),
            ActionEvent::CameraShake,
            ActionEvent::Hitstop,
        ]
    }
    pub fn self_on_block(self) -> Vec<ActionEvent> {
        vec![
            Movement::impulse(-Vec2::X * 1.5 * self.push_back).into(),
            ActionEvent::CameraTilt(-Vec2::X * self.push_back * 0.005),
            ActionEvent::Hitstop,
        ]
    }

    pub fn target_on_hit(self) -> Vec<ActionEvent> {
        vec![
            ActionEvent::ModifyResource(ResourceType::Health, -self.damage),
            self.get_stun(false),
            Movement::impulse(-Vec2::X * self.knock_back).into(),
            ActionEvent::Flash(FlashRequest {
                color: HIT_FLASH_COLOR,
                depth: 1.0,
                duration: 0.2,
                ..default()
            }),
        ]
    }

    pub fn target_on_block(self) -> Vec<ActionEvent> {
        vec![
            ActionEvent::ModifyResource(ResourceType::Health, -1), // Chip
            self.get_stun(true),
            Movement::impulse(-Vec2::X * 0.5 * self.knock_back).into(),
        ]
    }

    fn get_stun(&self, blocked: bool) -> ActionEvent {
        if blocked {
            match self.on_block {
                StunType::Launcher(_) | StunType::Roller(_) => {
                    // If launching on block, the design needs to be re-evaluated
                    todo!()
                }
                StunType::Stun(frames) => ActionEvent::BlockStun(frames),
            }
        } else {
            match self.on_hit {
                StunType::Launcher(height) => ActionEvent::Launch {
                    impulse: height * Vec2::Y,
                },
                StunType::Roller(impulse) => ActionEvent::Launch { impulse },
                StunType::Stun(frames) => ActionEvent::HitStun(frames),
            }
        }
    }
}
