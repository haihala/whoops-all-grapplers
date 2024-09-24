use bevy::prelude::*;

use crate::{ActionEvent, ToHit};

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub struct Attack {
    pub to_hit: ToHit,
    pub self_on_hit: Vec<ActionEvent>,
    pub self_on_block: Vec<ActionEvent>,
    pub target_on_hit: Vec<ActionEvent>,
    pub target_on_block: Vec<ActionEvent>,
}
