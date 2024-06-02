use bevy::prelude::*;
use wag_core::Player;

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

    // Bottom
    pub grid_items: Vec<Entity>,
}
impl ShopComponentsBuilder {
    pub fn build(self) -> ShopComponents {
        ShopComponents {
            countdown: self.countdown.expect("fully built UI"),
            countdown_text: self.countdown_text.expect("fully built UI"),
            big_icon: self.big_icon.expect("fully built UI"),
            item_name: self.item_name.expect("fully built UI"),
            explanation: self.explanation.expect("fully built UI"),
            cost: self.cost.expect("fully built UI"),
            dependencies: self.dependencies.expect("fully built UI"),
            grid_items: self.grid_items,
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

    // Bottom
    pub grid_items: Vec<Entity>,
}

#[derive(Debug)]
pub struct Shop {
    pub components: ShopComponents,
    pub selected_index: usize,
    pub max_index: usize, // Duplicated here for ease of access
    pub closed: bool,
}
impl Shop {
    pub fn get_selected_slot(&self) -> Entity {
        self.components.grid_items[self.selected_index]
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
