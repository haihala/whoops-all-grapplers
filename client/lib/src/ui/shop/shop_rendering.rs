use bevy::prelude::*;
use characters::{Character, Inventory, ItemCategory};
use foundation::{
    Clock, Icons, MatchState, Owner, Player, Players, RoundLog, FPS, ITEM_SLOT_COMPONENT_COLOR,
    ITEM_SLOT_DEFAULT_COLOR, ITEM_SLOT_DISABLED_COLOR, ITEM_SLOT_HIGHLIGHT_COLOR,
    ITEM_SLOT_OWNED_COLOR, ITEM_SLOT_UPGRADE_COLOR, POST_SHOP_DURATION, PRE_ROUND_DURATION,
    SELL_RETURN,
};

use crate::{
    assets::{Announcer, Music},
    camera, player_state_management,
    state_transitions::TransitionTimer,
};

use super::{
    setup_shop::{OwnedText, ShopItem, ShopMoney, ShopScore, SuggestionStar},
    Shops,
};

pub fn update_top_bar_scores(scores: Single<&mut Text, With<ShopScore>>, results: Res<RoundLog>) {
    let mut text = scores.into_inner();
    text.0 = format!(
        "{} - {}",
        results.wins(Player::One),
        results.wins(Player::Two)
    );
}

pub fn update_top_bar_moneys(
    mut moneys: Query<(&mut Text, &Owner), With<ShopMoney>>,
    inventories: Query<&Inventory>,
    players: Res<Players>,
) {
    for (mut text, owner) in &mut moneys {
        let inv = inventories.get(players.get(owner.0)).unwrap();
        text.0 = format!("${}", inv.money);
    }
}

