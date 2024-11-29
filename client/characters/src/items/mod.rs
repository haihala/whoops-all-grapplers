mod inventory;
mod universal_items;

pub use inventory::Inventory;
pub use universal_items::{universal_item_actions, universal_items};

use wag_core::{Icon, ItemId, Stats};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConsumableType {
    OneRound,
    UntilUsed,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ItemCategory {
    Consumable(ConsumableType),
    #[default]
    Basic,
    Upgrade(Vec<ItemId>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub category: ItemCategory,
    pub cost: usize,
    pub effect: Stats,
    pub explanation: String,
    pub icon: Icon,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            category: Default::default(),
            cost: Default::default(),
            effect: Default::default(),
            explanation: "Description missing".into(),
            icon: Icon::Blank,
        }
    }
}
