use bevy::prelude::*;
use time::{Clock, GameState, RoundResult, RoundTimer, ROUND_TIME};

use crate::assets::{Colors, Fonts};

use super::utils::{div, div_style, FULL};

#[derive(Component)]
pub struct RoundText;

pub fn update_timer(mut query: Query<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.single_mut().sections[0].value = (ROUND_TIME - clock.elapsed_time).floor().to_string();
}

pub fn hide_round_text(
    mut query: Query<&mut Visibility, With<RoundText>>,
    state: Res<State<GameState>>,
) {
    query.single_mut().is_visible = *state.current() != GameState::Combat;
}

pub fn update_round_text(
    mut query: Query<(&mut Visibility, &mut Text), With<RoundText>>,
    round_result: Option<Res<RoundResult>>,
) {
    if let Some(result) = round_result {
        let (mut visible, mut text) = query.single_mut();

        visible.is_visible = true;
        text.sections[0].value = if let Some(winner) = result.winner {
            format!("{} won the round", winner)
        } else {
            "Tie".to_string()
        }
    }
}

pub(super) fn setup_round_info_text(commands: &mut Commands, colors: &Colors, fonts: &Fonts) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(FULL, Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(40.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..div_style()
            },
            ..div()
        })
        .insert(Name::new("Round info text"))
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
