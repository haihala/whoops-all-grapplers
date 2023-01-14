use bevy::prelude::*;

#[derive(Default)]
pub struct ShopComponentsBuilder {
    // Top
    pub big_icon: Option<Entity>,
    pub explanation_box: Option<Entity>,

    // Middle
    pub owned_slots: Vec<Entity>,
    pub money_text: Option<Entity>,

    // Bottom
    pub consumables: Vec<Entity>,
    pub basics: Vec<Entity>,
    pub upgrades: Vec<Entity>,
}
impl ShopComponentsBuilder {
    pub fn build(self) -> ShopComponents {
        // Make sure there is at least one of each type, no problems in building the UI
        assert!(!self.owned_slots.is_empty());
        assert!(!self.consumables.is_empty());
        assert!(!self.basics.is_empty());
        // assert!(self.upgrades.len() > 1);

        ShopComponents {
            big_icon: self.big_icon.expect("fully built UI"),
            explanation_box: self.explanation_box.expect("fully built UI"),
            owned_slots: self.owned_slots,
            money_text: self.money_text.expect("fully built UI"),
            consumables: self.consumables,
            basics: self.basics,
            upgrades: self.upgrades,
        }
    }
}

#[derive(Debug)]
pub struct ShopComponents {
    // Top
    pub big_icon: Entity,
    pub explanation_box: Entity,

    // Middle
    pub owned_slots: Vec<Entity>,
    pub money_text: Entity,

    // Bottom
    pub consumables: Vec<Entity>,
    pub basics: Vec<Entity>,
    pub upgrades: Vec<Entity>,
}

#[derive(Debug, Resource)]
pub struct Shops {
    pub player_one: ShopComponents,
    pub player_two: ShopComponents,
}
