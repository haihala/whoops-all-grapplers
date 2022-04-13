use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;
use bitflags::bitflags;

use crate::{CancelLevel, MoveAction, MoveId, Phase};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug, Default)]
pub struct MoveState {
    pub start_frame: usize,
    pub phase_index: usize,
    pub move_id: MoveId,
    pub situation: MoveFlags,
}
impl MoveState {
    pub fn register_hit(&mut self) {
        self.situation |= MoveFlags::HIT;
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
            .filter_map(|(key, move_data)| move_data.input.map(|input| (*key, input)))
            .collect()
    }
}

bitflags! {
    #[derive(Default, Inspectable)]
    pub struct MoveStartCondition: u32 {
        const AIR = 0b00000001;
        const GROUND = 0b00000010;
    }

    #[derive(Default, Inspectable)]
    pub struct MoveFlags: u32 {
        const HIT = 0b00000001;
        const DRUGS = 0b00000010;
        const GRAB_PRESSED = 0b00000100;
        const FAST_PRESSED = 0b00001000;
        const STRONG_PRESSED = 0b00010000;
        const TAUNT_PRESSED = 0b00100000;
        const EQUIPMENT_PRESSED = 0b01000000;
    }
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct MoveCost {
    pub meter: i32,
    pub charge: bool,
}

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Move {
    pub input: Option<&'static str>,
    pub cancel_level: CancelLevel,
    pub phases: Vec<ConditionResolver>,
    pub cost: MoveCost,
    pub conditions: MoveStartCondition,
}

impl Move {
    pub fn get_action_index(&self, state: MoveState, current_frame: usize) -> Option<usize> {
        // Can be negative, which is why cast before operation
        let mut frames_left = current_frame as i32 - state.start_frame as i32;

        for (index, phase) in self
            .phases
            .iter()
            .map(|meta| meta.get(state.situation))
            .enumerate()
        {
            if let Some(duration) = phase.get_duration() {
                frames_left -= duration as i32;
                if frames_left < 0 {
                    return Some(index);
                }
            } else {
                // Current instruction is a move, it should be returned despite the time.
                return Some(index);
            }
        }
        None
    }

    pub fn get_action(&self, state: MoveState) -> MoveAction {
        self.phases
            .get(state.phase_index)
            .unwrap()
            .to_owned()
            .get(state.situation)
    }
}

#[derive(Debug, Inspectable, Clone, PartialEq)]
pub struct ConditionResolver {
    pub default: MoveAction,
    pub branches: Vec<(MoveFlags, MoveAction)>, // This way order is maintained
}
impl ConditionResolver {
    pub fn get(&self, situation: MoveFlags) -> MoveAction {
        for (cond, phase) in &self.branches {
            if situation.contains(*cond) {
                return phase.to_owned();
            }
        }
        self.default.to_owned()
    }
}
impl From<Phase> for ConditionResolver {
    fn from(phase: Phase) -> Self {
        ConditionResolver {
            default: phase.into(),
            branches: vec![],
        }
    }
}
impl From<MoveId> for ConditionResolver {
    fn from(id: MoveId) -> Self {
        ConditionResolver {
            default: id.into(),
            branches: vec![],
        }
    }
}
impl Default for ConditionResolver {
    fn default() -> Self {
        Phase::default().into()
    }
}
