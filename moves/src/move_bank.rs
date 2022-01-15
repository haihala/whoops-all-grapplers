use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use types::{Hitbox, MoveId, Player};

use crate::CancelLevel;

/// Component on players
pub struct MoveBank {
    moves: HashMap<MoveId, Move>,
}

impl MoveBank {
    pub fn new(owner: Player, moves: HashMap<MoveId, Move>) -> MoveBank {
        MoveBank {
            moves: moves
                .into_iter()
                .map(|(id, mut action)| {
                    action.claim(owner);
                    (id, action)
                })
                .collect(),
        }
    }

    pub fn get(&self, id: MoveId) -> &Move {
        assert!(self.moves.contains_key(&id));
        self.moves.get(&id).unwrap()
    }

    pub fn get_inputs(&self) -> HashMap<MoveId, &str> {
        self.moves
            .iter()
            .map(|(key, value)| (*key, value.input))
            .collect()
    }

    pub fn get_hitboxes(&self) -> HashMap<MoveId, Hitbox> {
        self.moves
            .iter()
            .filter_map(|(key, value)| {
                value.phases.iter().find_map(|phase| {
                    if let PhaseKind::Hitbox(hitbox) = &phase.kind {
                        Some((key.to_owned(), hitbox.to_owned()))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Default, Inspectable, Clone)]
pub struct Move {
    pub input: &'static str,
    pub cancel_level: CancelLevel,
    pub phases: Vec<Phase>,
    pub meter_cost: i32,
    pub air_ok: bool,
    pub ground_ok: bool,
}

impl Move {
    pub fn get_phase(&self, start_frame: usize, current_frame: usize) -> Option<&Phase> {
        let mut frames_left = current_frame as i32 - start_frame as i32;

        for phase in self.phases.iter() {
            frames_left -= phase.duration as i32;

            if frames_left < 0 {
                return Some(phase);
            }
        }

        None
    }

    fn claim(&mut self, owner: Player) {
        for phase in self.phases.iter_mut() {
            phase.kind.claim(owner);
        }
    }
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Phase {
    pub kind: PhaseKind,
    pub duration: usize,
    pub cancel_requirement: CancelLevel,
    pub mobility: Vec3,
}

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub enum PhaseKind {
    Animation,
    Grab {
        range: f32,
    },
    Hitbox(Hitbox),
    Projectile {
        hitbox: Hitbox,
        speed: f32,
        lifetime: Option<usize>,
    },
}
impl Default for PhaseKind {
    fn default() -> Self {
        PhaseKind::Animation
    }
}
impl PhaseKind {
    fn claim(&mut self, owner: Player) {
        match self {
            PhaseKind::Hitbox(mut hitbox) => {
                hitbox.owner = Some(owner);
                *self = PhaseKind::Hitbox(hitbox);
            }
            PhaseKind::Projectile {
                hitbox,
                speed,
                lifetime,
            } => {
                hitbox.owner = Some(owner);
                *self = PhaseKind::Projectile {
                    hitbox: *hitbox,
                    speed: *speed,
                    lifetime: *lifetime,
                };
            }
            _ => {}
        }
    }
}