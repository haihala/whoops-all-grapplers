use bevy::prelude::*;
use wag_core::{
    GameState, InMatch, LocalState, MatchState, OnlineState, RoundLog, GENERIC_TEXT_COLOR,
};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Component)]
pub struct RoundText;

pub fn update_round_text(
    mut query: Query<&mut Text, With<RoundText>>,
    round_log: Res<RoundLog>,
    game_state: Option<Res<State<MatchState>>>,
) {
    let Ok(ref mut text) = query.get_single_mut() else {
        return;
    };

    let Some(gs) = game_state else {
        return;
    };

    if gs.get() == &MatchState::PreRound {
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
                GameState::Local(LocalState::Loading),
                GameState::Local(LocalState::Match(MatchState::PreRound)),
                GameState::Local(LocalState::Match(MatchState::PostRound)),
                GameState::Online(OnlineState::Loading),
                GameState::Online(OnlineState::Match(MatchState::PreRound)),
                GameState::Online(OnlineState::Match(MatchState::PostRound)),
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
                StateScoped(InMatch),
                VisibleInStates(vec![
                    GameState::Local(LocalState::Loading),
                    GameState::Local(LocalState::Match(MatchState::PreRound)),
                    GameState::Local(LocalState::Match(MatchState::PostRound)),
                    GameState::Online(OnlineState::Loading),
                    GameState::Online(OnlineState::Match(MatchState::PreRound)),
                    GameState::Online(OnlineState::Match(MatchState::PostRound)),
                ]),
                RoundText,
            ));
        });
}
