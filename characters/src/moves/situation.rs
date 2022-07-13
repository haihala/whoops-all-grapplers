use bevy::{prelude::*, utils::HashSet};
use bevy_inspector_egui::Inspectable;
use types::GameButton;

use crate::{resources::Cost, Inventory, Resources};

use super::{MoveId, MoveType, Requirements};

/// Situation is supposed to contain everything needed to deduce the next phase of a move
#[derive(Inspectable, Eq, PartialEq, Debug, Component, Clone, Default)]
pub struct MoveSituation {
    // Owned
    pub start_frame: i32,
    pub phase_index: usize,
    pub move_id: MoveId,
    pub hit_registered: bool,
    pub move_type: Option<MoveType>,
    pub cancellable: bool,
    pub cancellable_since: Option<usize>,

    // Other components
    // Clone into this whenever initialized or changed
    pub cost: Option<Cost>,
    pub resources: Resources,
    pub inventory: Inventory,
    #[inspectable(ignore)]
    pub buttons_held: HashSet<GameButton>,
    pub grounded: bool,
}
impl MoveSituation {
    pub fn fulfills(&self, requirements: &Requirements, next_move_type: Option<MoveType>) -> bool {
        if let Some(hit_requirement) = requirements.has_hit {
            if hit_requirement != self.hit_registered {
                return false;
            }
        }

        if let Some(grounded) = requirements.grounded {
            if grounded != self.grounded {
                return false;
            }
        }

        if let Some(required_buttons) = requirements.buttons_held.clone() {
            if !required_buttons
                .iter()
                .all(|button| self.buttons_held.contains(button))
            {
                return false;
            }
        }

        if let Some(required_items) = requirements.items.clone() {
            if !required_items
                .iter()
                .all(|item| self.inventory.contains(item))
            {
                return false;
            }
        }

        if !self.resources.can_afford(&requirements.cost) {
            return false;
        }

        if let (Some(move_type), Some(nmt)) = (self.move_type, next_move_type) {
            if !self.cancellable {
                return false;
            }

            // Prevent canceling normals into normals and specials into specials by default
            // Allow canceling anything if more bar is spent
            if move_type >= nmt
                && requirements.cost.unwrap_or_default().meter
                    <= self.cost.unwrap_or_default().meter
            {
                return false;
            }
        }

        true
    }

    pub fn register_hit(&mut self) {
        self.hit_registered = true;
    }
}
