use bevy::prelude::*;
use characters::{ResourceBarVisual, ResourceType, WAGResources};
use wag_core::{Player, RoundLog};

#[derive(Debug, Component, Deref)]
pub struct ScoreText(pub Player); // TODO: Move this

#[derive(Debug, Component)]
pub struct PropertyBar(pub Player, pub ResourceType);

pub fn setup_bar(
    commands: &mut Commands,
    parent: Entity,
    instructions: ResourceBarVisual,
    marker: impl Component,
    name: impl Into<std::borrow::Cow<'static, str>>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(instructions.height),
                    margin: UiRect {
                        bottom: Val::Percent(1.0),
                        ..default()
                    },
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
            for _ in 0..instructions.segments {
                root_bar.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0 / instructions.segments as f32),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: instructions.default_color.into(),
                    ..default()
                });
            }
        });
}

#[allow(clippy::type_complexity)]
pub fn update_bars(
    mut bars: ParamSet<(
        Query<(&Children, &ResourceBarVisual, &PropertyBar)>,
        Query<(&mut Text, &ScoreText)>,
    )>,
    mut segments: Query<(&mut Style, &mut BackgroundColor)>,
    players: Query<(&Player, &WAGResources)>,
    round_log: Res<RoundLog>,
) {
    for (player, properties) in &players {
        for (key, property) in properties.iter() {
            for (children, bar_visual, bar) in &bars.p0() {
                if bar.0 != *player || bar.1 != *key {
                    continue;
                }

                let mut percentage = property.get_percentage();
                let per_segment = 100.0 / bar_visual.segments as f32;

                for child in children {
                    let (mut style, mut color) = segments.get_mut(*child).unwrap();
                    style.width = Val::Percent(percentage);

                    *color = if percentage >= per_segment {
                        percentage -= per_segment;
                        bar_visual.full_color.unwrap_or(bar_visual.default_color)
                    } else if percentage > 0.0 {
                        percentage = 0.0;
                        bar_visual.default_color
                    } else {
                        Color::NONE
                    }
                    .into();
                }
            }
        }

        for (mut text, score_text) in &mut bars.p1() {
            // TODO This could be moved elsewhere, but that is true for the others as well
            // Don't want to optimize prematurely
            if *player == **score_text {
                text.sections[0].value = round_log.wins(*player).to_string();
            }
        }
    }
}
