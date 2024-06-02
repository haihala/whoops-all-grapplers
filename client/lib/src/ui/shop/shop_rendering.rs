use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use wag_core::{
    GameState, Player, Players, ITEM_SLOT_DEFAULT_COLOR, ITEM_SLOT_DISABLED_COLOR,
    ITEM_SLOT_HIGHLIGHT_COLOR, POST_SHOP_DURATION, PRE_ROUND_DURATION,
};

use crate::state_transitions::TransitionTimer;

use super::{setup_shop::ShopItem, shop_inputs::ShopSlotState, Shops};

pub fn update_slot_visuals(
    mut query: Query<(&ShopSlotState, &mut BackgroundColor), Changed<ShopSlotState>>,
) {
    for (state, mut color) in &mut query {
        *color = match state {
            ShopSlotState::Default => ITEM_SLOT_DEFAULT_COLOR,
            ShopSlotState::Highlighted => ITEM_SLOT_HIGHLIGHT_COLOR,
            ShopSlotState::Disabled => ITEM_SLOT_DISABLED_COLOR,
        }
        .into()
    }
}

pub fn update_info_panel(
    slots: Query<&ShopItem>,
    mut texts: Query<(&mut Text, &mut Visibility)>,
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
