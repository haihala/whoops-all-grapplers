use bevy::prelude::*;
use moves::{Move, MoveFlags};
use types::MoveId;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    pub move_flag: Option<MoveFlags>,
    pub new_moves: Vec<(MoveId, Move)>,
    pub item_type: Option<ItemType>,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Gi(Gi),
    Gun(Gun),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Gi {
    pub window_opened: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Gun {
    pub max_ammo: usize,
    pub ammo: usize,
}
