mod navigation;
mod setup_shop;
mod shops_resource;

pub(super) use navigation::{navigate_shop, update_slot_visuals};
pub(super) use setup_shop::setup_shop;
pub use shops_resource::{ShopComponents, Shops};

const INVENTORY_SLOTS: usize = 6;
