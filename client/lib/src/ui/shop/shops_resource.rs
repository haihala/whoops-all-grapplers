use bevy::prelude::*;
use wag_core::Player;

use super::shop_inputs::{ShopCategory, ShopNavigation};

#[derive(Default)]
pub struct ShopComponentsBuilder {
    // Countdown
    pub countdown: Option<Entity>,
    pub countdown_text: Option<Entity>,

    // Top
    pub big_icon: Option<Entity>,
    pub item_name: Option<Entity>,
    pub explanation: Option<Entity>,
    pub cost: Option<Entity>,
    pub dependencies: Option<Entity>,

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
        assert!(!self.upgrades.is_empty());

        ShopComponents {
            countdown: self.countdown.expect("fully built UI"),
            countdown_text: self.countdown_text.expect("fully built UI"),
            big_icon: self.big_icon.expect("fully built UI"),
            item_name: self.item_name.expect("fully built UI"),
            explanation: self.explanation.expect("fully built UI"),
            cost: self.cost.expect("fully built UI"),
            dependencies: self.dependencies.expect("fully built UI"),
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
    // Countdown
    pub countdown: Entity,
    pub countdown_text: Entity,

    // Top
    pub big_icon: Entity,
    pub item_name: Entity,
    pub explanation: Entity,
    pub cost: Entity,
    pub dependencies: Entity,

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
    pub closed: bool,
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

    pub fn get_shop(&self, player: &Player) -> &Shop {
        match player {
            Player::One => &self.player_one,
            Player::Two => &self.player_two,
        }
    }
}
