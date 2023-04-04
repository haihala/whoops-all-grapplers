use bevy::prelude::*;

mod bars;
pub use bars::update_bars;

mod notifications;
pub use notifications::{update_notifications, Notifications};

mod round_timer;
pub use round_timer::update_timer;

use characters::{BarRenderInstructions, Properties, PropertyType};
use wag_core::{GameState, OnlyShowInGameState, Player, Players};

use crate::assets::{Colors, Fonts};

use self::bars::{PropertyBar, ScoreText};

pub fn setup_combat_hud(
    mut commands: Commands,
    colors: Res<Colors>,
    fonts: Res<Fonts>,
    properties: Query<&Properties>,
    players: Res<Players>,
) {
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                    },
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        ..default()
                    },
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            OnlyShowInGameState(vec![GameState::Combat, GameState::PostRound]),
            Name::new("Combat UI container"),
        ))
        .id();

    let timer_width = 15.0;
    let side_width = (100.0 - timer_width) / 2.0;

    setup_player_hud(
        &mut commands,
        container,
        side_width,
        &colors,
        &fonts,
        Player::One,
        properties.get(players.one).unwrap(),
    );
    round_timer::setup_timer(&mut commands, container, fonts.basic.clone(), timer_width);
    setup_player_hud(
        &mut commands,
        container,
        side_width,
        &colors,
        &fonts,
        Player::Two,
        properties.get(players.two).unwrap(),
    );
}

fn setup_player_hud(
    commands: &mut Commands,
    parent: Entity,
    width_percentage: f32,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
    properties: &Properties,
) {
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(3.0)),
                size: Size::new(Val::Percent(width_percentage), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .set_parent(parent)
        .id();

    setup_top_hud(commands, container, colors, fonts, player);
    notifications::setup_toasts(commands, container, player);
    setup_bottom_hud(commands, container, player, properties);
}

fn setup_top_hud(
    commands: &mut Commands,
    parent: Entity,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
) {
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: match player {
                        // Align towards the center
                        Player::One => AlignItems::FlexEnd,
                        Player::Two => AlignItems::FlexStart,
                    },
                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Player {player} health bar wrapper")),
        ))
        .set_parent(parent)
        .id();

    bars::setup_bar(
        commands,
        container,
        BarRenderInstructions::default_health(),
        PropertyBar(player, PropertyType::Health),
        "Health bar",
    );
    setup_round_counter(commands, container, colors, fonts, player);
}

fn setup_round_counter(
    commands: &mut Commands,
    parent: Entity,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
) {
    commands
        .spawn((
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 18.0,
                    color: colors.text,
                },
            ),
            ScoreText(player),
            Name::new("Round counter"),
        ))
        .set_parent(parent);
}

fn setup_bottom_hud(
    commands: &mut Commands,
    parent: Entity,
    player: Player,
    properties: &Properties,
) {
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: match player {
                    // Align towards the side of the screen
                    Player::One => AlignItems::FlexStart,
                    Player::Two => AlignItems::FlexEnd,
                },
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                ..default()
            },
            ..default()
        })
        .set_parent(parent)
        .id();

    for (prop_type, property) in properties.iter() {
        if matches!(prop_type, PropertyType::Health | PropertyType::Meter) {
            continue;
        }

        bars::setup_bar(
            commands,
            container,
            property.render_instructions.clone(),
            PropertyBar(player, *prop_type),
            format!("Special resource bar {:?}", prop_type),
        );
    }

    bars::setup_bar(
        commands,
        container,
        BarRenderInstructions::default_meter(),
        PropertyBar(player, PropertyType::Meter),
        "Meter bar",
    );
}
