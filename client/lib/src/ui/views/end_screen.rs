use crate::{
    assets::Fonts,
    state_transitions::TransitionTimer,
    ui::{SharedVerticalNav, VerticalMenuNavigation},
};
use bevy::prelude::*;
use foundation::{
    Controllers, GameButton, GameResult, GameState, InputEvent, MatchState, Player, StickPosition,
    CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR, GENERIC_TEXT_COLOR, VERTICAL_MENU_OPTION_BACKGROUND,
};

use super::{setup_view_subtitle, setup_view_title, MenuInputs};

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

pub fn setup_end_screen(mut commands: Commands, fonts: Res<Fonts>, game_result: Res<GameResult>) {
    let mut navigation = None;

    commands
        .spawn((
            NodeBundle {
                style: Style {
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
                ..default()
            },
            StateScoped(MatchState::EndScreen),
            Name::new("End screen UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, format!("Player {} wins!", game_result.winner));
            setup_view_subtitle(cb, &fonts, "go next?");
            let options = setup_end_screen_options(cb, &fonts);
            navigation = Some(VerticalMenuNavigation::from_buttons(options));
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

fn setup_end_screen_options(root: &mut ChildBuilder, fonts: &Fonts) -> Vec<Entity> {
    vec![
        setup_end_screen_option(root, fonts, EndScreenOption::Rematch),
        setup_end_screen_option(root, fonts, EndScreenOption::QuitToMainMenu),
        setup_end_screen_option(root, fonts, EndScreenOption::QuitToDesktop),
    ]
}

fn setup_end_screen_option(
    root: &mut ChildBuilder,
    fonts: &Fonts,
    option: EndScreenOption,
) -> Entity {
    root.spawn((
        NodeBundle {
            background_color: VERTICAL_MENU_OPTION_BACKGROUND.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Percent(1.0)),
                width: Val::Percent(40.0),
                ..default()
            },
            ..default()
        },
        Name::new(option.to_string()),
        option,
    ))
    .with_children(|cb| {
        cb.spawn(TextBundle::from_section(
            option.to_string(),
            TextStyle {
                font: fonts.basic.clone(),
                font_size: 30.0,
                ..default()
            },
        ));

        cb.spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|ccb| {
            ccb.spawn((
                TextBundle::from_section(
                    "P1",
                    TextStyle {
                        font: fonts.basic.clone(),
                        ..default()
                    },
                ),
                OptionHoverIndicator {
                    player: Player::One,
                },
            ));

            ccb.spawn((
                TextBundle::from_section(
                    "P2",
                    TextStyle {
                        font: fonts.basic.clone(),
                        ..default()
                    },
                ),
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
    mut events: ResMut<MenuInputs>,
    controllers: Res<Controllers>,
    options: Query<&EndScreenOption>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    mut quitter: EventWriter<AppExit>,
) {
    while let Some(ev) = events.pop_front() {
        let Some(player) = controllers.get_player(ev.player_handle) else {
            continue;
        };

        match ev.event {
            InputEvent::Point(StickPosition::N) => nav.up(player),
            InputEvent::Point(StickPosition::S) => nav.down(player),
            InputEvent::Press(GameButton::Fast) => {
                let selected = nav.selected(player);
                let option_type = options.get(selected).unwrap();

                match option_type {
                    EndScreenOption::Rematch => {
                        nav.lock_in(player);
                        if nav.both_locked() {
                            next_match_state.set(MatchState::None); // This will despawn shit
                            commands.insert_resource(TransitionTimer {
                                timer: Timer::from_seconds(0.0, TimerMode::Once),
                                state: MatchState::Loading,
                            });
                        }
                    }
                    EndScreenOption::QuitToMainMenu => {
                        next_game_state.set(GameState::MainMenu);
                        next_match_state.set(MatchState::None);
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
    mut indicators: Query<(&mut Visibility, &mut Text, &OptionHoverIndicator, Entity)>,
    hierarchy: Query<&Parent>,
    navigator: Res<EndScreenNav>,
) {
    for (mut visibility, mut text, indicator, entity) in &mut indicators {
        let middle = hierarchy.get(entity).unwrap();
        let option = **hierarchy.get(**middle).unwrap();

        let (locked, selected) = match indicator.player {
            Player::One => (navigator.p1_locked, navigator.p1_select.selected),
            Player::Two => (navigator.p2_locked, navigator.p2_select.selected),
        };

        text.sections[0].style.color = if locked {
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
}
