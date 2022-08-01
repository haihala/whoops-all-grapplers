mod inventory;
pub use inventory::Inventory;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct Item {
    pub tier: usize,
    pub cost: usize,
    pub is_starter: bool,
}
