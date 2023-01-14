mod inventory;
pub use inventory::Inventory;
use wag_core::{Icon, ItemId};

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub enum ItemCategory {
    Consumable,
    #[default]
    Basic,
    Upgrade(Vec<ItemId>),
}

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct Item {
    pub icon: Option<Icon>,
    pub category: ItemCategory,
    pub cost: usize,
}
