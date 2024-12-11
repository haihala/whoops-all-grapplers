use bevy::prelude::*;
use foundation::{
    GameButton, GameState, InputEvent, LocalController, LocalState, OnlineState, StickPosition,
    GENERIC_TEXT_COLOR, MAIN_MENU_HIGHLIGHT_TEXT_COLOR,
};

use crate::{assets::Fonts, entity_management::VisibleInStates, ui::VerticalMenuNavigation};

use super::{setup_view_title, MenuInputs};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct MainMenuNav(VerticalMenuNavigation);

#[derive(Debug, Component, Clone, Copy)]
pub enum MainMenuOptions {
    LocalPlay,
    OnlinePlay,
    QuitToDesktop,
}

impl std::fmt::Display for MainMenuOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MainMenuOptions::LocalPlay => "Local play",
                MainMenuOptions::OnlinePlay => "Online play",
                MainMenuOptions::QuitToDesktop => "Quit to desktop",
            }
        )
    }
}

pub fn setup_main_menu(mut commands: Commands, fonts: Res<Fonts>) {
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
                    row_gap: Val::Percent(0.5),
                    padding: UiRect::all(Val::Percent(20.0)),
                    ..default()
                },
                ..default()
            },
            VisibleInStates(vec![GameState::MainMenu]),
            Name::new("Main menu UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, "Whoops, all grapplers!");
            let buttons = setup_buttons(cb, &fonts);
            navigation = Some(VerticalMenuNavigation::from_buttons(buttons));
        });

    if let Some(nav) = navigation {
        commands.insert_resource(MainMenuNav(nav));
    }
}

fn setup_buttons(root: &mut ChildBuilder, fonts: &Fonts) -> Vec<Entity> {
    vec![
        MainMenuOptions::LocalPlay,
        MainMenuOptions::OnlinePlay,
        MainMenuOptions::QuitToDesktop,
    ]
    .into_iter()
    .map(|opt| {
        root.spawn((
            TextBundle::from_section(
                opt.to_string(),
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 36.0,
                    ..default()
                },
            ),
            Name::new(opt.to_string()),
            opt,
        ))
        .id()
    })
    .collect()
}

pub fn navigate_main_menu(
    mut commands: Commands,
    mut nav: ResMut<MainMenuNav>,
    mut events: ResMut<MenuInputs>,
    options: Query<&MainMenuOptions>,
    mut state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    while let Some(ev) = events.pop_front() {
        match ev.event {
            InputEvent::Point(StickPosition::N) => nav.up(),
            InputEvent::Point(StickPosition::S) => nav.down(),
            InputEvent::Press(GameButton::Fast) => match options.get(nav.selected).unwrap() {
                MainMenuOptions::LocalPlay => {
                    state.set(GameState::Local(LocalState::ControllerAssignment));
                }
                MainMenuOptions::OnlinePlay => {
                    commands.insert_resource(LocalController(ev.player_handle));
                    state.set(GameState::Online(OnlineState::CharacterSelect));
                }
                MainMenuOptions::QuitToDesktop => {
                    exit.send_default();
                }
            },
            _ => {}
        }
    }
}

pub fn update_main_menu_visuals(mmn: Res<MainMenuNav>, mut texts: Query<(Entity, &mut Text)>) {
    if !mmn.is_changed() {
        return;
    }

    for (entity, mut text) in &mut texts {
        text.sections[0].style.color = if entity == mmn.selected {
            MAIN_MENU_HIGHLIGHT_TEXT_COLOR
        } else {
            GENERIC_TEXT_COLOR
        }
    }
}
