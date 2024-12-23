use bevy::prelude::*;

use foundation::{InMatch, MatchState, RollbackSchedule, SystemStep};

mod combat;
mod round_text;
mod shop;
mod utils;
mod views;

pub use utils::*;

pub use combat::setup_combat_hud;
pub use combat::Notifications;
pub use shop::setup_shop;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(views::ViewsPlugin)
            .insert_resource(Notifications::default())
            .add_systems(
                RollbackSchedule,
                (
                    set_ui_scale,
                    (
                        combat::update_bars,
                        combat::update_counters,
                        combat::update_timer,
                        combat::update_score,
                    )
                        .chain()
                        .run_if(in_state(MatchState::Combat)),
                    (
                        combat::update_notifications,
                        combat::update_combo_counters,
                        round_text::update_round_text,
                    )
                        .chain()
                        .run_if(in_state(InMatch)),
                )
                    .chain()
                    .in_set(SystemStep::UI),
            )
            .add_systems(
                RollbackSchedule,
                (
                    shop::navigate_shop,
                    shop::update_slot_visuals,
                    shop::update_top_bar_moneys,
                    shop::update_top_bar_scores,
                    shop::update_info_panel,
                    shop::handle_shop_ending,
                )
                    .chain()
                    .run_if(in_state(MatchState::Shop))
                    .in_set(SystemStep::Shop),
            )
            .add_systems(PostStartup, round_text::setup_round_info_text);
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
