use bevy::prelude::*;
use wag_core::Stats;

use crate::ActionEvent;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub avoided: bool,
    pub hitbox_pos: Vec2,
    pub defender_stats: Stats,
}

#[derive(Debug, Clone)]
pub struct HitEffect {
    pub attacker: Vec<ActionEvent>,
    pub defender: Vec<ActionEvent>,
}
