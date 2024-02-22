use bevy::prelude::*;

use wag_core::{Clock, COMBAT_DURATION, ROUND_TIMER_TEXT_COLOR};

#[derive(Debug, Component)]
pub struct RoundTimer;

pub fn update_timer(mut query: Query<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.single_mut().sections[0].value = clock.timer_value().to_string();
}

pub fn setup_timer(
    commands: &mut Commands,
    parent: Entity,
    font: Handle<Font>,
    width_percentage: f32,
) {
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    width: Val::Percent(width_percentage),
                    height: Val::Percent(100.0),
                    top: Val::Percent(2.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Timer"),
        ))
        .set_parent(parent)
        .id();

    commands
        .spawn((
            TextBundle {
                text: Text::from_section(
                    COMBAT_DURATION.round().to_string(),
                    TextStyle {
                        font,
                        font_size: 100.0,
                        color: ROUND_TIMER_TEXT_COLOR,
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            RoundTimer,
        ))
        .set_parent(container);
}
