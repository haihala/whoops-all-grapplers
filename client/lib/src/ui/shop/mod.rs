mod setup_shop;
mod shop_inputs;
mod shop_rendering;
mod shops_resource;

pub(super) use setup_shop::setup_shop;
pub(super) use shop_inputs::navigate_shop;
pub(super) use shop_rendering::{
    handle_shop_ending, update_info_panel, update_slot_visuals, update_top_bar_moneys,
    update_top_bar_scores,
};
pub use shops_resource::Shops;

pub const SHOP_COLUMNS: usize = 10;
