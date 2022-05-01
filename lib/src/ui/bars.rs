use bevy::prelude::*;
use kits::Resources;
use types::Player;

use crate::damage::Health;

use super::{HEALTH_BAR_WIDTH, METER_BAR_WIDTH};

#[derive(Debug, Component)]
pub struct MeterBar(pub Player);
#[derive(Debug, Component)]
pub struct HealthBar(pub Player);

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: QuerySet<(
        QueryState<(&mut Style, &HealthBar)>,
        QueryState<(&mut Style, &MeterBar)>,
    )>,
    players: Query<(&Player, &Health, &Resources)>,
) {
    for (player, health, resources) in players.iter() {
        for (mut style, bar) in bars.q0().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.get_ratio() * HEALTH_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(resources.meter.get_ratio() * METER_BAR_WIDTH);
            }
        }
    }
}
