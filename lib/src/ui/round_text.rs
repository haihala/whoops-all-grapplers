use bevy::prelude::*;

use crate::{
    assets::{Colors, Fonts},
    game_flow::RoundResult,
};

pub struct RoundText;

pub fn setup(mut commands: Commands, fonts: Res<Fonts>, colors: Res<Colors>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(40.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.transparent.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "New round",
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(RoundText);
        });
}

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
