use std::collections::VecDeque;

use bevy::prelude::*;
use wag_core::{GameState, InCharacterSelect, LocalState, MatchState, SoundEffect, WagInputEvent};

use crate::{assets::Fonts, event_spreading::PlaySound};

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
        .add_systems(Update, update_menu_inputs)
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
                .run_if(in_state(GameState::Local(LocalState::ControllerAssignment))),
        )
        .add_systems(
            Update,
            (
                character_select::navigate_character_select,
                character_select::update_character_select_visuals,
            )
                .chain()
                .run_if(in_state(InCharacterSelect)),
        )
        .add_systems(OnEnter(MatchState::EndScreen), end_screen::setup_end_screen)
        .add_systems(
            Update,
            (
                end_screen::navigate_end_screen,
                end_screen::update_end_screen_visuals,
            )
                .chain()
                .run_if(in_state(MatchState::EndScreen))
                .after(end_screen::setup_end_screen),
        )
        .add_systems(OnExit(GameState::MainMenu), play_transition_noise)
        .add_systems(
            OnExit(GameState::Local(LocalState::ControllerAssignment)),
            play_transition_noise,
        )
        .add_systems(
            OnExit(GameState::Local(LocalState::CharacterSelect)),
            play_transition_noise,
        )
        .add_systems(OnExit(MatchState::EndScreen), play_transition_noise);
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
struct MenuInputs(VecDeque<WagInputEvent>);

// This is a workaround. Inputs would otherwise be duplicated per system, which causes
// duplication issues during state transitions.
fn update_menu_inputs(
    mut mi: ResMut<MenuInputs>,
    mut events: EventReader<WagInputEvent>,
    match_state: Res<State<MatchState>>,
) {
    let evs = events.read();

    if !matches!(*match_state.get(), MatchState::EndScreen | MatchState::None) {
        return;
    }

    for ev in evs {
        mi.push_back(ev.to_owned());
    }
}

fn play_transition_noise(mut commands: Commands) {
    commands.trigger(PlaySound(SoundEffect::PlasticCupFlick));
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
