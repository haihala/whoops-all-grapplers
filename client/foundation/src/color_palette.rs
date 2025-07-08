use bevy::color::palettes::css::*;
use bevy::prelude::*;

const WWE_RAW_RED: Color = Color::srgb(195.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0);

// Resource bars
pub const HEALTH_BAR_COLOR: Color = Color::srgb(0.9, 0.0, 0.0);

pub const METER_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::srgb(0.04, 0.5, 0.55);
pub const METER_BAR_FULL_SEGMENT_COLOR: Color = Color::srgb(0.14, 0.7, 0.8);

pub const CHARGE_BAR_PARTIAL_SEGMENT_COLOR: Color = Color::srgb(0.05, 0.4, 0.55);
pub const CHARGE_BAR_FULL_SEGMENT_COLOR: Color = Color::srgb(0.9, 0.1, 0.3);

// Texts (general)
pub const GENERIC_TEXT_COLOR: Color = Color::WHITE;
pub const MAIN_MENU_HIGHLIGHT_TEXT_COLOR: Color = Color::BLACK;
pub const CHARACTER_SELECT_HIGHLIGHT_TEXT_COLOR: Color = WWE_RAW_RED;
pub const ROUND_TIMER_TEXT_COLOR: Color = Color::WHITE;
pub const RESOURCE_COUNTER_TEXT_COLOR: Color = Color::WHITE;
pub const COMBO_COUNTER_TEXT_COLOR: Color = Color::WHITE;

// General utils
pub const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

// Shop
pub const ITEM_SLOT_HIGHLIGHT_COLOR: Color = WWE_RAW_RED;
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
pub const GENERIC_AREA_VISUALIZATION_COLOR: Color = Color::Srgba(YELLOW);

// Notifications
pub const NOTIFICATION_BACKGROUND_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.3);
pub const NOTIFICATION_TEXT_COLOR: Color = Color::BLACK;

// Signal colors
pub const HIT_FLASH_COLOR: Color = Color::WHITE;
pub const GI_PARRY_FLASH_COLOR: Color = Color::Srgba(ALICE_BLUE);
pub const TRACK_SPIKES_FLASH_COLOR: Color = Color::Srgba(YELLOW);
pub const JACKPOT_COLOR: Color = Color::srgb(0.2, 0.9, 0.1);
pub const JACKPOT_RING_BASE_COLOR: Color = Color::WHITE;

// Shader effects
pub const HIT_SPARK_BASE_COLOR: Color = TRANSPARENT;
pub const HIT_SPARK_MID_COLOR: Color = Color::srgb(1.0, 0.9, 0.7);
pub const HIT_SPARK_EDGE_COLOR: Color = Color::srgb(1.0, 1.0, 0.1);

pub const BLOCK_EFFECT_BASE_COLOR: Color = Color::WHITE;
pub const BLOCK_EFFECT_EDGE_COLOR: Color = Color::srgb(0.1, 0.2, 1.0);

pub const THROW_TECH_RING_EDGE_COLOR: Color = Color::WHITE;
pub const THROW_TECH_RING_BASE_COLOR: Color = Color::srgb(0.2, 1.0, 0.5);

pub const RC_PULSE_BASE_COLOR: Color = METER_BAR_PARTIAL_SEGMENT_COLOR;
pub const RC_PULSE_EDGE_COLOR: Color = METER_BAR_FULL_SEGMENT_COLOR;

pub const SPEED_LINES_BASE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const SPEED_LINES_EDGE_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

pub const CLASH_SPARK_BASE_COLOR: Color = Color::srgb(1.0, 0.5, 1.0);
pub const CLASH_SPARK_EDGE_COLOR: Color = Color::srgb(0.9, 0.1, 0.9);

pub const LIGHTNING_BOLT_INNER_COLOR: Color = Color::WHITE;
pub const LIGHTNING_BOLT_OUTER_COLOR: Color = Color::srgb(0.3, 0.4, 1.0);

pub const OPENER_INNER_COLOR: Color = Color::BLACK;
pub const HIGH_OPENER_COLOR: Color = Color::srgb(1.0, 0.1, 0.1);
pub const MID_OPENER_COLOR: Color = Color::srgb(1.0, 0.8, 0.1);
pub const LOW_OPENER_COLOR: Color = Color::srgb(0.0, 0.8, 0.9);

pub const PEBBLE_BORDER_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);
pub const PEBBLE_INNER_COLOR: Color = Color::srgb(0.33, 0.33, 0.33);

pub const SPARK_BURST_BORDER_COLOR: Color = Color::srgb(1.0, 0.7, 0.3);
pub const SPARK_BURST_INNER_COLOR: Color = Color::srgb(1.0, 0.8, 0.6);

pub const WEAKEN_STATUS_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

// Ronin sword slashes
pub const FAST_SWORD_VFX: Color = Color::srgb(0.8, 0.8, 0.8);
pub const STRONG_SWORD_VFX: Color = Color::srgb(0.9, 0.6, 0.5);
pub const METERED_SWORD_VFX: Color = Color::srgb(0.5, 0.6, 0.9);

// Player colors
pub const RONIN_ALT_SHIRT_COLOR: Color = Color::WHITE;
pub const RONIN_ALT_JEANS_COLOR: Color = Color::Srgba(MIDNIGHT_BLUE);
pub const RONIN_ALT_HELMET_COLOR: Color = Color::srgb(38.0 / 255.0, 50.0 / 255.0, 100.0 / 255.0);

pub const CPO_ALT_SHIRT_COLOR: Color = Color::srgb(250.0 / 255.0, 128.0 / 255.0, 114.0 / 255.0);
pub const CPO_ALT_SOCKS_COLOR: Color = Color::WHITE;

// UI
pub const LOADING_SCREEN_BACKGROUND: Color = Color::srgb(0.2, 0.2, 0.2);
pub const CONTROLLER_ASSIGNMENT_SIDE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const VERTICAL_MENU_OPTION_BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);
