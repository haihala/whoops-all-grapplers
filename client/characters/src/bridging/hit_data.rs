use bevy::prelude::*;
use foundation::Stats;

use crate::ActionEvent;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub avoided: bool,
    pub airborne: bool,
    pub hitbox_pos: Vec2,
    pub defender_stats: Stats,
}

#[derive(Clone)]
pub struct HitEffect {
    pub attacker: Vec<ActionEvent>,
    pub defender: Vec<ActionEvent>,
}
