use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use types::{GrabDescription, MoveId, SpawnDescriptor};

use crate::CancelLevel;

#[derive(Debug, Default, Component, Clone)]
pub struct MoveBank {
    moves: HashMap<MoveId, Move>,
}

impl MoveBank {
    pub fn new(moves: HashMap<MoveId, Move>) -> MoveBank {
        MoveBank { moves }
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
    pub fn get_phase_index(&self, start_frame: usize, current_frame: usize) -> Option<usize> {
        let mut frames_left = current_frame as i32 - start_frame as i32;

        for (index, phase) in self.phases.iter().enumerate() {
            frames_left -= phase.duration as i32;
            if frames_left < 0 {
                return Some(index);
            }
        }
        None
    }

    pub fn get_phase(&self, index: usize) -> Phase {
        self.phases.get(index).unwrap().to_owned()
    }
}

#[derive(Debug, Inspectable, Copy, Clone, PartialEq)]
pub enum MoveMobility {
    Impulse(Vec3),
    Perpetual(Vec3),
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
