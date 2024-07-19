use bevy::color::palettes::css::*;
use bevy::prelude::*;

// Resource bars
pub const HEALTH_BAR_COLOR: Color = Color::srgb(0.9, 0.0, 0.0);

pub const METER_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::srgb(0.04, 0.5, 0.55);
pub const METER_BAR_FULL_SEGMENT_COLOR: Color = Color::srgb(0.14, 0.7, 0.8);

pub const CHARGE_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::srgb(0.05, 0.4, 0.55);
pub const CHARGE_BAR_FULL_SEGMENT_COLOR: Color = Color::srgb(0.9, 0.1, 0.3);

// Texts (general)
pub const GENERIC_TEXT_COLOR: Color = Color::WHITE;
pub const ROUND_TIMER_TEXT_COLOR: Color = Color::WHITE;
pub const RESOURCE_COUNTER_TEXT_COLOR: Color = Color::WHITE;

// General utils
pub const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

// Shop
pub const ITEM_SLOT_HIGHLIGHT_COLOR: Color = Color::srgb(195.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0); // WWE Raw red
pub const ITEM_SLOT_DEFAULT_COLOR: Color = Color::Srgba(GRAY);
pub const ITEM_SLOT_DISABLED_COLOR: Color = Color::Srgba(BISQUE);
pub const ITEM_SLOT_COMPONENT_COLOR: Color = Color::Srgba(YELLOW);
pub const ITEM_SLOT_UPGRADE_COLOR: Color = Color::Srgba(BLUE);
pub const ITEM_SLOT_OWNED_COLOR: Color = Color::Srgba(LIME);
pub const SHOP_TIMER_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
pub const SHOP_DIVIDER_COLOR: Color = Color::BLACK;
pub const SHOP_DARK_BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.25);

// Box visualizations
pub const HITBOX_VISUALIZATION_COLOR: Color = Color::Srgba(RED);
pub const HURTBOX_VISUALIZATION_COLOR: Color = Color::Srgba(LIME);
pub const PUSHBOX_VISUALIZATION_COLOR: Color = Color::Srgba(BLUE);

// Notifications
pub const NOTIFICATION_BACKGROUND_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.3);
pub const NOTIFICATION_TEXT_COLOR: Color = Color::BLACK;

// Signal colors
pub const HIT_FLASH_COLOR: Color = Color::WHITE;
pub const GI_PARRY_FLASH_COLOR: Color = Color::Srgba(ALICE_BLUE);
pub const TRACK_SPIKES_FLASH_COLOR: Color = Color::Srgba(YELLOW);

// Player colors
pub const MIZUKI_ALT_SHIRT_COLOR: Color = Color::WHITE;
pub const MIZUKI_ALT_JEANS_COLOR: Color = Color::Srgba(MIDNIGHT_BLUE);
pub const MIZUKI_ALT_HELMET_COLOR: Color = Color::srgb(38.0 / 255.0, 50.0 / 255.0, 100.0 / 255.0);

// UI
pub const LOADING_SCREEN_BACKGROUND: Color = Color::srgb(0.2, 0.2, 0.2);
