use bevy::prelude::*;
use types::Player;

use crate::{damage::Health, meter::Meter};

pub struct MeterBar(pub Player);
pub struct HealthBar(pub Player);

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: QuerySet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
    )>,
    players: Query<(&Player, &Health, &Meter)>,
) {
    for (player, health, meter) in players.iter() {
        for (mut style, bar) in bars.q0_mut().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.get_ratio() * constants::HEALTH_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1_mut().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(meter.get_ratio() * constants::METER_BAR_WIDTH);
            }
        }
    }
}
