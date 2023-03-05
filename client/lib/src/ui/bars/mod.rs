use bevy::prelude::*;
use wag_core::Player;

mod spawn;
mod update;

pub use spawn::{spawn_charge_bars, spawn_health_bar, spawn_meter_bars};
pub use update::update;

#[derive(Debug, Component, Deref)]
pub struct HealthBar(pub Player);
#[derive(Debug, Component, Deref)]
pub struct ScoreText(pub Player);
#[derive(Debug, Component, Deref)]
pub struct MeterBar(pub Player);
#[derive(Debug, Component)]
pub struct SpecialResourceBar(pub Player, usize);
