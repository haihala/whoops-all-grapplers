use bevy::prelude::*;

use crate::game_flow::RoundResult;

pub struct RoundText;

pub fn round_start(mut query: Query<&mut Text, With<RoundText>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = "".to_string();
}

pub fn round_over(mut query: Query<&mut Text, With<RoundText>>, round_result: Res<RoundResult>) {
    let mut text = query.single_mut().unwrap();
    if let Some(winner) = round_result.winner {
        text.sections[0].value = format!("{} won the round", winner);
    } else {
        text.sections[0].value = "Tie".to_string();
    }
}
