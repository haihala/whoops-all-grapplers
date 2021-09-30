use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{Colors, Health, Meter, Player};

#[derive(Inspectable, Default)]
struct MeterBar(i32);
struct HealthBar(i32);

pub struct BarsPlugin;

impl Plugin for BarsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update.system());
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    // P1 Health
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
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
        .insert(HealthBar(1));

    // P1 Meter
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
                position: Rect {
                    bottom: Val::Percent(2.0),
                    right: Val::Percent(crate::HEALTH_BAR_ANCHOR),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.meter.clone(),
            ..Default::default()
        })
        .insert(MeterBar(1));

    // P2 Health
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
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
        .insert(HealthBar(2));

    // P2 Meter
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
                position: Rect {
                    bottom: Val::Percent(2.0),
                    left: Val::Percent(crate::HEALTH_BAR_ANCHOR),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.meter.clone(),
            ..Default::default()
        })
        .insert(MeterBar(2));
}

fn update(
    mut bars: QuerySet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
    )>,
    players: Query<(&Player, &Health, &Meter)>,
) {
    for (player, health, meter) in players.iter() {
        for (mut style, bar) in bars.q0_mut().iter_mut() {
            if player.0 == bar.0 {
                style.size.width = Val::Percent(health.ratio * crate::RESOURCE_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1_mut().iter_mut() {
            if player.0 == bar.0 {
                style.size.width = Val::Percent(meter.ratio * crate::RESOURCE_BAR_WIDTH);
            }
        }
    }
}
