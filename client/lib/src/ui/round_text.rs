use bevy::prelude::*;
use foundation::{InMatch, MatchState, RoundLog, GENERIC_TEXT_COLOR};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Component)]
pub struct RoundText;

pub fn update_round_text(
    mut query: Single<&mut Text, With<RoundText>>,
    round_log: Res<RoundLog>,
    game_state: Option<Res<State<MatchState>>>,
) {
    let text = query.as_mut();

    let Some(gs) = game_state else {
        return;
    };

    if gs.get() == &MatchState::PreRound {
        text.0 = "New round".to_string();
    } else if let Some(result) = round_log.last() {
        text.0 = if let Some(winner) = result.winner {
            format!("{winner} won the round")
        } else {
            "Tie".to_string()
        }
    }
}

pub fn setup_round_info_text(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                top: Val::Percent(40.0),
                left: Val::Px(0.0),
                ..default()
            },
            Name::new("Round info text"),
            VisibleInStates(vec![
                MatchState::Loading,
                MatchState::PreRound,
                MatchState::PostRound,
            ]),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font: fonts.basic.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor::from(GENERIC_TEXT_COLOR),
                StateScoped(InMatch),
                VisibleInStates(vec![
                    MatchState::Loading,
                    MatchState::PreRound,
                    MatchState::PostRound,
                ]),
                RoundText,
            ));
        });
}
