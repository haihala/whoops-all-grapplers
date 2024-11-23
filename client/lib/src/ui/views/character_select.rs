use crate::{
    assets::Fonts,
    entity_management::VisibleInStates,
    ui::{SharedVerticalNav, VerticalMenuNavigation},
};
use bevy::prelude::*;
use strum::IntoEnumIterator;
use wag_core::{
    CharacterId, Characters, Controllers, GameButton, GameState, InputEvent, LocalCharacter,
    LocalController, LocalState, MatchState, OnlineState, Player, StickPosition,
    CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR, GENERIC_TEXT_COLOR, VERTICAL_MENU_OPTION_BACKGROUND,
};

use super::{setup_view_title, MenuInputs};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CharacterSelectNav(SharedVerticalNav);

pub fn setup_character_select(mut commands: Commands, fonts: Res<Fonts>) {
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
            VisibleInStates(vec![
                GameState::Local(LocalState::CharacterSelect),
                GameState::Online(OnlineState::CharacterSelect),
            ]),
            Name::new("Character select UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, "Choose your character");
            let options = setup_character_options(cb, &fonts);
            navigation = Some(VerticalMenuNavigation::from_buttons(options));
        });

    if let Some(nav) = navigation {
        commands.insert_resource(CharacterSelectNav(SharedVerticalNav {
            p1_select: nav.clone(),
            p2_select: nav,
            p1_locked: false,
            p2_locked: false,
        }));
    }
}

#[derive(Debug, Component)]
pub struct CharacterHoverIndicator {
    character: CharacterId,
    player: Player,
}

fn setup_character_options(root: &mut ChildBuilder, fonts: &Fonts) -> Vec<Entity> {
    CharacterId::iter()
        .map(|character| {
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
                Name::new(character.to_string()),
                character,
            ))
            .with_children(|cb| {
                cb.spawn(TextBundle::from_section(
                    character.to_string(),
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
                        CharacterHoverIndicator {
                            character,
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
                        CharacterHoverIndicator {
                            character,
                            player: Player::Two,
                        },
                    ));
                });
            })
            .id()
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub fn navigate_character_select(
    mut commands: Commands,
    mut nav: ResMut<CharacterSelectNav>,
    controllers: Option<Res<Controllers>>,
    options: Query<&CharacterId>,
    mut game_state: ResMut<NextState<GameState>>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut events: ResMut<MenuInputs>,
    local_controller: Option<Res<LocalController>>,
) {
    while let Some(ev) = events.pop_front() {
        let player = if let Some(ref lc) = local_controller {
            if lc.0 != ev.player_handle {
                continue;
            }
            // Always player one in online
            Player::One
        } else {
            // Local play
            controllers
                .as_ref()
                .unwrap()
                .get_player(ev.player_handle)
                .unwrap()
        };

        match ev.event {
            InputEvent::Point(point) => {
                if point == StickPosition::N {
                    nav.up(player);
                } else if point == StickPosition::S {
                    nav.down(player);
                }
            }
            InputEvent::Press(button) => {
                if button == GameButton::Fast {
                    if local_controller.is_some() {
                        game_state.set(GameState::Online(OnlineState::Lobby));
                        commands.insert_resource(LocalCharacter(
                            *options.get(nav.p1_select.selected).unwrap(),
                        ));
                        return;
                    }

                    nav.lock_in(player);
                    if nav.both_locked() {
                        let [p1_char, p2_char] = options
                            .get_many([nav.p1_select.selected, nav.p2_select.selected])
                            .unwrap();
                        commands.insert_resource(Characters {
                            p1: *p1_char,
                            p2: *p2_char,
                        });
                        game_state.set(GameState::Local(LocalState::Match));
                        match_state.set(MatchState::Loading);
                    }
                } else if button == GameButton::Strong {
                    if nav.locked(player) {
                        nav.unlock(player);
                    } else {
                        game_state.set(GameState::Local(LocalState::ControllerAssignment));
                    }
                }
            }
            InputEvent::Release(_) => {}
        }
    }
}

pub fn update_character_select_visuals(
    mut indicators: Query<(&mut Visibility, &mut Text, &CharacterHoverIndicator)>,
    navigator: Res<CharacterSelectNav>,
    options: Query<&CharacterId>,
    local_controller: Option<Res<LocalController>>,
) {
    let [p1_char, p2_char] = options
        .get_many([navigator.p1_select.selected, navigator.p2_select.selected])
        .unwrap();

    for (mut visibility, mut text, indicator) in &mut indicators {
        let (locked, character) = match indicator.player {
            Player::One => (navigator.p1_locked, p1_char),
            Player::Two => (navigator.p2_locked, p2_char),
        };

        *visibility = if indicator.character == *character
            // This is to hide other character selector in online
            && (local_controller.is_none() || indicator.player == Player::One)
        {
            Visibility::Inherited // Visible, but only if parent is
        } else {
            Visibility::Hidden
        };

        text.sections[0].style.color = if locked {
            CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR
        } else {
            GENERIC_TEXT_COLOR
        };
    }
}
