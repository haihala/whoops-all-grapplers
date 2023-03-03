use bevy::prelude::*;
use wag_core::{Clock, GameState, OnlyShowInGameState, RoundLog, RoundTimer, COMBAT_DURATION};

use crate::assets::{Colors, Fonts};

mod timer;
pub use timer::{spawn_timer, TIMER_WIDTH};

use super::utils::{div, div_style, FULL};

#[derive(Component)]
pub struct RoundText;

pub fn update_timer(mut query: Query<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.single_mut().sections[0].value =
        (COMBAT_DURATION - clock.elapsed_time).floor().to_string();
}

pub fn update_round_text(
    mut query: Query<(&mut Visibility, &mut Text), With<RoundText>>,
    round_log: Res<RoundLog>,
) {
    if let Some(result) = round_log.last() {
        let (mut visible, mut text) = query.single_mut();

        visible.is_visible = true;
        text.sections[0].value = if let Some(winner) = result.winner {
            format!("{winner} won the round")
        } else {
            "Tie".to_string()
        }
    }
}

pub(super) fn setup_round_info_text(commands: &mut Commands, colors: &Colors, fonts: &Fonts) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    size: Size::new(FULL, Val::Percent(10.0)),
                    position: UiRect {
                        top: Val::Percent(40.0),
                        left: Val::Px(0.0),
                        ..default()
                    },
                    ..div_style()
                },
                ..div()
            },
            Name::new("Round info text"),
            OnlyShowInGameState(vec![GameState::Loading, GameState::PostRound]),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "New round",
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 100.0,
                            color: colors.text,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    }),
                    ..default()
                },
                RoundText,
            ));
        });
}
