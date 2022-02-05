use bevy::prelude::*;
use time::{Clock, GameState, RoundResult, RoundTimer, ROUND_TIME};

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
