use bevy::prelude::*;

use wag_core::Player;

#[derive(Default)]
pub struct ShopRootBuilder {
    // Top
    pub big_icon: Option<Entity>,
    pub explanation_box: Option<Entity>,

    // Middle
    pub owned_slots: Vec<Entity>,

    // Bottom
    pub consumables: Vec<Entity>,
    pub basics: Vec<Entity>,
    pub upgrades: Vec<Entity>,
}
impl ShopRootBuilder {
    pub fn build(self, owner: Player) -> ShopRoot {
        // Make sure there is at least one of each type, no problems in building the UI
        assert!(!self.owned_slots.is_empty());
        assert!(!self.consumables.is_empty());
        assert!(!self.basics.is_empty());
        // assert!(self.upgrades.len() > 1);

        ShopRoot {
            owner,
            big_icon: self.big_icon.expect("fully built UI"),
            explanation_box: self.explanation_box.expect("fully built UI"),
            owned_slots: self.owned_slots,
            consumables: self.consumables,
            basics: self.basics,
            upgrades: self.upgrades,
        }
    }
}

#[derive(Debug, Component)]
pub struct ShopRoot {
    pub owner: Player,

    // Top
    pub big_icon: Entity,
    pub explanation_box: Entity,

    // Middle
    pub owned_slots: Vec<Entity>,

    // Bottom
    pub consumables: Vec<Entity>,
    pub basics: Vec<Entity>,
    pub upgrades: Vec<Entity>,
}
