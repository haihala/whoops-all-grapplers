use bevy::prelude::*;

use crate::{ActionEvent, Movement, ResourceType, ToHit};

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
    Launcher,
    Stun(usize),
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct CommonAttackProps {
    pub damage: i32,
    pub knock_back: Vec2,
    pub push_back: Vec2,
    pub on_hit: StunType,
    pub on_block: StunType,
}

impl Default for CommonAttackProps {
    fn default() -> Self {
        Self {
            damage: 5,
            knock_back: -Vec2::X,
            push_back: -Vec2::X,
            on_hit: StunType::Stun(15),
            on_block: StunType::Stun(10),
        }
    }
}

impl CommonAttackProps {
    pub fn self_on_hit(self) -> Vec<ActionEvent> {
        vec![
            Movement::impulse(self.push_back).into(),
            ActionEvent::CameraTilt(self.push_back * -0.03),
            ActionEvent::CameraShake,
            ActionEvent::Hitstop,
        ]
    }
    pub fn self_on_block(self) -> Vec<ActionEvent> {
        vec![
            Movement::impulse(2.0 * self.push_back).into(),
            ActionEvent::CameraTilt(self.push_back * 0.01),
            ActionEvent::Hitstop,
        ]
    }

    pub fn target_on_hit(self) -> Vec<ActionEvent> {
        vec![
            ActionEvent::ModifyResource(ResourceType::Health, -self.damage),
            self.get_stun(false),
            Movement::impulse(self.knock_back).into(),
        ]
    }

    pub fn target_on_block(self) -> Vec<ActionEvent> {
        vec![
            ActionEvent::ModifyResource(ResourceType::Health, -1), // Chip
            self.get_stun(true),
            Movement::impulse(0.5 * self.knock_back).into(),
        ]
    }

    fn get_stun(&self, blocked: bool) -> ActionEvent {
        if blocked {
            match self.on_block {
                StunType::Launcher => {
                    warn!("Launch on block?");
                    ActionEvent::Launch
                }
                StunType::Stun(frames) => ActionEvent::BlockStun(frames),
            }
        } else {
            match self.on_hit {
                StunType::Launcher => ActionEvent::Launch,
                StunType::Stun(frames) => ActionEvent::HitStun(frames),
            }
        }
    }
}
