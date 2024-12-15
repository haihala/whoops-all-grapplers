use bevy::prelude::*;
use characters::{CounterVisual, GaugeType, Gauges, ResourceBarVisual};
use foundation::{Player, RESOURCE_COUNTER_TEXT_COLOR, TRANSPARENT};

#[derive(Debug, Component)]
pub struct ResourceGauge(pub Player, pub GaugeType);
#[derive(Debug, Component)]
pub struct ResourceCounter(pub Player, pub GaugeType);

pub const SCREEN_EDGE_PADDING: f32 = 5.0;

pub fn setup_bar(
    commands: &mut Commands,
    player: Player,
    parent: Entity,
    instructions: ResourceBarVisual,
    marker: impl Component,
    name: impl Into<std::borrow::Cow<'static, str>>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0 - SCREEN_EDGE_PADDING),
                height: Val::Percent(instructions.height),
                column_gap: Val::Percent(instructions.segment_gap),
                flex_direction: match player {
                    Player::One => FlexDirection::RowReverse,
                    Player::Two => FlexDirection::Row,
                },
                margin: UiRect {
                    bottom: Val::Percent(1.0),
                    left: Val::Percent(if player == Player::One {
                        SCREEN_EDGE_PADDING
                    } else {
                        0.0
                    }),
                    right: Val::Percent(if player == Player::Two {
                        SCREEN_EDGE_PADDING
                    } else {
                        0.0
                    }),
                    ..default()
                },
                ..default()
            },
            instructions,
            marker,
            Name::new(name),
        ))
        .set_parent(parent)
        .with_children(|root_bar| {
            for i in 0..instructions.segments {
                root_bar.spawn((
                    Node {
                        width: Val::Percent(instructions.segment_width()),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(instructions.default_color),
                    Name::new(format!("Segment {}", i)),
                ));
            }
        });
}

pub fn setup_counter(
    commands: &mut Commands,
    player: Player,
    parent: Entity,
    font: Handle<Font>,
    instructions: CounterVisual,
    marker: impl Component,
    name: impl Into<std::borrow::Cow<'static, str>>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                column_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Row,
                justify_content: match player {
                    // Align towards the center
                    Player::One => JustifyContent::FlexEnd,
                    Player::Two => JustifyContent::FlexStart,
                },
                margin: UiRect {
                    bottom: Val::Percent(1.0),
                    ..default()
                },
                ..default()
            },
            instructions,
            Name::new(name),
        ))
        .set_parent(parent)
        .with_children(|root_bar| {
            let text_style = (
                TextFont {
                    font: font.clone(),
                    font_size: 36.0,
                    ..default()
                },
                TextColor(RESOURCE_COUNTER_TEXT_COLOR),
            );

            let spawn_label = |root: &mut ChildBuilder| {
                root.spawn((
                    Text::new(instructions.label),
                    text_style.clone(),
                    Name::new("Label"),
                ));
            };

            let spawn_counter = |root: &mut ChildBuilder| {
                root.spawn((
                    Text::new("0"),
                    text_style.clone(),
                    marker,
                    Name::new("Value"),
                ));
            };

            match player {
                Player::One => {
                    spawn_label(root_bar);
                    spawn_counter(root_bar);
                }
                Player::Two => {
                    spawn_counter(root_bar);
                    spawn_label(root_bar);
                }
            }
        });
}

pub fn update_bars(
    mut segments: Query<(&mut Node, &mut BackgroundColor)>,
    bars: Query<(&Children, &ResourceBarVisual, &ResourceGauge)>,
    players: Query<(&Player, &Gauges)>,
) {
    for (player, properties) in &players {
        for (key, property) in properties.iter() {
            for (children, bar_visual, bar) in &bars {
                if bar.0 != *player || bar.1 != *key {
                    continue;
                }

                let mut percentage = property.get_percentage();
                let per_segment = 100.0 / bar_visual.segments as f32;

                for child in children {
                    let (mut style, mut color) = segments.get_mut(*child).unwrap();

                    (*color, style.width) = if percentage >= per_segment {
                        percentage -= per_segment;
                        (
                            bar_visual
                                .full_color
                                .unwrap_or(bar_visual.default_color)
                                .into(),
                            Val::Percent(bar_visual.segment_width()),
                        )
                    } else if percentage > 0.0 {
                        let width =
                            Val::Percent(bar_visual.segment_width() * percentage / per_segment);
                        percentage = 0.0;
                        (bar_visual.default_color.into(), width)
                    } else {
                        (TRANSPARENT.into(), Val::Percent(0.0))
                    };
                }
            }
        }
    }
}

pub fn update_counters(
    mut counters: Query<(&mut Text, &ResourceCounter)>,
    players: Query<(&Player, &Gauges)>,
) {
    for (player, properties) in &players {
        for (key, property) in properties.iter() {
            for (mut text, counter) in &mut counters {
                if counter.0 != *player || counter.1 != *key {
                    continue;
                }

                text.0 = property.current.to_string();
            }
        }
    }
}
