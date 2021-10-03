use bevy::prelude::*;

use crate::{Colors, Health, Meter, Player};

struct MeterBar(Player);
struct HealthBar(Player);

pub struct BarsPlugin;

impl Plugin for BarsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update.system());
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    create_healthbar(&mut commands, &colors, Player::One);
    create_meterbar(&mut commands, &colors, Player::One);

    create_healthbar(&mut commands, &colors, Player::Two);
    create_meterbar(&mut commands, &colors, Player::Two);
}

fn create_healthbar(commands: &mut Commands, colors: &Res<Colors>, player: Player) {
    let base_position = Rect {
        top: Val::Percent(2.0),
        ..Default::default()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
                position: get_bar_position(base_position, player),
                ..Default::default()
            },
            material: colors.health.clone(),
            ..Default::default()
        })
        .insert(HealthBar(player));
}

fn create_meterbar(commands: &mut Commands, colors: &Res<Colors>, player: Player) {
    let base_position = Rect {
        bottom: Val::Percent(2.0),
        ..Default::default()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(crate::RESOURCE_BAR_WIDTH), Val::Percent(10.0)),
                position: get_bar_position(base_position, player),
                ..Default::default()
            },
            material: colors.meter.clone(),
            ..Default::default()
        })
        .insert(MeterBar(player));
}

fn get_bar_position(base: Rect<Val>, player: Player) -> Rect<Val> {
    match player {
        Player::One => Rect {
            right: Val::Percent(crate::HEALTH_BAR_ANCHOR),
            ..base
        },
        Player::Two => Rect {
            left: Val::Percent(crate::HEALTH_BAR_ANCHOR),
            ..base
        },
    }
}

#[allow(clippy::type_complexity)]
fn update(
    mut bars: QuerySet<(
        Query<(&mut Style, &HealthBar)>,
        Query<(&mut Style, &MeterBar)>,
    )>,
    players: Query<(&Player, &Health, &Meter)>,
) {
    for (player, health, meter) in players.iter() {
        for (mut style, bar) in bars.q0_mut().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(health.ratio * crate::RESOURCE_BAR_WIDTH);
            }
        }
        for (mut style, bar) in bars.q1_mut().iter_mut() {
            if *player == bar.0 {
                style.size.width = Val::Percent(meter.ratio * crate::RESOURCE_BAR_WIDTH);
            }
        }
    }
}
