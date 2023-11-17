mod inventory;
pub use inventory::Inventory;
use wag_core::{ItemId, Stats};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ItemCategory {
    Consumable,
    #[default]
    Basic,
    Upgrade(Vec<ItemId>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    // TODO: Icons here
    pub category: ItemCategory,
    pub cost: usize,
    pub effect: Stats,
    pub explanation: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            category: Default::default(),
            cost: Default::default(),
            effect: Stats::identity(),
            explanation: "Description missing".into(),
        }
    }
}
