use bevy::prelude::*;
use wag_core::{GameState, RoundLog, GENERIC_TEXT_COLOR};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Component)]
pub struct RoundText;

pub fn update_round_text(
    mut query: Query<(&mut Visibility, &mut Text), With<RoundText>>,
    round_log: Res<RoundLog>,
    game_state: Res<State<GameState>>,
) {
    let (mut visible, mut text) = query.single_mut();

    if !game_state.get().show_round_text() {
        *visible = Visibility::Hidden;
        return;
    }

    *visible = Visibility::Inherited;
    if game_state.get() == &GameState::ControllerAssignment {
        text.sections[0].value = "Press start to claim characters (first press = p1)".to_string();
    } else if game_state.get() == &GameState::PreRound {
        text.sections[0].value = "New round".to_string();
    } else if let Some(result) = round_log.last() {
        text.sections[0].value = if let Some(winner) = result.winner {
            format!("{winner} won the round")
        } else {
            "Tie".to_string()
        }
    }
}

pub fn setup_round_info_text(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(40.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Round info text"),
            VisibleInStates(vec![
                GameState::Loading,
                GameState::ControllerAssignment,
                GameState::PreRound,
                GameState::PostRound,
            ]),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Loading...",
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 48.0,
                            color: GENERIC_TEXT_COLOR,
                        },
                    ),
                    ..default()
                },
                RoundText,
            ));
        });
}