pub fn update_slot_visuals(
    player_query: Query<(&Inventory, &Character, &Player)>,
    item_query: Query<(
        &ShopItem,
        &Owner,
        Entity,
        &Children,
        &OwnedText,
        &SuggestionStar,
    )>,
    mut colors: Query<&mut BackgroundColor>,
    mut texts: Query<&mut Text>,
    mut visibilities: Query<&mut Visibility>,
    shops: Res<Shops>,
) {
    for (inventory, character, player) in &player_query {
        let shop = shops.get_shop(player);
        let selected_slot = shop.get_selected_slot();
        let selected_item_id = item_query
            .iter()
            .find_map(|(shop_item, _, e, _, _, _)| {
                if e == selected_slot {
                    Some(**shop_item)
                } else {
                    None
                }
            })
            .unwrap();
        let selected_item = character.items.get(&selected_item_id).unwrap();

        for (shop_item, owner, item_entity, children, owned_text, suggestion_star) in &item_query {
            if *player != owner.0 {
                continue;
            }

            let [mut wrapper_color, mut child_color] =
                colors.get_many_mut([item_entity, children[0]]).unwrap();

            let item_id = **shop_item;
            let item = character.items.get(&item_id).unwrap();

            *wrapper_color = if item_entity == selected_slot {
                ITEM_SLOT_HIGHLIGHT_COLOR
            } else {
                match (&selected_item.category, &item.category) {
                    (ItemCategory::Upgrade(components), _) if components.contains(&item_id) => {
                        ITEM_SLOT_COMPONENT_COLOR
                    }
                    (_, ItemCategory::Upgrade(ref components))
                        if components.contains(&selected_item_id) =>
                    {
                        ITEM_SLOT_UPGRADE_COLOR
                    }
                    _ => ITEM_SLOT_DEFAULT_COLOR,
                }
            }
            .into();

            let mut text = texts.get_mut(**owned_text).unwrap();
            let item_count = inventory.count(item_id);
            (*child_color, text.0) = if item_count != 0 {
                (
                    ITEM_SLOT_OWNED_COLOR.into(),
                    if item.max_stack == 1 {
                        "Owned".into()
                    } else {
                        format!("{item_count}/{}", item.max_stack)
                    },
                )
            } else {
                (
                    if inventory.can_buy(item_id, item) {
                        ITEM_SLOT_DEFAULT_COLOR.into()
                    } else {
                        ITEM_SLOT_DISABLED_COLOR.into()
                    },
                    "".into(),
                )
            };

            *visibilities.get_mut(**suggestion_star).unwrap() = if item.suggested {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_info_panel(
    slots: Query<&ShopItem>,
    mut visibilities: Query<(Entity, &mut Visibility)>,
    mut icon_query: Query<(&mut ImageNode, &SuggestionStar)>,
    mut writer: TextUiWriter,
    icons: Res<Icons>,
    shops: Res<Shops>,
    characters: Query<(&Character, &Inventory)>,
    players: Res<Players>,
) {
    for player in [Player::One, Player::Two] {
        let shop = shops.get_shop(&player);
        let active_slot = shop.get_selected_slot();
        let slot = slots.get(active_slot).unwrap();
        let item_id = slot.0;

        let (character, inventory) = characters.get(players.get(player)).unwrap();
        let item_name = item_id.display_name();
        let item = character.items.get(&item_id).unwrap();

        let mut allowed_ops = vec![];

        if inventory.has_space_for(item_id, item) {
            allowed_ops.push(("Buy", item.cost));
        }

        if inventory.contains(item_id) {
            allowed_ops.push((
                "Sell",
                (SELL_RETURN * (inventory.sell_price(character, item_id) as f32)) as usize,
            ));
        };

        let price_line = allowed_ops
            .into_iter()
            .map(|(op, amount)| format!("{op} for ${amount}"))
            .reduce(|a, b| format!("{a}, {b}"))
            .unwrap();

        // Update texts
        for (entity, section, content) in [
            (shop.components.item_name, 1, item_name),
            (shop.components.explanation, 1, item.explanation.to_owned()),
            (shop.components.cost, 1, price_line),
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
            let (entity, mut visibility) = visibilities.get_mut(entity).unwrap();
            if content.is_empty() {
                *visibility = Visibility::Hidden;
            } else {
                *writer.get_text(entity, section).unwrap() = content;
                *visibility = Visibility::Inherited;
            }
        }

        let (mut icon, star) = icon_query.get_mut(shop.components.big_icon).unwrap();
        icon.image = icons.0.get(&item.icon).unwrap().clone();
        let mut star_vis = visibilities.get_mut(**star).unwrap().1;
        *star_vis = if item.suggested {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_shop_ending(
    mut commands: Commands,
    mut shops: ResMut<Shops>,
    mut next_state: ResMut<NextState<MatchState>>,
    mut local_timer: Local<Option<usize>>,
    mut countdown_roots: Query<&mut Visibility>,
    mut countdown_texts: Query<&mut Text>,
    mut music: ResMut<Music>,
    mut announcer: ResMut<Announcer>,
    round_log: Res<RoundLog>,
    clock: Res<Clock>,
) {
    commands.run_system_cached(camera::reset_camera);
    let round_num = round_log.rounds_played() + 1;
    if shops.player_one.closed && shops.player_two.closed {
        end_shopping(
            &mut shops,
            &mut next_state,
            &mut commands,
            &mut countdown_roots,
            &mut music,
            &mut announcer,
            round_num,
            clock.frame,
        );
        *local_timer = None;
        return;
    }

    let mut end = false;
    for shop in [&shops.player_one, &shops.player_two] {
        if shop.closed {
            if let Some(transition_frame) = *local_timer {
                if transition_frame <= clock.frame {
                    end = true;
                    break;
                }

                let frames_left = (transition_frame - clock.frame) as f32;
                let secs_left = (frames_left / FPS).ceil() as usize;
                countdown_texts
                    .get_mut(shop.components.countdown_text)
                    .unwrap()
                    .0 = secs_left.to_string();
            } else {
                *local_timer = Some(clock.frame + (FPS * POST_SHOP_DURATION) as usize);
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
            &mut music,
            &mut announcer,
            round_num,
            clock.frame,
        );
        *local_timer = None;
    }

    // If one is closed and timer has ran out -> pre-combat
    // If one is closed and timer is not yet out -> make sure the counter is visible and up to date
}

#[allow(clippy::too_many_arguments)]
fn end_shopping(
    shops: &mut Shops,
    next_state: &mut ResMut<NextState<MatchState>>,
    commands: &mut Commands,
    countdown_roots: &mut Query<&mut Visibility>,
    music: &mut Music,
    announcer: &mut Announcer,
    round_num: usize,
    clock_frame: usize,
) {
    next_state.set(MatchState::PreRound);
    announcer.round_start(round_num);

    commands.insert_resource(TransitionTimer {
        frame: clock_frame + (FPS * PRE_ROUND_DURATION) as usize,
        state: MatchState::Combat,
    });

    music.pop();
    commands.run_system_cached(player_state_management::reset_combat);

    for shop in [&mut shops.player_one, &mut shops.player_two] {
        shop.closed = false;
        *countdown_roots.get_mut(shop.components.countdown).unwrap() = Visibility::Hidden;
    }
}
