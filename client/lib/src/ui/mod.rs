use bevy::prelude::*;

use wag_core::{GameState, InMatch, PRE_ROUND_DURATION};

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
                    .run_if(in_state(GameState::Combat)),
            )
            .add_systems(OnEnter(GameState::PostRound), combat::update_score)
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
                    .run_if(in_state(GameState::Shop)),
            )
            .add_systems(
                OnEnter(GameState::SetupMatch),
                (
                    combat::setup_combat_hud,
                    round_text::setup_round_info_text,
                    shop::setup_shop,
                    // This only works because no other place uses this state
                    |mut commands: Commands, mut state: ResMut<NextState<GameState>>| {
                        state.set(GameState::PreRound);
                        commands.insert_resource(TransitionTimer::from(Timer::from_seconds(
                            PRE_ROUND_DURATION,
                            TimerMode::Once,
                        )));
                    },
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
