use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use input_parsing::InputParser;
use wag_core::{ItemId, MoveId, Owner, Player, Players, INVENTORY_SIZE};

use crate::assets::{Colors, Fonts};

use super::{
    setup_shop::{render_item_icon, InventorySlot, ShopItem},
    shops_resource::Shop,
    Shops,
};

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
    mut parsers: Query<(&mut InputParser, &Player, &mut Inventory, &Character)>,
    mut slots: Query<(Entity, &Owner, Option<&ShopItem>, &mut ShopSlotState)>,
    mut shops: ResMut<Shops>,
) {
    for (mut parser, player, mut inventory, character) in &mut parsers {
        let events = parser.get_events();
        if events.is_empty() {
            continue;
        }

        let shop = shops.get_mut_shop(player);

        for event in events {
            match event {
                MoveId::Up => move_selection(shop, Up),
                MoveId::Down => move_selection(shop, Down),
                MoveId::Left => move_selection(shop, Left),
                MoveId::Right => move_selection(shop, Right),
                MoveId::Primary => primary_button_pressed(shop, &mut inventory, character, &slots),
                // MoveId::Secondary => todo!(),
                // MoveId::Back => todo!(),
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

const SELL_RETURN: f32 = 0.5;

fn sell(inventory: &mut Inventory, character: &Character, index: usize) {
    if let Some(id) = inventory.items.get(index) {
        let refund = ((get_recursive_cost(character, id) as f32) * SELL_RETURN) as usize;
        inventory.sell(index, refund);
    }
}

fn get_recursive_cost(character: &Character, id: &ItemId) -> usize {
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

pub fn update_slot_visuals(
    mut query: Query<(&ShopSlotState, &mut BackgroundColor), Changed<ShopSlotState>>,
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

pub fn update_inventory_ui(
    mut commands: Commands,
    inventories: Query<(&Inventory, &Player), Changed<Inventory>>,
    mut money_texts: Query<&mut Text>,
    mut slots: Query<(Entity, &mut InventorySlot, &Owner)>,
    fonts: Res<Fonts>,
    colors: Res<Colors>,
    shops: Res<Shops>,
) {
    for (inventory, player) in &inventories {
        // Update money text
        let shop = shops.get_shop(player);
        let mut text = money_texts.get_mut(shop.components.money_text).unwrap();
        text.sections[1].value = inventory.money.to_string();

        // Update slots
        for (entity, mut slot, owner) in &mut slots {
            if **owner != *player {
                continue;
            }
            if let Some(id) = inventory.items.get(slot.index) {
                let old_item = slot.id;
                let different_item = old_item.is_some() && old_item.unwrap() != *id;

                if different_item {
                    commands.entity(entity).despawn_descendants();
                }

                if old_item.is_none() || different_item {
                    commands.entity(entity).add_children(|root| {
                        render_item_icon(
                            root,
                            TextStyle {
                                font: fonts.basic.clone(),
                                font_size: 36.0,
                                color: colors.text,
                            },
                            *id,
                        );
                    });

                    slot.id = Some(*id);
                }
            } else {
                // Slot is empty
                commands.entity(entity).despawn_descendants();
                slot.id = None;
            }
        }
    }
}

pub fn update_info_panel(
    slots: Query<(Option<&ShopItem>, Option<&InventorySlot>)>,
    mut texts: Query<(&mut Text, &mut Visibility)>,
    shops: Res<Shops>,
    characters: Query<&Character>,
    players: Res<Players>,
) {
    for player in [Player::One, Player::Two] {
        let shop = shops.get_shop(&player);
        let active_slot = shop.get_selected_slot();
        let (maybe_available, maybe_inventory_slot) = slots.get(active_slot).unwrap();

        let character = characters.get(players.get(player)).unwrap();
        let contents = if let Some(available) = maybe_available {
            available_item_info(character, available)
        } else if let Some(inventory_slot) = maybe_inventory_slot {
            inventory_slot_info(character, inventory_slot)
        } else {
            panic!("Selected slot isn't an inventory slot nor is it a purchasable item");
        };

        // Update actual texts
        for (entity, section, maybe_content) in [
            (shop.components.item_name, 0, contents.item_name),
            (shop.components.explanation, 0, contents.explanation),
            (shop.components.cost, 0, contents.operation),
            (shop.components.cost, 2, contents.cost),
            (shop.components.dependencies, 1, contents.dependencies),
        ] {
            let (mut text, mut visibility) = texts.get_mut(entity).unwrap();
            if let Some(content) = maybe_content {
                text.sections[section].value = content;
                visibility.is_visible = true;
            } else {
                visibility.is_visible = false;
            }
        }
    }
}

#[derive(Debug, Default)]
struct InfoPanelContents {
    // pub big_icon: ,  // TODO: Icons
    pub item_name: Option<String>,
    pub explanation: Option<String>,
    pub operation: Option<String>,
    pub cost: Option<String>,
    pub dependencies: Option<String>,
}

fn available_item_info(character: &Character, shop_item: &ShopItem) -> InfoPanelContents {
    let item = character.items.get(shop_item).unwrap().clone();

    InfoPanelContents {
        item_name: Some(shop_item.display_name()),
        explanation: if !item.explanation.is_empty() {
            Some(item.explanation)
        } else {
            None
        },
        operation: Some("Buy".into()),
        cost: Some(item.cost.to_string()),
        dependencies: if let ItemCategory::Upgrade(deps) = item.category {
            Some(
                deps.into_iter()
                    .map(|id| id.display_name())
                    .intersperse(", ".to_string())
                    .collect(),
            )
        } else {
            None
        },
    }
}

fn inventory_slot_info(character: &Character, inventory_slot: &InventorySlot) -> InfoPanelContents {
    if let Some(item_id) = inventory_slot.id {
        let item = character.items.get(&item_id).unwrap().clone();

        InfoPanelContents {
            item_name: Some(item_id.display_name()),
            explanation: Some(item.explanation),
            operation: Some("Sell".into()),
            cost: Some(
                (((get_recursive_cost(character, &item_id) as f32) * SELL_RETURN) as usize)
                    .to_string(),
            ),
            dependencies: if let ItemCategory::Upgrade(deps) = item.category {
                Some(
                    deps.into_iter()
                        .map(|id| id.display_name())
                        .intersperse(", ".to_string())
                        .collect(),
                )
            } else {
                None
            },
        }
    } else {
        InfoPanelContents {
            item_name: Some("Empty inventory slot".into()),
            ..default()
        }
    }
}
