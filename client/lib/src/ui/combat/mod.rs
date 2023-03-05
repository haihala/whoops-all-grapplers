use bevy::prelude::*;

mod bars;
pub use bars::update_bars;

mod notifications;
pub use notifications::{update_notifications, Notifications};

mod round_timer;
pub use round_timer::update_timer;

use characters::{BarRenderInstructions, Character};
use wag_core::{GameState, OnlyShowInGameState, Player, Players};

use crate::{
    assets::{Colors, Fonts},
    ui::combat::bars::HealthBar,
};

use self::bars::{MeterBar, ScoreText, SpecialResourceBar};

pub fn setup_combat_hud(
    mut commands: Commands,
    colors: Res<Colors>,
    fonts: Res<Fonts>,
    characters: Query<&Character>,
    players: Res<Players>,
) {
    commands
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
        .with_children(|root| {
            let timer_width = 15.0;
            let side_width = (100.0 - timer_width) / 2.0;

            setup_player_hud(
                root,
                side_width,
                &colors,
                &fonts,
                Player::One,
                characters.get(players.one).unwrap(),
            );
            round_timer::setup_timer(root, fonts.basic.clone(), timer_width);
            setup_player_hud(
                root,
                side_width,
                &colors,
                &fonts,
                Player::Two,
                characters.get(players.two).unwrap(),
            );
        });
}

fn setup_player_hud(
    root: &mut ChildBuilder,
    width_percentage: f32,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
    character: &Character,
) {
    root.spawn(NodeBundle {
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
    .with_children(|cb| {
        setup_top_hud(cb, colors, fonts, player);
        notifications::setup_toasts(cb, player);
        setup_bottom_hud(cb, player, character);
    });
}

fn setup_top_hud(root: &mut ChildBuilder, colors: &Colors, fonts: &Fonts, player: Player) {
    root.spawn((
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
    .with_children(|cb| {
        bars::setup_bar(
            cb,
            BarRenderInstructions::default_health(),
            HealthBar(player),
            "Health bar",
        );
        setup_round_counter(cb, colors, fonts, player);
    });
}

fn setup_round_counter(root: &mut ChildBuilder, colors: &Colors, fonts: &Fonts, player: Player) {
    root.spawn((
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
    ));
}

fn setup_bottom_hud(root: &mut ChildBuilder, player: Player, character: &Character) {
    root.spawn(NodeBundle {
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
    .with_children(|cb| {
        for (index, property) in character.special_properties.iter().enumerate() {
            bars::setup_bar(
                cb,
                property.render_instructions.clone(),
                SpecialResourceBar(player, index),
                format!("Special resource bar {}", index),
            );
        }

        bars::setup_bar(
            cb,
            BarRenderInstructions::default_meter(),
            MeterBar(player),
            "Meter bar",
        );
    });
}
