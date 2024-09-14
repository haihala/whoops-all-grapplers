use bevy::prelude::*;

use wag_core::{
    GameState, InMatch, InMatchSetup, LocalState, MatchState, OnlineState, SynctestState,
    PRE_ROUND_DURATION,
};

mod combat;
mod round_text;
mod shop;
mod utils;
mod views;

pub use utils::*;

pub use combat::Notifications;

use crate::state_transitions::TransitionTimer;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(views::ViewsPlugin)
            .insert_resource(Notifications::default())
            .add_systems(
                Last,
                (
                    combat::update_bars,
                    combat::update_counters,
                    combat::update_timer,
                )
                    .run_if(in_state(MatchState::Combat)),
            )
            .add_systems(OnEnter(MatchState::PostRound), combat::update_score)
            .add_systems(
                Update,
                (combat::update_notifications, round_text::update_round_text)
                    .run_if(in_state(InMatch)),
            )
            .add_systems(
                Update,
                (
                    shop::navigate_shop,
                    shop::update_slot_visuals,
                    shop::update_top_bar_moneys,
                    shop::update_top_bar_scores,
                    shop::update_info_panel,
                    shop::handle_shop_ending,
                )
                    .run_if(in_state(MatchState::Shop)),
            )
            .add_systems(
                OnEnter(InMatchSetup),
                (
                    combat::setup_combat_hud,
                    round_text::setup_round_info_text,
                    shop::setup_shop,
                    // This only works because no other place uses this state
                    exit_match_setup,
                )
                    .chain(),
            )
            .add_systems(Update, set_ui_scale);
    }
}

fn set_ui_scale(
    windows: Query<&Window>,
    mut ui_scale: ResMut<UiScale>,
    mut local_width: Local<f32>,
) {
    let window = windows.single();

    if window.width() == *local_width {
        return;
    }

    ui_scale.0 = window.width() / 1920.0;
    *local_width = window.width();
}

fn exit_match_setup(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    let (next, after) = match current_state.get() {
        GameState::Online(_) => (
            GameState::Online(OnlineState::Match(MatchState::PreRound)),
            GameState::Online(OnlineState::Match(MatchState::Combat)),
        ),
        GameState::Local(_) => (
            GameState::Local(LocalState::Match(MatchState::PreRound)),
            GameState::Local(LocalState::Match(MatchState::Combat)),
        ),
        GameState::Synctest(_) => (
            GameState::Synctest(SynctestState::Match(MatchState::PreRound)),
            GameState::Synctest(SynctestState::Match(MatchState::Combat)),
        ),
        GameState::MainMenu => panic!("Trying to exit match setup in main menu"),
    };

    next_state.set(next);
    commands.insert_resource(TransitionTimer {
        timer: Timer::from_seconds(PRE_ROUND_DURATION, TimerMode::Once),
        state: after,
    });
}
