use bevy::prelude::*;

mod attack;
pub use attack::{Attack, CommonAttackProps, StunType};

mod to_hit;
pub use to_hit::{AttackHeight, BlockType, Hitbox, Lifetime, Projectile, ToHit};

mod situation_shorthands;
pub use situation_shorthands::*;

mod move_history;
pub use move_history::MoveHistory;

mod move_situation;
pub use move_situation::Situation;

mod move_data;
pub use move_data::Move;

mod move_phases;
pub use move_phases::{Action, CancelPolicy, FlowControl, Movement};

mod hit_tracker;
pub use hit_tracker::*;

mod targets;
pub use targets::Hurtbox;

#[derive(PartialEq, PartialOrd, Debug, Reflect, Clone, Copy, Eq, Default)]
pub enum MoveType {
    #[default]
    Normal,
    Special,
}

#[test]
fn move_type_ord() {
    assert!(MoveType::Normal < MoveType::Special)
}
