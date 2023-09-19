use bevy::prelude::*;

use wag_core::{in_combat, GameState};

mod combat;
mod round_text;
mod shop;

pub use combat::{Notifications, ResourceCounter};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Notifications::default())
            .add_systems(
                Last,
                (
                    combat::update_bars,
                    combat::update_counters,
                    combat::update_score,
                    combat::update_timer,
                )
                    .distributive_run_if(in_combat),
            )
            .add_systems(
                Update,
                (combat::update_notifications, round_text::update_round_text),
            )
            .add_systems(
                Update,
                (
                    shop::navigate_shop,
                    shop::update_slot_visuals,
                    shop::update_info_panel,
                    shop::update_inventory_ui,
                    shop::handle_shop_ending,
                )
                    .run_if(in_state(GameState::Shop)),
            )
            .add_systems(
                PostStartup,
                (
                    combat::setup_combat_hud,
                    round_text::setup_round_info_text,
                    shop::setup_shop,
                ),
            );
    }
}
