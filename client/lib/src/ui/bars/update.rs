use bevy::prelude::*;
use characters::Resources;
use wag_core::{Player, RoundLog};

use crate::{assets::Colors, damage::Health};

use super::{ChargeBar, HealthBar, MeterBar, ScoreText};

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: ParamSet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
        Query<(&mut Style, &mut BackgroundColor, &ChargeBar)>,
        Query<(&mut Text, &ScoreText)>,
    )>,
    players: Query<(&Player, &Health, &Resources)>,
    round_log: Res<RoundLog>,
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
        for (mut text, score_text) in &mut bars.p3() {
            // TODO This could be moved elsewhere, but that is true for the others as well
            // Don't want to optimize prematurely
            if *player == **score_text {
                text.sections[0].value = round_log.wins(*player).to_string();
            }
        }
    }
}
