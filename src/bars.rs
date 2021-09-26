use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{Colors, Health, Player};

#[derive(Inspectable, Default)]
struct HealthBar(i32);

pub struct BarsPlugin;

impl Plugin for BarsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update.system());
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::HEALTH_BAR_WIDTH), Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(2.0),
                    right: Val::Percent(crate::HEALTH_BAR_ANCHOR),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.health.clone(),
            ..Default::default()
        })
        .insert(HealthBar(1)); // P1

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::HEALTH_BAR_WIDTH), Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(2.0),
                    left: Val::Percent(crate::HEALTH_BAR_ANCHOR),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.health.clone(),
            ..Default::default()
        })
        .insert(HealthBar(2)); // P2
}

fn update(mut bars: Query<(&mut Style, &HealthBar)>, players: Query<(&Player, &Health)>) {
    for (mut style, bar) in bars.iter_mut() {
        for (player, health) in players.iter() {
            if player.0 == bar.0 {
                style.size.width = Val::Percent(health.ratio * crate::HEALTH_BAR_WIDTH);
            }
        }
    }
}
