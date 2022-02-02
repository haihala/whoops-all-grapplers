use bevy::prelude::*;
use time::RoundResult;

#[derive(Component)]
pub struct RoundText;

pub fn round_start(mut query: Query<&mut Text, With<RoundText>>) {
    query.single_mut().sections[0].value = "".to_string();
}

pub fn round_over(mut query: Query<&mut Text, With<RoundText>>, round_result: Res<RoundResult>) {
    query.single_mut().sections[0].value = if let Some(winner) = round_result.winner {
        format!("{} won the round", winner)
    } else {
        "Tie".to_string()
    }
}
