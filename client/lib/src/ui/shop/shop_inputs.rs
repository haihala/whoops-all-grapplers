use bevy::prelude::*;
use characters::{Character, Inventory};
use foundation::{Controllers, InputStream, MenuInput, Owner, Player};

use super::{setup_shop::ShopItem, shops_resource::Shop, Shops, SHOP_COLUMNS};

pub fn navigate_shop(
    mut players: Query<(&Player, &mut Inventory, &Character)>,
    slots: Query<(Entity, &Owner, Option<&ShopItem>)>,
    mut shops: ResMut<Shops>,
    input_stream: Res<InputStream>,
    controllers: Res<Controllers>,
) {
    let evs = input_stream.menu_events.clone();
    for (player, mut inventory, character) in &mut players {
        let shop = shops.get_mut_shop(player);

        if shop.closed {
            continue;
        }

        let input_device = controllers.get_handle(*player);

        for ev in &evs {
            if ev.player_handle != input_device {
                continue;
            }

            match ev.event {
                MenuInput::Up => move_selection(shop, Up),
                MenuInput::Down => move_selection(shop, Down),
                MenuInput::Left => move_selection(shop, Left),
                MenuInput::Right => move_selection(shop, Right),
                MenuInput::Accept => buy(shop, &mut inventory, character, &slots),
                MenuInput::Cancel => sell(shop, &mut inventory, character, &slots),
                MenuInput::Secondary => shop.closed = true,
            };
        }
    }
}

enum CardinalDiretion {
    Up,
    Down,
    Left,
    Right,
}

use CardinalDiretion::*;

fn move_selection(shop: &mut Shop, direction: CardinalDiretion) {
    let row = shop.selected_index / SHOP_COLUMNS;
    let col = shop.selected_index % SHOP_COLUMNS;

    let selected_row_last_col = if (row + 1) * SHOP_COLUMNS > shop.max_index {
        // Incomplete row
        shop.max_index - row * SHOP_COLUMNS
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
    slots: &Query<(Entity, &Owner, Option<&ShopItem>)>,
) {
    let selected_slot = shop.get_selected_slot();
    let (_, _, selected_item) = slots.get(selected_slot).unwrap();
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
    slots: &Query<(Entity, &Owner, Option<&ShopItem>)>,
) {
    let selected_slot = shop.get_selected_slot();
    let (_, _, selected_item) = slots.get(selected_slot).unwrap();
    let shop_item = selected_item.unwrap();

    if inventory.contains(&shop_item.0) {
        inventory.sell(character, shop_item.0);
    }
}
