use bevy::prelude::*;
use foundation::{
    GameState, InCharacterSelect, LocalState, MatchState, RollbackSchedule, SoundEffect, SystemStep,
};

use crate::assets::Fonts;

mod character_select;
mod controller_assignment;
mod credits;
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
                credits::setup_credits_menu,
            ),
        )
        .add_systems(
            RollbackSchedule,
            (
                (
                    main_menu::navigate_main_menu,
                    main_menu::update_main_menu_visuals,
                )
                    .chain()
                    .run_if(in_state(GameState::MainMenu)),
                credits::navigate_credits.run_if(in_state(GameState::Credits)),
                (
                    controller_assignment::navigate_controller_assignment_menu,
                    controller_assignment::update_controller_assignment_menu_visuals,
                )
                    .chain()
                    .run_if(in_state(GameState::Local(LocalState::ControllerAssignment))),
                (
                    character_select::navigate_character_select,
                    character_select::update_character_select_visuals,
                )
                    .chain()
                    .run_if(in_state(InCharacterSelect)),
                (
                    end_screen::navigate_end_screen,
                    end_screen::update_end_screen_visuals,
                )
                    .chain()
                    .run_if(in_state(MatchState::EndScreen))
                    .after(end_screen::setup_end_screen),
            )
                .chain()
                .in_set(SystemStep::MenuNavigation),
        )
        .add_systems(OnEnter(MatchState::EndScreen), end_screen::setup_end_screen)
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

fn play_transition_noise(mut commands: Commands) {
    commands.trigger(SoundEffect::PlasticCupFlick);
}

fn setup_view_title(root: &mut ChildBuilder, fonts: &Fonts, text: impl Into<String>) {
    root.spawn((
        Text::new(text),
        TextFont {
            font: fonts.basic.clone(),
            font_size: 128.0,
            ..default()
        },
        Name::new("Title"),
    ));
}

fn setup_view_subtitle(root: &mut ChildBuilder, fonts: &Fonts, text: impl Into<String>) {
    root.spawn((
        Text::new(text),
        TextFont {
            font: fonts.basic.clone(),
            font_size: 64.0,
            ..default()
        },
        Name::new("Title"),
    ));
}
