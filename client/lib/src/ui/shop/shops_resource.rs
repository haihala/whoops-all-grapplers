use bevy::prelude::*;
use wag_core::Player;

use super::navigation::{ShopCategory, ShopNavigation};

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

#[derive(Debug)]
pub struct Shop {
    pub components: ShopComponents,
    pub navigation: ShopNavigation,
}
impl Shop {
    pub fn get_selected_slot(&self) -> Entity {
        match self.navigation {
            ShopNavigation::Owned(index) => self.components.owned_slots[index],
            ShopNavigation::Available(category, index) => match category {
                ShopCategory::Consumable => self.components.consumables[index],
                ShopCategory::Basic => self.components.basics[index],
                ShopCategory::Upgrade => self.components.upgrades[index],
            },
        }
    }

    pub fn category_size(&self, category: ShopCategory) -> usize {
        match category {
            ShopCategory::Consumable => self.components.consumables.len(),
            ShopCategory::Basic => self.components.basics.len(),
            ShopCategory::Upgrade => self.components.upgrades.len(),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Shops {
    pub player_one: Shop,
    pub player_two: Shop,
}
impl Shops {
    pub fn get_mut_shop(&mut self, player: &Player) -> &mut Shop {
        match player {
            Player::One => &mut self.player_one,
            Player::Two => &mut self.player_two,
        }
    }
}
