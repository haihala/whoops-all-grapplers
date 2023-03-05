use bevy::prelude::*;
use characters::Properties;
use wag_core::{Player, RoundLog};

use crate::assets::Colors;

use super::{HealthBar, MeterBar, ScoreText, SpecialResourceBar};

#[allow(clippy::type_complexity)]
pub fn update(
    mut bars: ParamSet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
        Query<(&mut Style, &mut BackgroundColor, &SpecialResourceBar)>,
        Query<(&mut Text, &ScoreText)>,
    )>,
    players: Query<(&Player, &Properties)>,
    round_log: Res<RoundLog>,
    colors: Res<Colors>,
) {
    for (player, properties) in &players {
        for (mut style, bar) in &mut bars.p0() {
            if *player == bar.0 {
                style.size.width = Val::Percent(properties.health.get_percentage());
            }
        }
        for (mut style, bar) in &mut bars.p1() {
            if *player == bar.0 {
                style.size.width = Val::Percent(properties.meter.get_percentage());
            }
        }
        for (mut style, mut color, bar) in &mut bars.p2() {
            if *player == bar.0 {
                let resource = &properties.special_properties[bar.1];

                style.size.width = Val::Percent(resource.get_percentage());

                *color = if resource.is_full() {
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
