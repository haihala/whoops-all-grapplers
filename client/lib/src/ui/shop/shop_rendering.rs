use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use wag_core::{
    GameState, Owner, Player, Players, POST_SHOP_DURATION, PRE_ROUND_DURATION, SELL_RETURN,
};

use crate::{
    assets::{Colors, Fonts},
    state_transitions::TransitionTimer,
};

use super::{
    setup_shop::{render_item_icon, InventorySlot, ShopItem},
    shop_inputs::{get_recursive_cost, ShopSlotState},
    Shops,
};

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
                    render_item_icon(
                        &mut commands,
                        entity,
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 36.0,
                            color: colors.text,
                        },
                        *id,
                    );

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
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
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

pub fn handle_shop_ending(
    mut commands: Commands,
    mut shops: ResMut<Shops>,
    mut next_state: ResMut<NextState<GameState>>,
    mut local_timer: Local<Option<Timer>>,
    mut countdown_roots: Query<&mut Visibility>,
    mut countdown_texts: Query<&mut Text>,
    time: Res<Time>,
) {
    if shops.player_one.closed && shops.player_two.closed {
        end_shopping(
            &mut shops,
            &mut next_state,
            &mut commands,
            &mut countdown_roots,
        );
        *local_timer = None;
        return;
    }

    let mut end = false;
    for shop in [&shops.player_one, &shops.player_two] {
        if shop.closed {
            if let Some(timer) = &mut *local_timer {
                timer.tick(time.delta());
                end = timer.finished();

                // Update text
                let value =
                    ((POST_SHOP_DURATION - timer.elapsed_secs()).floor() as i32).to_string();
                countdown_texts
                    .get_mut(shop.components.countdown_text)
                    .unwrap()
                    .sections[0]
                    .value = value;
            } else {
                *local_timer = Some(Timer::from_seconds(POST_SHOP_DURATION, TimerMode::Once));
                *countdown_roots.get_mut(shop.components.countdown).unwrap() =
                    Visibility::Inherited;
            }
        }
    }

    if end {
        end_shopping(
            &mut shops,
            &mut next_state,
            &mut commands,
            &mut countdown_roots,
        );
        *local_timer = None;
    }

    // If one is closed and timer has ran out -> pre-combat
    // If one is closed and timer is not yet out -> make sure the counter is visible and up to date
}

fn end_shopping(
    shops: &mut Shops,
    next_state: &mut ResMut<NextState<GameState>>,
    commands: &mut Commands,
    countdown_roots: &mut Query<&mut Visibility>,
) {
    next_state.set(GameState::PreRound);

    commands.insert_resource(TransitionTimer::from(Timer::from_seconds(
        PRE_ROUND_DURATION,
        TimerMode::Once,
    )));

    for shop in [&mut shops.player_one, &mut shops.player_two] {
        shop.closed = false;
        *countdown_roots.get_mut(shop.components.countdown).unwrap() = Visibility::Hidden;
    }
}
