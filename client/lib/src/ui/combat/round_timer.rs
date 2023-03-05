use bevy::prelude::*;

use wag_core::{Clock, COMBAT_DURATION};

#[derive(Debug, Component)]
pub struct RoundTimer;

pub fn update_timer(mut query: Query<&mut Text, With<RoundTimer>>, clock: Res<Clock>) {
    query.single_mut().sections[0].value =
        (COMBAT_DURATION - clock.elapsed_time).floor().to_string();
}

pub fn setup_timer(parent: &mut ChildBuilder, font: Handle<Font>, width_percentage: f32) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    size: Size::new(Val::Percent(width_percentage), Val::Percent(100.0)),
                    position: UiRect {
                        top: Val::Percent(2.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Name::new("Timer"),
        ))
        .with_children(|timer_wrapper| {
            timer_wrapper.spawn((
                TextBundle {
                    text: Text::from_section(
                        COMBAT_DURATION.round().to_string(),
                        TextStyle {
                            font,
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }),
                    ..default()
                },
                RoundTimer,
            ));
        });
}
