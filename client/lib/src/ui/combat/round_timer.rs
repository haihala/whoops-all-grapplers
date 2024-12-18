use bevy::prelude::*;

use foundation::{Clock, COMBAT_DURATION, ROUND_TIMER_TEXT_COLOR};

#[derive(Debug, Component)]
pub struct RoundTimer;

pub fn update_timer(query: Single<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.into_inner().0 = clock.timer_value.to_string();
}

pub fn setup_timer(
    commands: &mut Commands,
    parent: Entity,
    font: Handle<Font>,
    width_percentage: f32,
) {
    let container = commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                width: Val::Percent(width_percentage),
                height: Val::Percent(100.0),
                top: Val::Percent(2.0),
                ..default()
            },
            Name::new("Timer"),
        ))
        .set_parent(parent)
        .id();

    commands
        .spawn((
            Text::new(COMBAT_DURATION.round().to_string()),
            TextFont {
                font,
                font_size: 100.0,
                ..default()
            },
            TextColor(ROUND_TIMER_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            RoundTimer,
        ))
        .set_parent(container);
}
