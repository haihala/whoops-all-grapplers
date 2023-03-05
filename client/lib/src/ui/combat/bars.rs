use bevy::prelude::*;
use characters::{BarRenderInstructions, Properties, Property};
use wag_core::{Player, RoundLog};

#[derive(Debug, Component, Deref)]
pub struct HealthBar(pub Player);
#[derive(Debug, Component, Deref)]
pub struct ScoreText(pub Player); // TODO: Move this
#[derive(Debug, Component, Deref)]
pub struct MeterBar(pub Player);
#[derive(Debug, Component)]
pub struct SpecialResourceBar(pub Player, pub usize);

pub fn setup_bar(
    root: &mut ChildBuilder,
    instructions: BarRenderInstructions,
    marker: impl Component,
    name: impl Into<std::borrow::Cow<'static, str>>,
) {
    root.spawn((
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
    ));
}

#[allow(clippy::type_complexity)]
pub fn update_bars(
    mut bars: ParamSet<(
        Query<(
            &mut Style,
            &mut BackgroundColor,
            &BarRenderInstructions,
            &HealthBar,
        )>,
        Query<(
            &mut Style,
            &mut BackgroundColor,
            &BarRenderInstructions,
            &MeterBar,
        )>,
        Query<(
            &mut Style,
            &mut BackgroundColor,
            &BarRenderInstructions,
            &SpecialResourceBar,
        )>,
        Query<(&mut Text, &ScoreText)>,
    )>,
    players: Query<(&Player, &Properties)>,
    round_log: Res<RoundLog>,
) {
    for (player, properties) in &players {
        update_bar(&mut bars.p0(), |bar| bar.0 == *player, &properties.health);
        update_bar(&mut bars.p1(), |bar| bar.0 == *player, &properties.meter);

        for property in &properties.special_properties {
            update_bar(&mut bars.p2(), |bar| bar.0 == *player, property);
        }

        for (mut text, score_text) in &mut bars.p3() {
            // TODO This could be moved elsewhere, but that is true for the others as well
            // Don't want to optimize prematurely
            if *player == **score_text {
                text.sections[0].value = round_log.wins(*player).to_string();
            }
        }
    }
}

fn update_bar<T: Component>(
    query: &mut Query<(&mut Style, &mut BackgroundColor, &BarRenderInstructions, &T)>,
    matching_player: impl Fn(&T) -> bool,
    value: &Property,
) {
    for (mut style, mut color, render_instructions, bar) in query {
        if matching_player(bar) {
            style.size.width = Val::Percent(value.get_percentage());

            *color = if value.is_full() {
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
}
