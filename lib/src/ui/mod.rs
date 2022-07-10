use bevy::prelude::*;
use time::GameState;
use types::Player;

use crate::assets::{Colors, Fonts};

mod bars;
mod text;
mod utils;

use bars::{spawn_charge_bars, spawn_health_bar, spawn_meter_bars};
use utils::*;

use self::text::{setup_round_info_text, spawn_timer};

// Top bars
const TOP_CONTAINER_TOP_PAD: f32 = 0.0;
const TOP_CONTAINER_SIDE_PAD: f32 = 5.0;
const TOP_CONTAINER_WIDTH: f32 = 100.0 - 2.0 * TOP_CONTAINER_SIDE_PAD;
const TOP_CONTAINER_HEIGHT: f32 = 10.0;

// Bottom bars
const BOTTOM_CONTAINER_BOTTOM_PAD: f32 = 3.0;
const BOTTOM_CONTAINER_SIDE_PAD: f32 = 3.0;
const BOTTOM_CONTAINER_WIDTH: f32 = 100.0 - 2.0 * BOTTOM_CONTAINER_SIDE_PAD; // Relative to screen
const BOTTOM_CONTAINER_HEIGHT: f32 = 10.0; // Relative to screen

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
                ..div_style()
            },
            ..div()
        })
        .insert(Name::new("Top bar"))
        .with_children(|top_bar_wrapper| {
            spawn_health_bar(top_bar_wrapper, colors.health, Player::One);
            spawn_timer(top_bar_wrapper, fonts.basic.clone());
            spawn_health_bar(top_bar_wrapper, colors.health, Player::Two);
        });
}

fn setup_bottom_bars(commands: &mut Commands, colors: &Colors) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
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
            ..div()
        })
        .insert(Name::new("Bottom bars"))
        .with_children(|parent| {
            spawn_meter_bars(parent, colors);
            spawn_charge_bars(parent, colors);
        });
}
