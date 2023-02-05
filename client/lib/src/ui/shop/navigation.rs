use bevy::prelude::*;
use input_parsing::InputParser;
use wag_core::{MoveId, Player};

use crate::assets::Colors;

use super::{shops_resource::Shop, Shops, INVENTORY_SLOTS};

#[derive(Component, Debug, Clone, Copy)]
pub enum ShopNavigation {
    Owned(usize),
    Available(ShopCategory, usize),
}

impl Default for ShopNavigation {
    fn default() -> Self {
        Self::Owned(0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShopCategory {
    Consumable,
    Basic,
    Upgrade,
}
impl ShopCategory {
    fn next(&self) -> ShopCategory {
        match self {
            ShopCategory::Consumable => ShopCategory::Basic,
            ShopCategory::Basic => ShopCategory::Upgrade,
            ShopCategory::Upgrade => ShopCategory::Consumable,
        }
    }

    fn previous(&self) -> ShopCategory {
        match self {
            ShopCategory::Consumable => ShopCategory::Upgrade,
            ShopCategory::Basic => ShopCategory::Consumable,
            ShopCategory::Upgrade => ShopCategory::Basic,
        }
    }

    // This is used to go from available items back to the inventory at an appropriate index
    fn inventory_index(&self) -> usize {
        match self {
            ShopCategory::Consumable => 0,
            ShopCategory::Basic => INVENTORY_SLOTS / 2,
            ShopCategory::Upgrade => INVENTORY_SLOTS,
        }
    }
}

#[derive(Component, Default)]
pub enum ShopSlotState {
    #[default]
    Default,
    Highlighted,
    Disabled,
}

pub fn navigate_shop(
    mut parsers: Query<(&mut InputParser, &Player)>,
    mut slots: Query<&mut ShopSlotState>,
    mut shops: ResMut<Shops>,
) {
    for (mut parser, player) in &mut parsers {
        let shop = shops.get_mut_shop(player);
        let events = parser.get_events();

        for event in events {
            match event {
                MoveId::Up => move_selection(&mut slots, shop, Up),
                MoveId::Down => move_selection(&mut slots, shop, Down),
                MoveId::Left => move_selection(&mut slots, shop, Left),
                MoveId::Right => move_selection(&mut slots, shop, Right),
                MoveId::Primary => primary_button_pressed(shop),
                // MoveId::Secondary => todo!(),
                // MoveId::Back => todo!(),
                _ => {}
            };
        }
        parser.clear();
    }
}

enum CardinalDiretion {
    Up,
    Down,
    Left,
    Right,
}
use CardinalDiretion::*;

fn move_selection(
    slots: &mut Query<&mut ShopSlotState>,
    shop: &mut Shop,
    direction: CardinalDiretion,
) {
    // Normalize currently selected space
    set_selected_slot(slots, shop, ShopSlotState::Default);

    shop.navigation = match shop.navigation {
        ShopNavigation::Owned(index) => owned_slot_navigation(shop, direction, index),
        ShopNavigation::Available(category, index) => {
            available_slot_navigation(shop, direction, category, index)
        }
    };

    // Highlight new active space
    set_selected_slot(slots, shop, ShopSlotState::Highlighted);
}

fn set_selected_slot(slots: &mut Query<&mut ShopSlotState>, shop: &mut Shop, state: ShopSlotState) {
    let entity = shop.get_selected_slot();
    let mut slot = slots.get_mut(entity).unwrap();
    *slot = state;
}
const LOW_BOUND: usize = INVENTORY_SLOTS / 2 - 1;
const MID_BOUND: usize = INVENTORY_SLOTS / 2 + 1;

fn owned_slot_navigation(
    shop: &mut Shop,
    direction: CardinalDiretion,
    index: usize,
) -> ShopNavigation {
    let category = match index {
        0..LOW_BOUND => ShopCategory::Consumable,
        LOW_BOUND..MID_BOUND => ShopCategory::Basic,
        MID_BOUND..=INVENTORY_SLOTS => ShopCategory::Upgrade,
        _ => panic!("Weird index when moving in the shop"),
    };

    match direction {
        Up => ShopNavigation::Available(category, shop.category_size(category)),
        Down => ShopNavigation::Available(category, 0),
        Left => ShopNavigation::Owned(if index > 0 {
            index - 1
        } else {
            INVENTORY_SLOTS - 1
        }),
        Right => ShopNavigation::Owned(if index < INVENTORY_SLOTS - 1 {
            index + 1
        } else {
            0
        }),
    }
}

fn available_slot_navigation(
    shop: &mut Shop,
    direction: CardinalDiretion,
    category: ShopCategory,
    index: usize,
) -> ShopNavigation {
    match direction {
        Up => {
            if index > 0 {
                ShopNavigation::Available(category, index - 1)
            } else {
                ShopNavigation::Owned(category.inventory_index())
            }
        }
        Down => {
            if index < shop.category_size(category) - 1 {
                ShopNavigation::Available(category, index + 1)
            } else {
                ShopNavigation::Owned(category.inventory_index())
            }
        }
        Left => switch_category(shop, category.previous(), index),
        Right => switch_category(shop, category.next(), index),
    }
}

fn switch_category(shop: &Shop, new_category: ShopCategory, old_index: usize) -> ShopNavigation {
    // Make sure index is valid if moving to a category with fewer items
    let new_index = old_index.min(shop.category_size(new_category));

    ShopNavigation::Available(new_category, new_index)
}

fn primary_button_pressed(shop: &mut Shop) {
    match shop.navigation {
        ShopNavigation::Owned(index) => sell(shop, index),
        ShopNavigation::Available(category, index) => buy(shop, category, index),
    }
}

fn sell(shop: &mut Shop, index: usize) {
    todo!();
}

fn buy(shop: &mut Shop, category: ShopCategory, index: usize) {
    todo!();
}

pub fn update_slot_visuals(
    mut query: Query<(&ShopSlotState, &mut BackgroundColor)>,
    colors: Res<Colors>,
) {
    for (state, mut color) in &mut query {
        *color = match state {
            ShopSlotState::Default => colors.default_item_slot,
            ShopSlotState::Highlighted => colors.highlighted_item_slot,
            ShopSlotState::Disabled => colors.disabled_item_slot,
        }
        .into()
    }
}
