use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use input_parsing::InputParser;
use wag_core::{Facing, ItemId, MoveId, Owner, Player, INVENTORY_SIZE, SELL_RETURN};

use super::{setup_shop::ShopItem, shops_resource::Shop, Shops};

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
            ShopCategory::Basic => INVENTORY_SIZE / 2,
            ShopCategory::Upgrade => INVENTORY_SIZE - 1,
        }
    }
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum ShopSlotState {
    #[default]
    Default,
    Highlighted,
    Disabled,
}

pub fn navigate_shop(
    mut parsers: Query<(
        &mut InputParser,
        &Player,
        &mut Inventory,
        &Character,
        &Facing,
    )>,
    mut slots: Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
    mut shops: ResMut<Shops>,
) {
    for (mut parser, player, mut inventory, character, facing) in &mut parsers {
        let events = parser.get_events();
        let shop = shops.get_mut_shop(player);

        if events.is_empty() || shop.closed {
            continue;
        }

        for event in events {
            match event {
                MoveId::Up => move_selection(shop, Up),
                MoveId::Down => move_selection(shop, Down),
                MoveId::Back => move_selection(shop, Left.mirror_if(facing.to_flipped())),
                MoveId::Forward => move_selection(shop, Right.mirror_if(facing.to_flipped())),
                MoveId::Primary => primary_button_pressed(shop, &mut inventory, character, &slots),
                MoveId::Start => shop.closed = true,
                _ => {}
            };
        }

        parser.clear();

        let selected = shop.get_selected_slot();
        for (entity, owner, shop_item, mut slot_state) in &mut slots {
            if *player != **owner {
                continue;
            }

            let correct_state = if selected == entity {
                ShopSlotState::Highlighted
            } else if shop_item
                .map(|item| character.items.get(item).unwrap().cost > inventory.money)
                .unwrap_or_default()
            {
                ShopSlotState::Disabled
            } else {
                ShopSlotState::Default
            };

            // Hopefully only trigger change detection for the slots that actually changed
            if correct_state != *slot_state {
                *slot_state = correct_state;
            }
        }
    }
}

enum CardinalDiretion {
    Up,
    Down,
    Left,
    Right,
}
impl CardinalDiretion {
    fn mirror_if(self, condition: bool) -> Self {
        if !condition {
            self
        } else {
            match self {
                Left => Right,
                Right => Left,
                other => other,
            }
        }
    }
}
use CardinalDiretion::*;

fn move_selection(shop: &mut Shop, direction: CardinalDiretion) {
    shop.navigation = match shop.navigation {
        ShopNavigation::Owned(index) => owned_slot_navigation(shop, direction, index),
        ShopNavigation::Available(category, index) => {
            available_slot_navigation(shop, direction, category, index)
        }
    };
}

const LOW_BOUND: usize = INVENTORY_SIZE / 2 - 1;
const MID_BOUND: usize = INVENTORY_SIZE / 2 + 2;

fn owned_slot_navigation(
    shop: &mut Shop,
    direction: CardinalDiretion,
    index: usize,
) -> ShopNavigation {
    let category = match index {
        0..LOW_BOUND => ShopCategory::Consumable,
        LOW_BOUND..MID_BOUND => ShopCategory::Basic,
        MID_BOUND..=INVENTORY_SIZE => ShopCategory::Upgrade,
        _ => panic!("Weird index when moving in the shop"),
    };

    match direction {
        Up => ShopNavigation::Available(category, shop.category_size(category) - 1),
        Down => ShopNavigation::Available(category, 0),
        Left => ShopNavigation::Owned(if index > 0 {
            index - 1
        } else {
            INVENTORY_SIZE - 1
        }),
        Right => ShopNavigation::Owned(if index < INVENTORY_SIZE - 1 {
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
    let new_index = old_index.min(shop.category_size(new_category) - 1);

    ShopNavigation::Available(new_category, new_index)
}

fn primary_button_pressed(
    shop: &mut Shop,
    inventory: &mut Inventory,
    character: &Character,
    slots: &Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
) {
    match shop.navigation {
        ShopNavigation::Owned(index) => sell(inventory, character, index),
        ShopNavigation::Available(_, _) => buy(shop, inventory, character, slots),
    }
}

fn sell(inventory: &mut Inventory, character: &Character, index: usize) {
    if let Some(id) = inventory.items.get(index) {
        let refund = ((get_recursive_cost(character, id) as f32) * SELL_RETURN) as usize;
        inventory.sell(index, refund);
    }
}

pub fn get_recursive_cost(character: &Character, id: &ItemId) -> usize {
    let item = character.items.get(id).unwrap();

    (if let ItemCategory::Upgrade(deps) = &item.category {
        deps.iter()
            .map(|dependency| get_recursive_cost(character, dependency))
            .sum()
    } else {
        0
    }) + item.cost
}

fn buy(
    shop: &mut Shop,
    inventory: &mut Inventory,
    character: &Character,
    slots: &Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
) {
    let selected_slot = shop.get_selected_slot();
    let (_, _, selected_item, _) = slots.get(selected_slot).unwrap();
    let shop_item = selected_item.unwrap();

    let item = character.items.get(shop_item).unwrap().clone();
    if inventory.can_buy(&item) {
        inventory.buy(**shop_item, item)
    }
}
