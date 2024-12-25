use crate::{
    assets::Fonts,
    entity_management::VisibleInStates,
    networking,
    ui::{SharedVerticalNav, VerticalMenuNavigation},
};
use bevy::prelude::*;
use foundation::{
    CharacterId, Characters, Controllers, GameState, InputStream, LocalCharacter, LocalController,
    LocalState, MatchState, MenuInput, OnlineState, Player, SoundRequest,
    CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR, GENERIC_TEXT_COLOR, VERTICAL_MENU_OPTION_BACKGROUND,
};
use strum::IntoEnumIterator;

use super::setup_view_title;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CharacterSelectNav(SharedVerticalNav);

pub fn setup_character_select(mut commands: Commands, fonts: Res<Fonts>) {
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
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Percent(1.0)),
                    width: Val::Percent(40.0),
                    ..default()
                },
                BackgroundColor(VERTICAL_MENU_OPTION_BACKGROUND),
                Name::new(character.to_string()),
                character,
            ))
            .with_children(|cb| {
                cb.spawn((
                    Text::new(character.to_string()),
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
                        CharacterHoverIndicator {
                            character,
                            player: Player::One,
                        },
                    ));

                    ccb.spawn((
                        Text::new("P2"),
                        TextFont {
                            font: fonts.basic.clone(),
                            ..default()
                        },
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
    input_stream: ResMut<InputStream>,
    local_controller: Option<Res<LocalController>>,
) {
    for ev in input_stream.menu_events.clone() {
        let (player, is_online) = if let Some(ref lc) = local_controller {
            if lc.0 != ev.player_handle {
                continue;
            }
            // Always player one in online
            (Player::One, true)
        } else {
            // Local play
            (
                controllers
                    .as_ref()
                    .unwrap()
                    .get_player(ev.player_handle)
                    .unwrap(),
                false,
            )
        };

        match ev.event {
            MenuInput::Up => nav.up(player),
            MenuInput::Down => nav.down(player),
            MenuInput::Accept => {
                commands.trigger(SoundRequest::menu_transition());

                if is_online {
                    game_state.set(GameState::Online(OnlineState::Lobby));
                    networking::setup_socket(&mut commands);
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
            }
            MenuInput::Cancel => {
                commands.trigger(SoundRequest::menu_transition());
                if is_online {
                    game_state.set(GameState::MainMenu);
                    return;
                }

                if nav.locked(player) {
                    nav.unlock(player);
                } else {
                    game_state.set(GameState::Local(LocalState::ControllerAssignment));
                }
            }
            _ => {}
        }
    }
}

pub fn update_character_select_visuals(
    mut indicators: Query<(&mut Visibility, &mut TextColor, &CharacterHoverIndicator)>,
    navigator: Res<CharacterSelectNav>,
    options: Query<&CharacterId>,
    local_controller: Option<Res<LocalController>>,
) {
    let [p1_char, p2_char] = options
        .get_many([navigator.p1_select.selected, navigator.p2_select.selected])
        .unwrap();

    for (mut visibility, mut text_color, indicator) in &mut indicators {
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

        text_color.0 = if locked {
            CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR
        } else {
            GENERIC_TEXT_COLOR
        };
    }
}
