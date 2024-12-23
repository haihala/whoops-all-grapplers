use crate::{
    assets::Fonts,
    entity_management::VisibleInStates,
    networking,
    state_transitions::TransitionTimer,
    ui::{SharedVerticalNav, VerticalMenuNavigation},
};
use bevy::prelude::*;
use foundation::{
    Clock, Controllers, GameButton, GameResult, GameState, InputEvent, InputStream, MatchState,
    Player, RoundLog, SoundRequest, StickPosition, CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR,
    GENERIC_TEXT_COLOR, VERTICAL_MENU_OPTION_BACKGROUND,
};

use super::{setup_view_subtitle, setup_view_title};

#[derive(Debug, Component)]
pub struct MatchResultTextMarker;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct EndScreenNav(SharedVerticalNav);

#[derive(Debug, Component, Clone, Copy)]
pub enum EndScreenOption {
    Rematch,
    QuitToMainMenu,
    QuitToDesktop,
}
impl std::fmt::Display for EndScreenOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EndScreenOption::Rematch => "Rematch",
                EndScreenOption::QuitToMainMenu => "Quit to main menu",
                EndScreenOption::QuitToDesktop => "Quit to desktop",
            }
        )
    }
}

pub fn setup_end_screen(mut commands: Commands, fonts: Res<Fonts>) {
    let mut navigation = None;

    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(5.0),
                padding: UiRect::all(Val::Percent(20.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            VisibleInStates(vec![MatchState::EndScreen]),
            Name::new("End screen UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, "").insert(MatchResultTextMarker);
            setup_view_subtitle(cb, &fonts, "go next?");

            navigation = Some(VerticalMenuNavigation::from_buttons(
                vec![
                    EndScreenOption::Rematch,
                    EndScreenOption::QuitToMainMenu,
                    EndScreenOption::QuitToDesktop,
                ]
                .into_iter()
                .map(|opt| setup_end_screen_option(cb, &fonts, opt))
                .collect::<Vec<_>>(),
            ));
        });

    if let Some(nav) = navigation {
        commands.insert_resource(EndScreenNav(SharedVerticalNav {
            p1_select: nav.clone(),
            p2_select: nav,
            p1_locked: false,
            p2_locked: false,
        }));
    }
}

#[derive(Debug, Component)]
pub struct OptionHoverIndicator {
    player: Player,
}

fn setup_end_screen_option(
    root: &mut ChildBuilder,
    fonts: &Fonts,
    option: EndScreenOption,
) -> Entity {
    root.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.0)),
            width: Val::Percent(40.0),
            ..default()
        },
        BackgroundColor(VERTICAL_MENU_OPTION_BACKGROUND),
        Name::new(option.to_string()),
        option,
    ))
    .with_children(|cb| {
        cb.spawn((
            Text::new(option.to_string()),
            TextFont {
                font: fonts.basic.clone(),
                font_size: 30.0,
                ..default()
            },
        ));

        cb.spawn(Node {
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|ccb| {
            ccb.spawn((
                Text::new("P1"),
                TextFont {
                    font: fonts.basic.clone(),
                    ..default()
                },
                OptionHoverIndicator {
                    player: Player::One,
                },
            ));

            ccb.spawn((
                Text::new("P2"),
                TextFont {
                    font: fonts.basic.clone(),
                    ..default()
                },
                OptionHoverIndicator {
                    player: Player::Two,
                },
            ));
        });
    })
    .id()
}

#[allow(clippy::too_many_arguments)]
pub fn navigate_end_screen(
    mut commands: Commands,
    mut nav: ResMut<EndScreenNav>,
    input_stream: ResMut<InputStream>,
    controllers: Res<Controllers>,
    options: Query<&EndScreenOption>,
    game_state: ResMut<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    mut quitter: EventWriter<AppExit>,
    mut log: ResMut<RoundLog>,
    clock: Res<Clock>,
) {
    for ev in input_stream.events.clone() {
        let Some(player) = controllers.get_player(ev.player_handle) else {
            continue;
        };

        match ev.event {
            InputEvent::Point(StickPosition::N) => nav.up(player),
            InputEvent::Point(StickPosition::S) => nav.down(player),
            InputEvent::Press(GameButton::Fast) => {
                commands.trigger(SoundRequest::menu_transition());
                let selected = nav.selected(player);
                let option_type = options.get(selected).unwrap();

                match option_type {
                    EndScreenOption::Rematch => {
                        nav.lock_in(player);
                        if nav.both_locked() {
                            // Despawn state scoped entities
                            next_match_state.set(MatchState::None);
                            commands.insert_resource(TransitionTimer {
                                frame: clock.frame + 1,
                                state: MatchState::Loading,
                            });
                            log.clear();
                        }
                    }
                    EndScreenOption::QuitToMainMenu => {
                        next_game_state.set(GameState::MainMenu);
                        next_match_state.set(MatchState::None);
                        log.clear();

                        if game_state.get().is_online() {
                            networking::network_teardown(&mut commands);
                        }
                    }
                    EndScreenOption::QuitToDesktop => {
                        quitter.send_default();
                    }
                }
            }
            InputEvent::Press(GameButton::Strong) => {
                nav.unlock(player);
            }
            _ => {}
        }
    }
}

pub fn update_end_screen_visuals(
    mut indicators: Query<(
        &mut Visibility,
        &mut TextColor,
        &OptionHoverIndicator,
        Entity,
    )>,
    result_text: Single<&mut Text, With<MatchResultTextMarker>>,
    result: Res<GameResult>,
    hierarchy: Query<&Parent>,
    navigator: Res<EndScreenNav>,
) {
    for (mut visibility, mut text_color, indicator, entity) in &mut indicators {
        let middle = hierarchy.get(entity).unwrap();
        let option = **hierarchy.get(**middle).unwrap();

        let (locked, selected) = match indicator.player {
            Player::One => (navigator.p1_locked, navigator.p1_select.selected),
            Player::Two => (navigator.p2_locked, navigator.p2_select.selected),
        };

        text_color.0 = if locked {
            CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR
        } else {
            GENERIC_TEXT_COLOR
        };

        *visibility = if selected == option {
            Visibility::Inherited // Visible, but only if parent is
        } else {
            Visibility::Hidden
        };
    }

    result_text.into_inner().0 = format!("Player {} wins!", result.winner);
}
