use bevy::prelude::*;
use characters::{BarRenderInstructions, Properties, PropertyType};
use wag_core::{Player, RoundLog};

#[derive(Debug, Component, Deref)]
pub struct ScoreText(pub Player); // TODO: Move this

#[derive(Debug, Component)]
pub struct PropertyBar(pub Player, pub PropertyType);

pub fn setup_bar(
    commands: &mut Commands,
    parent: Entity,
    instructions: BarRenderInstructions,
    marker: impl Component,
    name: impl Into<std::borrow::Cow<'static, str>>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(instructions.height)),
                    margin: UiRect {
                        bottom: Val::Percent(1.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: instructions.default_color.into(),
                ..default()
            },
            instructions,
            marker,
            Name::new(name),
        ))
        .set_parent(parent);
}

#[allow(clippy::type_complexity)]
pub fn update_bars(
    mut bars: ParamSet<(
        Query<(
            &mut Style,
            &mut BackgroundColor,
            &BarRenderInstructions,
            &PropertyBar,
        )>,
        Query<(&mut Text, &ScoreText)>,
    )>,
    players: Query<(&Player, &Properties)>,
    round_log: Res<RoundLog>,
) {
    for (player, properties) in &players {
        for (key, property) in properties.iter() {
            for (mut style, mut color, render_instructions, bar) in &mut bars.p0() {
                if bar.0 != *player || bar.1 != *key {
                    continue;
                }

                style.size.width = Val::Percent(property.get_percentage());

                *color = if property.is_full() {
                    if let Some(full_color) = render_instructions.full_color {
                        full_color
                    } else {
                        render_instructions.default_color
                    }
                } else {
                    render_instructions.default_color
                }
                .into();
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
