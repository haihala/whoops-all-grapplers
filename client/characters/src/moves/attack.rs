use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{Action, Movement, ToHit};

#[derive(Debug, Clone, PartialEq, Component, Inspectable)]
pub struct Attack {
    pub to_hit: ToHit,
    pub self_on_hit: Vec<Action>,
    pub self_on_block: Vec<Action>,
    pub target_on_hit: Vec<Action>,
    pub target_on_block: Vec<Action>,
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

    pub fn with_to_self_on_hit(self, additional_actions: Vec<Action>) -> Attack {
        Attack {
            self_on_hit: [self.self_on_hit.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_self_on_block(self, additional_actions: Vec<Action>) -> Attack {
        Attack {
            self_on_block: [self.self_on_block.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_target_on_hit(self, additional_actions: Vec<Action>) -> Attack {
        Attack {
            target_on_hit: [self.target_on_hit.clone(), additional_actions].concat(),
            ..self
        }
    }

    pub fn with_to_target_on_block(self, additional_actions: Vec<Action>) -> Attack {
        Attack {
            target_on_block: [self.target_on_block.clone(), additional_actions].concat(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum StunType {
    Launcher,
    Stun(usize),
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub struct CommonAttackProps {
    pub damage: usize,
    pub knock_back: Vec2,
    pub push_back: Vec2,
    pub stun: StunType,
}

impl Default for CommonAttackProps {
    fn default() -> Self {
        Self {
            damage: 5,
            knock_back: -Vec2::X,
            push_back: -Vec2::X,
            stun: StunType::Stun(15),
        }
    }
}

impl CommonAttackProps {
    pub fn self_on_hit(self) -> Vec<Action> {
        vec![Movement::impulse(self.push_back).into()]
    }
    pub fn self_on_block(self) -> Vec<Action> {
        vec![Movement::impulse(2.0 * self.push_back).into()]
    }

    pub fn target_on_hit(self) -> Vec<Action> {
        vec![
            Action::TakeDamage(self.damage),
            self.get_stun(false),
            Movement::impulse(self.knock_back).into(),
        ]
    }

    pub fn target_on_block(self) -> Vec<Action> {
        vec![
            Action::TakeDamage(1), // Chip
            self.get_stun(true),
            Movement::impulse(0.5 * self.knock_back).into(),
        ]
    }

    fn get_stun(&self, blocked: bool) -> Action {
        match self.stun {
            StunType::Launcher => Action::Launch,
            StunType::Stun(frames) => {
                if blocked {
                    Action::BlockStun(frames)
                } else {
                    Action::HitStun(frames)
                }
            }
        }
    }
}
