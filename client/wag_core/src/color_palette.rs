use bevy::prelude::*;

// Resource bars
pub const HEALTH_BAR_COLOR: Color = Color::rgb(0.9, 0.0, 0.0);

pub const METER_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::rgb(0.04, 0.5, 0.55);
pub const METER_BAR_FULL_SEGMENT_COLOR: Color = Color::rgb(0.14, 0.7, 0.8);

pub const CHARGE_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::rgb(0.05, 0.4, 0.55);
pub const CHARGE_BAR_FULL_SEGMENT_COLOR: Color = Color::rgb(0.9, 0.1, 0.3);

// Texts (general)
pub const GENERIC_TEXT_COLOR: Color = Color::WHITE;
pub const ROUND_TIMER_TEXT_COLOR: Color = Color::WHITE;
pub const RESOURCE_COUNTER_TEXT_COLOR: Color = Color::WHITE;

// General utils
pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

// Shop
pub const ITEM_SLOT_HIGHLIGHT_COLOR: Color = Color::rgb(195.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0); // WWE Raw red
pub const ITEM_SLOT_DEFAULT_COLOR: Color = Color::GRAY;
pub const ITEM_SLOT_DISABLED_COLOR: Color = Color::BISQUE;
pub const SHOP_TIMER_BACKGROUND_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.8);
pub const SHOP_DIVIDER_COLOR: Color = Color::BLACK;
pub const SHOP_DARK_BACKGROUND_COLOR: Color = Color::DARK_GRAY;
pub const SHOP_LIGHT_BACKGROUND_COLOR: Color = Color::GRAY;

// Box visualizations
pub const HITBOX_VISUALIZATION_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const HURTBOX_VISUALIZATION_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
pub const PUSHBOX_VISUALIZATION_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);

// Notifications
pub const NOTIFICATION_BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.3);
pub const NOTIFICATION_TEXT_COLOR: Color = Color::BLACK;

// Signal colors
pub const HIT_FLASH_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);