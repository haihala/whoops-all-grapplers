use bevy::prelude::*;

mod gauges;
pub use gauges::{update_bars, update_counters, ResourceCounter, ResourceGauge};

mod notifications;
pub use notifications::{update_combo_counters, update_notifications, Notifications};

mod round_timer;
pub use round_timer::update_timer;

use characters::{GaugeType, Gauges, RenderInstructions, ResourceBarVisual};
use foundation::{InMatch, MatchState, Player, Players, RoundLog, GENERIC_TEXT_COLOR};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Debug, Component, Deref)]
pub struct ScoreText(pub Player);

#[derive(Debug, Component)]
pub struct CombatUI;

pub fn setup_combat_hud(
    mut commands: Commands,
    fonts: Res<Fonts>,
    properties: Query<&Gauges>,
    players: Res<Players>,
    existing_huds: Query<Entity, With<CombatUI>>,
) {
    for entity in &existing_huds {
        commands.entity(entity).despawn_recursive();
    }

    let container = commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Visibility::Hidden,
            VisibleInStates(vec![MatchState::Combat, MatchState::PostRound]),
            StateScoped(InMatch),
            Name::new("Combat UI container"),
            CombatUI,
        ))
        .id();

    let timer_width = 15.0;
    let side_width = (100.0 - timer_width) / 2.0;

    setup_player_hud(
        &mut commands,
        container,
        side_width,
        &fonts,
        Player::One,
        properties.get(players.one).unwrap(),
    );
    round_timer::setup_timer(&mut commands, container, fonts.basic.clone(), timer_width);
    setup_player_hud(
        &mut commands,
        container,
        side_width,
        &fonts,
        Player::Two,
        properties.get(players.two).unwrap(),
    );
}

fn setup_player_hud(
    commands: &mut Commands,
    parent: Entity,
    width_percentage: f32,
    fonts: &Fonts,
    player: Player,
    properties: &Gauges,
) {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(3.0)),
                width: Val::Percent(width_percentage),
                height: Val::Percent(100.0),
                ..default()
            },
            Name::new("Player HUD"),
        ))
        .set_parent(parent)
        .id();

    setup_top_hud(commands, container, fonts, player);
    notifications::setup_toasts(commands, container, player);
    notifications::setup_combo_counter(commands, container, player, fonts);
    setup_bottom_hud(commands, fonts, container, player, properties);
}

fn setup_top_hud(commands: &mut Commands, parent: Entity, fonts: &Fonts, player: Player) {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: match player {
                    // Align towards the center
                    Player::One => AlignItems::FlexEnd,
                    Player::Two => AlignItems::FlexStart,
                },
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                ..default()
            },
            Name::new(format!("Player {player} health bar wrapper")),
        ))
        .set_parent(parent)
        .id();

    gauges::setup_bar(
        commands,
        player,
        container,
        ResourceBarVisual::default_health(),
        ResourceGauge(player, GaugeType::Health),
        "Health bar",
    );
    setup_round_counter(commands, container, fonts, player);
}

fn setup_round_counter(commands: &mut Commands, parent: Entity, fonts: &Fonts, player: Player) {
    commands
        .spawn((
            Text::new("0"),
            TextFont {
                font: fonts.basic.clone(),
                font_size: 40.0,
                ..default()
            },
            TextColor(GENERIC_TEXT_COLOR),
            ScoreText(player),
            Name::new("Round counter"),
        ))
        .set_parent(parent);
}

fn setup_bottom_hud(
    commands: &mut Commands,
    fonts: &Fonts,
    parent: Entity,
    player: Player,
    properties: &Gauges,
) {
    let container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: match player {
                // Align towards the side of the screen
                Player::One => AlignItems::FlexStart,
                Player::Two => AlignItems::FlexEnd,
            },
            width: Val::Percent(100.0),
            height: Val::Percent(40.0),
            margin: UiRect {
                bottom: Val::Percent(gauges::SCREEN_EDGE_PADDING),
                ..default()
            },
            ..default()
        })
        .set_parent(parent)
        .id();

    for (prop_type, property) in properties.iter() {
        if matches!(prop_type, GaugeType::Health | GaugeType::Meter) {
            continue;
        }

        match property.render_instructions {
            RenderInstructions::Bar(bar) => {
                gauges::setup_bar(
                    commands,
                    player,
                    container,
                    bar,
                    ResourceGauge(player, *prop_type),
                    format!("Special resource bar {:?}", prop_type),
                );
            }
            RenderInstructions::Counter(counter) => {
                gauges::setup_counter(
                    commands,
                    player,
                    container,
                    fonts.basic.clone(),
                    counter,
                    ResourceCounter(player, *prop_type),
                    format!("Special resource counter {:?}", prop_type),
                );
            }
            RenderInstructions::None => {}
        }
    }

    gauges::setup_bar(
        commands,
        player,
        container,
        ResourceBarVisual::default_meter(),
        ResourceGauge(player, GaugeType::Meter),
        "Meter bar",
    );
}

pub fn update_score(
    mut score_texts: Query<(&mut Text, &ScoreText)>,
    players: Query<&Player>,
    round_log: Res<RoundLog>,
) {
    for player in &players {
        for (mut text, score_text) in &mut score_texts {
            if *player == **score_text {
                text.0 = round_log.wins(*player).to_string();
            }
        }
    }
}
