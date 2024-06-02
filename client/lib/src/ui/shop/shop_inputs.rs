use bevy::prelude::*;
use characters::{Character, Inventory};
use input_parsing::InputParser;
use wag_core::{ActionId, Facing, Owner, Player};

use super::{setup_shop::ShopItem, shops_resource::Shop, Shops, SHOP_COLUMNS};

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
                ActionId::Up => move_selection(shop, Up),
                ActionId::Down => move_selection(shop, Down),
                ActionId::Back => move_selection(shop, Left.mirror_if(facing.to_flipped())),
                ActionId::Forward => move_selection(shop, Right.mirror_if(facing.to_flipped())),
                ActionId::Primary => buy(shop, &mut inventory, character, &slots),
                ActionId::Secondary => sell(shop, &mut inventory, character, &slots),
                ActionId::Start => shop.closed = true,
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
                .map(|item| character.items.get(&item.0).unwrap().cost > inventory.money)
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
    let row = shop.selected_index / SHOP_COLUMNS;
    let col = shop.selected_index % SHOP_COLUMNS;

    let selected_row_last_col = if shop.selected_index + SHOP_COLUMNS > shop.max_index {
        // Incomplete row
        shop.max_index % SHOP_COLUMNS - 1
    } else {
        SHOP_COLUMNS - 1
    };

    shop.selected_index = match direction {
        Up => {
            if row == 0 {
                // Handle flipping over
                let last_row_length = shop.max_index % SHOP_COLUMNS;

                if col > last_row_length {
                    // Put cursor on the second last row
                    shop.max_index - last_row_length - SHOP_COLUMNS + col
                } else {
                    shop.max_index - last_row_length + col
                }
            } else {
                shop.selected_index - SHOP_COLUMNS
            }
        }
        Down => {
            let potential_index = shop.selected_index + SHOP_COLUMNS;
            if potential_index > shop.max_index {
                col
            } else {
                potential_index
            }
        }
        Left => {
            if col == 0 {
                // Loop over to the last one
                row * SHOP_COLUMNS + selected_row_last_col
            } else {
                shop.selected_index - 1
            }
        }
        Right => {
            if col == selected_row_last_col {
                // Loop over to the first one
                row * SHOP_COLUMNS
            } else {
                shop.selected_index + 1
            }
        }
    };
}

fn buy(
    shop: &Shop,
    inventory: &mut Inventory,
    character: &Character,
    slots: &Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
) {
    let selected_slot = shop.get_selected_slot();
    let (_, _, selected_item, _) = slots.get(selected_slot).unwrap();
    let shop_item = selected_item.unwrap();

    let item = character.items.get(&shop_item.0).unwrap().clone();
    if inventory.can_buy(shop_item.0, &item) {
        inventory.buy(**shop_item, item)
    }
}

fn sell(
    shop: &Shop,
    inventory: &mut Inventory,
    character: &Character,
    slots: &Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
) {
    let selected_slot = shop.get_selected_slot();
    let (_, _, selected_item, _) = slots.get(selected_slot).unwrap();
    let shop_item = selected_item.unwrap();

    if inventory.contains(&shop_item.0) {
        inventory.sell(character, shop_item.0);
    }
}
