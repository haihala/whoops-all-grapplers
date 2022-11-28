use bevy::prelude::*;
use wag_core::Player;

use crate::{
    assets::Colors,
    ui::{
        text::TIMER_WIDTH,
        utils::{div, div_style},
        FULL,
    },
};

use super::{ChargeBar, HealthBar, MeterBar};

const HEALTH_BAR_WIDTH: f32 = (100.0 - TIMER_WIDTH) / 2.0; // Relative to wrapper
const HEALTH_BAR_HEIGHT: f32 = 50.0; // Relative to wrapper
const RESOURCE_BAR_WIDTH: f32 = 30.0; // Relative to wrapper
const RESOURCE_BAR_HEIGHT: f32 = 5.0; // Relative to wrapper

pub fn spawn_health_bar(parent: &mut ChildBuilder, color: Color, player: Player) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: match player {
                        // to drain to the middle
                        Player::One => FlexDirection::RowReverse,
                        Player::Two => FlexDirection::Row,
                    },
                    size: Size::new(
                        Val::Percent(HEALTH_BAR_WIDTH),
                        Val::Percent(HEALTH_BAR_HEIGHT),
                    ),
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Player {player} health bar")),
        ))
        .with_children(|health_bar_wrapper| {
            health_bar_wrapper.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(FULL, FULL),
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                },
                HealthBar(player),
            ));
        });
}

pub fn spawn_meter_bars(parent: &mut ChildBuilder, colors: &Colors) {
    resource_bars(
        parent,
        colors.meter.into(),
        MeterBar(Player::One),
        MeterBar(Player::Two),
    );
}

pub fn spawn_charge_bars(parent: &mut ChildBuilder, colors: &Colors) {
    resource_bars(
        parent,
        colors.charge_default.into(),
        ChargeBar(Player::One),
        ChargeBar(Player::Two),
    );
}

fn resource_bars(
    parent: &mut ChildBuilder,
    color: BackgroundColor,
    component_p1: impl Component + std::fmt::Debug,
    component_p2: impl Component + std::fmt::Debug,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(FULL, Val::Percent(RESOURCE_BAR_HEIGHT)),
                padding: UiRect {
                    bottom: Val::Percent(1.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            resource_bar(parent, color, component_p1);
            resource_bar(parent, color, component_p2);
        });
}

fn resource_bar(
    parent: &mut ChildBuilder,
    background_color: BackgroundColor,
    component: impl Component + std::fmt::Debug,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(RESOURCE_BAR_WIDTH), FULL),
                    ..div_style()
                },
                ..div()
            },
            Name::new(format!("{:?}", component)),
        ))
        .with_children(|container| {
            container.spawn((
                NodeBundle {
                    background_color,
                    ..div()
                },
                component,
            ));
        });
}
