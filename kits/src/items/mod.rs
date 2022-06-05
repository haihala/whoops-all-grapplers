mod inventory;
mod item_id;

pub use inventory::Inventory;
pub use item_id::ItemId;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct Item {
    pub tier: usize,
    pub cost: usize,
    pub is_starter: bool,
}
