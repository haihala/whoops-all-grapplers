use bevy::prelude::*;

use wag_core::{in_combat, GameState};

mod combat;
mod round_text;
mod shop;

pub use combat::Notifications;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Notifications::default())
            .add_systems(
                (combat::update_bars, combat::update_timer)
                    .in_base_set(CoreSet::Last)
                    .distributive_run_if(in_combat),
            )
            .add_system(combat::update_notifications)
            .add_system(round_text::update_round_text)
            .add_systems(
                (
                    shop::navigate_shop,
                    shop::update_slot_visuals,
                    shop::update_info_panel,
                    shop::update_inventory_ui,
                    shop::handle_shop_ending,
                )
                    .in_set(OnUpdate(GameState::Shop)),
            )
            .add_startup_systems(
                (
                    combat::setup_combat_hud,
                    round_text::setup_round_info_text,
                    shop::setup_shop,
                )
                    .in_base_set(StartupSet::PostStartup),
            );
    }
}
