use bevy::prelude::*;
use characters::Resources;
use types::Player;

use crate::{assets::Colors, damage::Health};

use super::{ChargeBar, HealthBar, MeterBar};

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: ParamSet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
        Query<(&mut Style, &mut UiColor, &ChargeBar)>,
    )>,
    players: Query<(&Player, &Health, &Resources)>,
    colors: Res<Colors>,
) {
    for (player, health, resources) in &players {
        for (mut style, bar) in &mut bars.p0() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.get_percentage());
            }
        }
        for (mut style, bar) in &mut bars.p1() {
            if *player == bar.0 {
                style.size.width = Val::Percent(resources.meter.get_percentage());
            }
        }
        for (mut style, mut color, bar) in &mut bars.p2() {
            if *player == bar.0 {
                style.size.width = Val::Percent(resources.charge.get_percentage());
                *color = if resources.charge.is_charged() {
                    colors.charge_full.into()
                } else {
                    colors.charge_default.into()
                };
            }
        }
    }
}
