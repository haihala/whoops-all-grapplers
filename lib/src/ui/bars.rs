use bevy::prelude::*;
use types::Player;

use crate::{damage::Health, meter::Meter};

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
    players: Query<(&Player, &Health, &Meter)>,
) {
    for (player, health, meter) in players.iter() {
        for (mut style, bar) in bars.q0().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.get_ratio() * HEALTH_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(meter.get_ratio() * METER_BAR_WIDTH);
            }
        }
    }
}
