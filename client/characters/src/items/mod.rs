mod inventory;
pub use inventory::Inventory;
use wag_core::{ItemId, StatusEffect};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ItemCategory {
    Consumable,
    #[default]
    Basic,
    Upgrade(Vec<ItemId>),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    // TODO: Icons here
    pub category: ItemCategory,
    pub cost: usize,
    pub effect: StatusEffect,
    pub explanation: String,
}
