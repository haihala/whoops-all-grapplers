use std::sync::Arc;

use bevy::prelude::*;

use crate::{HitEffect, HitInfo, Situation, ToHit};

pub type OnHitEffect = Arc<dyn Fn(&Situation, &HitInfo) -> HitEffect + Send + Sync>;

#[derive(Component, Clone)]
pub struct Attack {
    pub to_hit: ToHit,
    pub on_hit: OnHitEffect,
}
