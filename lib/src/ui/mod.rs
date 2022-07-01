use bevy::prelude::*;
use time::{GameState, RoundTimer, ROUND_TIME};
use types::Player;

use crate::assets::{Colors, Fonts};

mod bars;
mod text;

use bars::{ChargeBar, HealthBar, MeterBar};
use text::RoundText;

// Top bars
const TOP_CONTAINER_TOP_PAD: f32 = 0.0;
const TOP_CONTAINER_SIDE_PAD: f32 = 5.0;
const TOP_CONTAINER_WIDTH: f32 = 100.0 - 2.0 * TOP_CONTAINER_SIDE_PAD;
const TOP_CONTAINER_HEIGHT: f32 = 10.0;

const TIMER_WIDTH: f32 = 10.0;
const TIMER_TOP_PADDING: f32 = 2.0;
const HEALTH_BAR_WIDTH: f32 = (100.0 - TIMER_WIDTH) / 2.0; // Relative to wrapper
const HEALTH_BAR_HEIGHT: f32 = 50.0; // Relative to wrapper

// Bottom bars
const BOTTOM_CONTAINER_BOTTOM_PAD: f32 = 3.0;
const BOTTOM_CONTAINER_SIDE_PAD: f32 = 3.0;
const BOTTOM_CONTAINER_WIDTH: f32 = 100.0 - 2.0 * BOTTOM_CONTAINER_SIDE_PAD; // Relative to screen
const BOTTOM_CONTAINER_HEIGHT: f32 = 10.0; // Relative to screen
const RESOURCE_BAR_WIDTH: f32 = 30.0; // Relative to wrapper
const RESOURCE_BAR_HEIGHT: f32 = 45.0; // Relative to wrapper (BOTTOM_CONTAINER_HEIGHT)

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::new()
                .with_system(bars::update)
                .with_system(
                    text::update_timer.with_run_criteria(State::on_update(GameState::Combat)),
                )
                .with_system(text::hide_round_text.after(text::update_timer))
                .with_system(text::update_round_text.after(text::hide_round_text)),
        );
    }
}

fn setup_ui(mut commands: Commands, colors: Res<Colors>, fonts: Res<Fonts>) {
    setup_top_bars(&mut commands, &colors, &fonts);
    setup_bottom_bars(&mut commands, &colors);
    setup_round_info_text(&mut commands, &colors, &fonts);
}

fn setup_top_bars(commands: &mut Commands, colors: &Colors, fonts: &Fonts) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                size: Size::new(
                    Val::Percent(TOP_CONTAINER_WIDTH),
                    Val::Percent(TOP_CONTAINER_HEIGHT),
                ),
                position: Rect {
                    top: Val::Percent(TOP_CONTAINER_TOP_PAD),
                    left: Val::Percent(TOP_CONTAINER_SIDE_PAD),
                    ..default()
                },
                ..default()
            },
            color: TRANSPARENT.into(),
            ..default()
        })
        .with_children(|top_bar_wrapper| {
            top_bar_wrapper
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(HEALTH_BAR_WIDTH),
                            Val::Percent(HEALTH_BAR_HEIGHT),
                        ),
                        ..default()
                    },
                    color: colors.health.into(),
                    ..default()
                })
                .insert(HealthBar(Player::One));
            top_bar_wrapper
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(TIMER_WIDTH), Val::Percent(100.0)),
                        position: Rect {
                            top: Val::Percent(TIMER_TOP_PADDING),
                            ..default()
                        },
                        ..default()
                    },
                    color: TRANSPARENT.into(),
                    ..default()
                })
                .with_children(|timer_wrapper| {
                    timer_wrapper
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                ROUND_TIME.round().to_string(),
                                TextStyle {
                                    font: fonts.basic.clone(),
                                    font_size: 100.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    vertical: VerticalAlign::Center,
                                },
                            ),
                            ..default()
                        })
                        .insert(RoundTimer);
                });
            top_bar_wrapper
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(HEALTH_BAR_WIDTH),
                            Val::Percent(HEALTH_BAR_HEIGHT),
                        ),
                        ..default()
                    },
                    color: colors.health.into(),
                    ..default()
                })
                .insert(HealthBar(Player::Two));
        });
}

fn setup_bottom_bars(commands: &mut Commands, colors: &Colors) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(
                    Val::Percent(BOTTOM_CONTAINER_WIDTH),
                    Val::Percent(BOTTOM_CONTAINER_HEIGHT),
                ),
                position: Rect {
                    bottom: Val::Percent(BOTTOM_CONTAINER_BOTTOM_PAD),
                    left: Val::Percent(BOTTOM_CONTAINER_SIDE_PAD),
                    ..default()
                },
                ..default()
            },
            color: TRANSPARENT.into(),
            ..default()
        })
        .with_children(|parent| {
            meter_bars(parent, colors);
            charge_bars(parent, colors);
        });
}

fn meter_bars(parent: &mut ChildBuilder, colors: &Colors) {
    resource_bars(
        parent,
        colors.meter.into(),
        MeterBar(Player::One),
        MeterBar(Player::Two),
    );
}

fn charge_bars(parent: &mut ChildBuilder, colors: &Colors) {
    resource_bars(
        parent,
        colors.charge_default.into(),
        ChargeBar(Player::One),
        ChargeBar(Player::Two),
    );
}

fn resource_bars(
    parent: &mut ChildBuilder,
    color: UiColor,
    component_p1: impl Component,
    component_p2: impl Component,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(100.0), Val::Percent(RESOURCE_BAR_HEIGHT)),
                ..default()
            },
            color: TRANSPARENT.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(RESOURCE_BAR_WIDTH), Val::Percent(100.0)),
                        ..default()
                    },
                    color,
                    ..default()
                })
                .insert(component_p1);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(RESOURCE_BAR_WIDTH), Val::Percent(100.0)),
                        ..default()
                    },
                    color,
                    ..default()
                })
                .insert(component_p2);
        });
}

fn setup_round_info_text(commands: &mut Commands, colors: &Colors, fonts: &Fonts) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(40.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: TRANSPARENT.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "New round",
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 100.0,
                            color: colors.text,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(RoundText);
        });
}
