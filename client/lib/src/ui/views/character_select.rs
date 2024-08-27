use crate::{
    assets::Fonts,
    entity_management::VisibleInStates,
    ui::{SharedVerticalNav, VerticalMenuNavigation},
};
use bevy::{input::gamepad::GamepadEvent, prelude::*};
use strum::IntoEnumIterator;
use wag_core::{
    CharacterId, Characters, Controllers, GameState, Player, CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR,
    GENERIC_TEXT_COLOR, VERTICAL_MENU_OPTION_BACKGROUND,
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
            VisibleInStates(vec![GameState::CharacterSelect]),
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

pub fn navigate_character_select(
    mut commands: Commands,
    mut nav: ResMut<CharacterSelectNav>,
    controllers: Res<Controllers>,
    options: Query<&CharacterId>,
    mut state: ResMut<NextState<GameState>>,
    mut events: ResMut<MenuInputs>,
) {
    // TODO: Analog stick
    while let Some(ev) = events.pop_front() {
        match ev {
            GamepadEvent::Button(ev_btn) if ev_btn.value == 1.0 => {
                let Some(player) = controllers.get_player(ev_btn.gamepad) else {
                    continue;
                };

                match ev_btn.button_type {
                    GamepadButtonType::DPadUp => nav.up(player),
                    GamepadButtonType::DPadDown => nav.down(player),
                    GamepadButtonType::East => {
                        if nav.locked(player) {
                            nav.unlock(player);
                        } else {
                            state.set(GameState::ControllerAssignment);
                        }
                    }
                    GamepadButtonType::South => {
                        nav.lock_in(player);
                        if nav.both_locked() {
                            let [p1_char, p2_char] = options
                                .get_many([nav.p1_select.selected, nav.p2_select.selected])
                                .unwrap();
                            commands.insert_resource(Characters {
                                p1: *p1_char,
                                p2: *p2_char,
                            });
                            state.set(GameState::Loading);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn update_character_select_visuals(
    mut indicators: Query<(&mut Visibility, &mut Text, &CharacterHoverIndicator)>,
    navigator: Res<CharacterSelectNav>,
    options: Query<&CharacterId>,
) {
    let [p1_char, p2_char] = options
        .get_many([navigator.p1_select.selected, navigator.p2_select.selected])
        .unwrap();

    for (mut visibility, mut text, indicator) in &mut indicators {
        let (locked, character) = match indicator.player {
            Player::One => (navigator.p1_locked, p1_char),
            Player::Two => (navigator.p2_locked, p2_char),
        };

        text.sections[0].style.color = if locked {
            CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR
        } else {
            GENERIC_TEXT_COLOR
        };

        *visibility = if indicator.character == *character {
            Visibility::Inherited // Visible, but only if parent is
        } else {
            Visibility::Hidden
        };
    }
}
