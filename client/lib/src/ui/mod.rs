use bevy::prelude::*;

use wag_core::GameState;

mod combat;
mod round_text;
mod shop;

pub use combat::Notifications;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Notifications::default())
            .add_system_set_to_stage(
                CoreStage::Last,
                SystemSet::new()
                    .with_run_criteria(State::on_update(GameState::Combat))
                    .with_system(combat::update_bars)
                    .with_system(combat::update_timer),
            )
            .add_system(combat::update_notifications)
            .add_system(round_text::update_round_text)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(State::on_update(GameState::Shop))
                    .with_system(shop::navigate_shop)
                    .with_system(shop::update_slot_visuals.after(shop::navigate_shop))
                    .with_system(shop::update_info_panel.after(shop::navigate_shop))
                    .with_system(shop::update_inventory_ui.after(shop::navigate_shop))
                    .with_system(shop::handle_shop_ending.after(shop::navigate_shop)),
            )
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new()
                    .with_system(combat::setup_combat_hud)
                    .with_system(round_text::setup_round_info_text)
                    .with_system(shop::setup_shop),
            );
    }
}
