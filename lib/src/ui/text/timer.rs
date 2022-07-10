use bevy::prelude::*;
use time::{RoundTimer, ROUND_TIME};

use crate::ui::utils::{div, div_style, FULL};

pub const TIMER_WIDTH: f32 = 10.0;
const TIMER_TOP_PADDING: f32 = 2.0;

pub fn spawn_timer(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(TIMER_WIDTH), FULL),
                position: Rect {
                    top: Val::Percent(TIMER_TOP_PADDING),
                    ..default()
                },
                ..div_style()
            },
            ..div()
        })
        .with_children(|timer_wrapper| {
            timer_wrapper
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        ROUND_TIME.round().to_string(),
                        TextStyle {
                            font,
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..default()
                })
                .insert(RoundTimer);
        });
}
