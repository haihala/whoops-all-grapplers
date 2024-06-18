use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use wag_core::{
    GameState, Owner, Player, Players, RoundLog, ITEM_SLOT_COMPONENT_COLOR,
    ITEM_SLOT_DEFAULT_COLOR, ITEM_SLOT_DISABLED_COLOR, ITEM_SLOT_HIGHLIGHT_COLOR,
    ITEM_SLOT_OWNED_COLOR, ITEM_SLOT_UPGRADE_COLOR, POST_SHOP_DURATION, PRE_ROUND_DURATION,
};

use crate::{assets::Icons, state_transitions::TransitionTimer};

use super::{
    setup_shop::{ShopItem, ShopMoney, ShopScore},
    Shops,
};

pub fn update_top_bar_scores(
    mut scores: Query<&mut Text, With<ShopScore>>,
    results: Res<RoundLog>,
) {
    let mut text = scores.get_single_mut().unwrap();
    text.sections[0].value = results.wins(Player::One).to_string();
    text.sections[2].value = results.wins(Player::Two).to_string();
}

pub fn update_top_bar_moneys(
    mut moneys: Query<(&mut Text, &Owner), With<ShopMoney>>,
    inventories: Query<&Inventory>,
    players: Res<Players>,
) {
    for (mut text, owner) in &mut moneys {
        let inv = inventories.get(players.get(owner.0)).unwrap();
        text.sections[1].value = inv.money.to_string();
    }
}

pub fn update_slot_visuals(
    player_query: Query<(&Inventory, &Character, &Player)>,
    mut item_query: Query<(&ShopItem, &Owner, Entity, &mut BackgroundColor)>,
    shops: Res<Shops>,
) {
    for (inventory, character, player) in &player_query {
        let shop = shops.get_shop(player);
        let selected_slot = shop.get_selected_slot();
        let selected_item_id = item_query
            .iter()
            .find(|(_, _, e, _)| *e == selected_slot)
            .map(|(shop_item, _, _, _)| **shop_item)
            .unwrap();
        let selected_item = character.items.get(&selected_item_id).unwrap();

        for (shop_item, owner, item_entity, mut color) in &mut item_query {
            if *player != owner.0 {
                continue;
            }

            if item_entity == selected_slot {
                *color = ITEM_SLOT_HIGHLIGHT_COLOR.into();
                continue;
            }

            let item_id = **shop_item;

            if let ItemCategory::Upgrade(ref components) = selected_item.category {
                if components.contains(&item_id) {
                    *color = ITEM_SLOT_COMPONENT_COLOR.into();
                    continue;
                }
            }

            let item = character.items.get(&item_id).unwrap();

            if let ItemCategory::Upgrade(ref components) = item.category {
                if components.contains(&selected_item_id) {
                    *color = ITEM_SLOT_UPGRADE_COLOR.into();
                    continue;
                }
            }

            if inventory.contains(&item_id) {
                *color = ITEM_SLOT_OWNED_COLOR.into();
                continue;
            }

            *color = if inventory.can_buy(item_id, item) {
                ITEM_SLOT_DEFAULT_COLOR
            } else {
                ITEM_SLOT_DISABLED_COLOR
            }
            .into();
        }
    }
}

pub fn update_info_panel(
    slots: Query<&ShopItem>,
    mut texts: Query<(&mut Text, &mut Visibility)>,
    mut icon_query: Query<&mut UiImage>,
    icons: Res<Icons>,
    shops: Res<Shops>,
    characters: Query<(&Character, &Inventory)>,
    players: Res<Players>,
) {
    for player in [Player::One, Player::Two] {
        let shop = shops.get_shop(&player);
        let active_slot = shop.get_selected_slot();
        let slot = slots.get(active_slot).unwrap();

        let (character, inventory) = characters.get(players.get(player)).unwrap();
        let item_name = slot.0.display_name();
        let item = character.items.get(&slot.0).unwrap();

        let (verb, cost) = if inventory.contains(&slot.0) {
            ("Sell", inventory.sell_price(character, slot.0))
        } else {
            // TODO: Recursive buy
            ("Buy", item.cost)
        };

        // Update texts
        for (entity, section, content) in [
            (shop.components.item_name, 0, item_name),
            (shop.components.explanation, 0, item.explanation.to_owned()),
            (shop.components.cost, 0, verb.to_string()),
            (shop.components.cost, 2, cost.to_string()),
            (
                shop.components.dependencies,
                1,
                if let ItemCategory::Upgrade(deps) = &item.category {
                    deps.iter()
                        .map(|id| id.display_name())
                        .intersperse(", ".to_string())
                        .collect()
                } else {
                    "".to_string()
                },
            ),
        ] {
            let (mut text, mut visibility) = texts.get_mut(entity).unwrap();
            if content.is_empty() {
                *visibility = Visibility::Hidden;
            } else {
                text.sections[section].value = content;
                *visibility = Visibility::Inherited;
            }
        }

        let mut icon = icon_query.get_mut(shop.components.big_icon).unwrap();
        icon.texture = icons.0.get(&item.icon).unwrap().clone();
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
