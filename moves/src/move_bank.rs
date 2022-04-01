use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;
use bitflags::bitflags;

use types::{GrabDescription, MoveId, SpawnDescriptor};

use crate::CancelLevel;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug, Default)]
pub struct MoveState {
    pub start_frame: usize,
    pub phase_index: usize,
    pub move_id: MoveId,
    pub situation: PhaseCondition,
}
impl MoveState {
    pub fn register_hit(&mut self) {
        self.situation |= PhaseCondition::HIT;
    }
}

#[derive(Debug, Default, Component, Clone)]
pub struct MoveBank {
    moves: HashMap<MoveId, Move>,
}

impl MoveBank {
    pub fn new(moves: HashMap<MoveId, Move>) -> MoveBank {
        MoveBank { moves }
    }

    pub fn register_move(&mut self, id: MoveId, move_data: Move) {
        self.moves.insert(id, move_data);
    }

    pub fn get(&self, id: MoveId) -> &Move {
        self.moves.get(&id).unwrap()
    }

    pub fn get_inputs(&self) -> HashMap<MoveId, &'static str> {
        self.moves
            .iter()
            .map(|(key, value)| (*key, value.input))
            .collect()
    }
}

bitflags! {
    #[derive(Default, Inspectable)]
    pub struct MoveCondition: u32 {
        const AIR = 1;
        const GROUND = 2;
    }

    #[derive(Default, Inspectable)]
    pub struct PhaseCondition: u32 {
        const HIT = 1;
        const DRUGS = 2;
    }
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct MoveCost {
    pub meter: i32,
    pub charge: bool,
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Move {
    pub input: &'static str,
    pub cancel_level: CancelLevel,
    pub phases: Vec<PhaseSwitch>,
    pub cost: MoveCost,
    pub conditions: MoveCondition,
}

impl Move {
    pub fn get_phase_index(&self, state: MoveState, current_frame: usize) -> Option<usize> {
        // Can be negative, which is why cast before operation
        let mut frames_left = current_frame as i32 - state.start_frame as i32;

        for (index, phase) in self
            .phases
            .iter()
            .map(|meta| meta.get(state.situation))
            .enumerate()
        {
            frames_left -= phase.duration as i32;
            if frames_left < 0 {
                return Some(index);
            }
        }
        None
    }

    pub fn get_phase(&self, state: MoveState) -> Phase {
        self.phases
            .get(state.phase_index)
            .unwrap()
            .to_owned()
            .get(state.situation)
    }
}

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub struct PhaseSwitch {
    pub default: Phase,
    pub branches: Vec<(PhaseCondition, Phase)>, // This way order is maintained
}
impl PhaseSwitch {
    pub fn get(&self, situation: PhaseCondition) -> Phase {
        for (cond, phase) in &self.branches {
            if situation.contains(*cond) {
                return phase.to_owned();
            }
        }
        self.default.to_owned()
    }
}
impl From<Phase> for PhaseSwitch {
    fn from(phase: Phase) -> Self {
        PhaseSwitch {
            default: phase,
            branches: vec![],
        }
    }
}
impl Default for PhaseSwitch {
    fn default() -> Self {
        Phase::default().into()
    }
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Phase {
    pub kind: PhaseKind,
    pub duration: usize,
    pub cancellable: bool,
    pub mobility: Option<MoveMobility>,
}

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub enum PhaseKind {
    Animation,
    Grab(GrabDescription),
    Attack(SpawnDescriptor),
}
impl Default for PhaseKind {
    fn default() -> Self {
        PhaseKind::Animation
    }
}

#[derive(Debug, Inspectable, Copy, Clone, PartialEq)]
pub enum MoveMobility {
    Impulse(Vec3),
    Perpetual(Vec3),
}
