mod inventory;
pub use inventory::Inventory;
use wag_core::ItemId;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub enum ItemCategory {
    Consumable,
    #[default]
    Basic,
    Upgrade(Vec<ItemId>),
}

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct Item {
    // TODO: Icons here
    pub category: ItemCategory,
    pub cost: usize,
}
