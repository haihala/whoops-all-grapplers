use bevy::prelude::*;
use wag_core::{GameState, OnlyShowInGameState, Player};

use crate::assets::{Colors, Fonts};

mod bars;
mod notifications;
mod shop;
mod text;
mod utils;

use bars::{spawn_charge_bars, spawn_health_bar, spawn_meter_bars};
use notifications::setup_toasts;
use text::{setup_round_info_text, spawn_timer};
use utils::*;

pub use notifications::Notifications;

// Top bars
const TOP_CONTAINER_TOP_PAD: f32 = 0.0;
const TOP_CONTAINER_SIDE_PAD: f32 = 5.0;
const TOP_CONTAINER_WIDTH: f32 = 100.0 - 2.0 * TOP_CONTAINER_SIDE_PAD;
const TOP_CONTAINER_HEIGHT: f32 = 10.0;

// Bottom bars
const BOTTOM_CONTAINER_BOTTOM_PAD: f32 = 3.0;
const BOTTOM_CONTAINER_SIDE_PAD: f32 = 3.0;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::new()
                .with_run_criteria(State::on_update(GameState::Combat))
                .with_system(bars::update)
                .with_system(text::update_timer),
        )
        .add_system(notifications::update)
        .add_system(
            // State::on_enter doesn't work for some reason.
            text::update_round_text.with_run_criteria(State::on_update(GameState::PostRound)),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(State::on_update(GameState::Shop))
                .with_system(shop::navigate_shop)
                .with_system(shop::update_slot_visuals.after(shop::navigate_shop))
                .with_system(shop::update_info_panel.after(shop::navigate_shop))
                .with_system(shop::update_inventory_ui.after(shop::navigate_shop)),
        )
        .add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            SystemSet::new()
                .with_system(setup_combat_hud)
                .with_system(shop::setup_shop),
        );
    }
}

fn setup_combat_hud(mut commands: Commands, colors: Res<Colors>, fonts: Res<Fonts>) {
    setup_top_bars(&mut commands, &colors, &fonts);
    setup_bottom_bars(&mut commands, &colors);
    setup_round_info_text(&mut commands, &colors, &fonts);
    setup_toasts(&mut commands);
}

fn setup_top_bars(commands: &mut Commands, colors: &Colors, fonts: &Fonts) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(
                        Val::Percent(TOP_CONTAINER_WIDTH),
                        Val::Percent(TOP_CONTAINER_HEIGHT),
                    ),
                    position: UiRect {
                        top: Val::Percent(TOP_CONTAINER_TOP_PAD),
                        left: Val::Percent(TOP_CONTAINER_SIDE_PAD),
                        ..default()
                    },
                    ..div_style()
                },
                ..div()
            },
            Name::new("Top bar"),
            OnlyShowInGameState(vec![GameState::Combat]),
        ))
        .with_children(|top_bar_wrapper| {
            spawn_health_bar(top_bar_wrapper, colors, fonts, Player::One);
            spawn_timer(top_bar_wrapper, fonts.basic.clone());
            spawn_health_bar(top_bar_wrapper, colors, fonts, Player::Two);
        });
}

fn setup_bottom_bars(commands: &mut Commands, colors: &Colors) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    size: Size::new(FULL, FULL),
                    position: UiRect {
                        bottom: Val::Percent(BOTTOM_CONTAINER_BOTTOM_PAD),
                        left: Val::Percent(BOTTOM_CONTAINER_SIDE_PAD),
                        ..default()
                    },

                    ..default()
                },
                ..div()
            },
            Name::new("Bottom bars"),
            OnlyShowInGameState(vec![GameState::Combat]),
        ))
        .with_children(|parent| {
            spawn_meter_bars(parent, colors);
            spawn_charge_bars(parent, colors);
        });
}
