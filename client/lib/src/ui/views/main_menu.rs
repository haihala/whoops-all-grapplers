use bevy::{input::gamepad::GamepadEvent, prelude::*};
use wag_core::{GameState, GENERIC_TEXT_COLOR, MAIN_MENU_HIGHLIGHT_TEXT_COLOR};

use crate::{
    assets::Fonts,
    entity_management::VisibleInStates,
    ui::{OnPress, VerticalMenuNavigation},
};

use super::{setup_view_title, MenuInputs};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct MainMenuNav(VerticalMenuNavigation);

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
        root.spawn((
            TextBundle::from_section(
                "Local play!",
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 36.0,
                    ..default()
                },
            ),
            Name::new("Local play"),
        ))
        .observe(go_to_controller_assignment)
        .id(),
        root.spawn((
            TextBundle::from_section(
                "Quit",
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 36.0,
                    ..default()
                },
            ),
            Name::new("Quit"),
        ))
        .observe(quit)
        .id(),
    ]
}

fn go_to_controller_assignment(_: Trigger<OnPress>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::ControllerAssignment);
}

fn quit(_: Trigger<OnPress>, mut exit: EventWriter<AppExit>) {
    exit.send_default();
}

pub fn navigate_main_menu(
    mut commands: Commands,
    mut mmn: ResMut<MainMenuNav>,
    mut events: ResMut<MenuInputs>,
) {
    // TODO: Analog stick
    while let Some(ev) = events.pop_front() {
        match ev {
            GamepadEvent::Button(ev_btn) if ev_btn.value == 1.0 => match ev_btn.button_type {
                GamepadButtonType::DPadUp => mmn.up(),
                GamepadButtonType::DPadDown => mmn.down(),
                // TODO: This triggers automatically when rematching
                GamepadButtonType::South => commands.trigger_targets(OnPress, mmn.selected),
                _ => {}
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
