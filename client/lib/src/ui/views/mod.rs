use std::collections::VecDeque;

use bevy::{input::gamepad::GamepadEvent, prelude::*};
use wag_core::{GameState, InMenu};

use crate::assets::Fonts;

mod character_select;
mod controller_assignment;
mod end_screen;
mod main_menu;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            (
                main_menu::setup_main_menu,
                controller_assignment::setup_controller_assignment,
                character_select::setup_character_select,
            ),
        )
        .init_resource::<MenuInputs>()
        .add_systems(Update, update_menu_inputs.run_if(in_state(InMenu)))
        .add_systems(
            Update,
            (
                main_menu::navigate_main_menu,
                main_menu::update_main_menu_visuals,
            )
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                controller_assignment::navigate_controller_assignment_menu,
                controller_assignment::update_controller_assignment_menu_visuals,
            )
                .chain()
                .run_if(in_state(GameState::ControllerAssignment)),
        )
        .add_systems(
            Update,
            (
                character_select::navigate_character_select,
                character_select::update_character_select_visuals,
            )
                .chain()
                .run_if(in_state(GameState::CharacterSelect)),
        )
        .add_systems(OnEnter(GameState::EndScreen), end_screen::setup_end_screen)
        .add_systems(
            Update,
            (
                end_screen::navigate_end_screen,
                end_screen::update_end_screen_visuals,
            )
                .chain()
                .run_if(in_state(GameState::EndScreen))
                .after(end_screen::setup_end_screen),
        );
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
struct MenuInputs(VecDeque<GamepadEvent>);

// This is a workaround. Inputs would otherwise be duplicated per system, which causes
// duplication issues during state transitions.
fn update_menu_inputs(mut mi: ResMut<MenuInputs>, mut events: EventReader<GamepadEvent>) {
    for ev in events.read() {
        mi.push_back(ev.to_owned());
    }
}

fn setup_view_title(root: &mut ChildBuilder, fonts: &Fonts, text: impl Into<String>) {
    root.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: fonts.basic.clone(),
                font_size: 128.0,
                ..default()
            },
        ),
        Name::new("Title"),
    ));
}

fn setup_view_subtitle(root: &mut ChildBuilder, fonts: &Fonts, text: impl Into<String>) {
    root.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: fonts.basic.clone(),
                font_size: 64.0,
                ..default()
            },
        ),
        Name::new("Title"),
    ));
}
