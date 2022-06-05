use bevy::prelude::*;
use kits::Resources;
use types::Player;

use crate::{assets::Colors, damage::Health};

use super::{HEALTH_BAR_WIDTH, RESOURCE_BAR_WIDTH};

#[derive(Debug, Component)]
pub struct MeterBar(pub Player);
#[derive(Debug, Component)]
pub struct HealthBar(pub Player);
#[derive(Debug, Component)]
pub struct ChargeBar(pub Player);

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: QuerySet<(
        QueryState<(&mut Style, &HealthBar)>,
        QueryState<(&mut Style, &MeterBar)>,
        QueryState<(&mut Style, &mut UiColor, &ChargeBar)>,
    )>,
    players: Query<(&Player, &Health, &Resources)>,
    colors: Res<Colors>,
) {
    for (player, health, resources) in players.iter() {
        for (mut style, bar) in bars.q0().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.get_ratio() * HEALTH_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(resources.meter.get_ratio() * RESOURCE_BAR_WIDTH);
            }
        }
        for (mut style, mut color, bar) in bars.q2().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(resources.charge.get_ratio() * RESOURCE_BAR_WIDTH);
                *color = if resources.charge.is_charged() {
                    colors.charge_full.into()
                } else {
                    colors.charge_default.into()
                };
            }
        }
    }
}
