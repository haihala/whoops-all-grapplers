use bevy::prelude::*;

use characters::{Character, Item, ItemCategory::*};
use wag_core::{
    GameState, ItemId, OnlyShowInGameState, Owner, Player, Players, GENERIC_TEXT_COLOR,
    INVENTORY_SIZE, SHOP_DARK_BACKGROUND_COLOR, SHOP_DIVIDER_COLOR, SHOP_LIGHT_BACKGROUND_COLOR,
    SHOP_TIMER_BACKGROUND_COLOR,
};

use crate::assets::Fonts;

use super::shop_inputs::{ShopNavigation, ShopSlotState};
use super::shops_resource::{Shop, ShopComponents, ShopComponentsBuilder, Shops};

#[derive(Debug, Component, Deref)]
pub struct ShopItem(pub ItemId);

#[derive(Debug, Component)]
pub struct InventorySlot {
    pub index: usize,
    pub id: Option<ItemId>,
}

pub fn setup_shop(
    mut commands: Commands,
    characters: Query<&Character>,
    players: Res<Players>,
    fonts: Res<Fonts>,
) {
    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DIVIDER_COLOR.into(), // This will color the divider between the sides
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            OnlyShowInGameState(vec![GameState::Shop]),
            Name::new("Shop ui container"),
        ))
        .id();

    let player_one_components = setup_shop_root(
        &mut commands,
        container,
        Player::One,
        characters.get(players.one).unwrap(),
        &fonts,
    );

    let player_two_components = setup_shop_root(
        &mut commands,
        container,
        Player::Two,
        characters.get(players.two).unwrap(),
        &fonts,
    );

    commands.insert_resource(Shops {
        player_one: Shop {
            components: player_one_components,
            navigation: ShopNavigation::default(),
            closed: false,
        },
        player_two: Shop {
            components: player_two_components,
            navigation: ShopNavigation::default(),
            closed: false,
        },
    });
}

fn setup_shop_root(
    commands: &mut Commands,
    parent: Entity,
    owner: Player,
    character: &Character,
    fonts: &Fonts,
) -> ShopComponents {
    let mut shop_root_builder = ShopComponentsBuilder::default();

    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DIVIDER_COLOR.into(),
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(49.9), // Not quite 50 so there is a gap between them
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Player {} shop root", &owner)),
        ))
        .set_parent(parent)
        .id();

    setup_info_panel(commands, container, &mut shop_root_builder, fonts);
    setup_inventory(commands, container, &mut shop_root_builder, fonts, owner);
    setup_available_items(
        commands,
        container,
        &mut shop_root_builder,
        fonts,
        character,
        owner,
    );
    setup_countdown_number(commands, container, &mut shop_root_builder, fonts);

    shop_root_builder.build()
}

fn setup_countdown_number(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    fonts: &Fonts,
) {
    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_TIMER_BACKGROUND_COLOR.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("Countdown"),
        ))
        .set_parent(parent)
        .id();

    shop_root.countdown = Some(container);
    shop_root.countdown_text = Some(
        commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "10",
                    TextStyle {
                        color: GENERIC_TEXT_COLOR,
                        font: fonts.basic.clone(),
                        font_size: 256.0,
                    },
                ),
                ..default()
            })
            .set_parent(container)
            .id(),
    );
}

fn setup_info_panel(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    fonts: &Fonts,
) {
    let absolute_margin = 3.0;
    let margin = UiRect::all(Val::Px(absolute_margin));
    let icon_size = 200.0;

    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    height: Val::Px(icon_size + absolute_margin * 4.0), // Top and bottom, margin and padding
                    width: Val::Auto,
                    margin,
                    padding: margin,
                    ..default()
                },
                ..default()
            },
            Name::new("Info panel"),
        ))
        .set_parent(parent)
        .id();

    shop_root.big_icon = Some(big_icon(commands, container, icon_size));
    setup_explanation_box(commands, container, shop_root, fonts);
}

fn big_icon(commands: &mut Commands, parent: Entity, size: f32) -> Entity {
    commands
        .spawn((
            ImageBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Px(size),
                    flex_shrink: 0.0,
                    ..default()
                },
                ..default()
            },
            Name::new("Big icon"),
        ))
        .set_parent(parent)
        .id()
}

fn setup_explanation_box(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    fonts: &Fonts,
) {
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(10.0),
                        ..default()
                    },
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Name::new("Explanations"),
        ))
        .set_parent(parent)
        .id();

    let basic_style = TextStyle {
        font: fonts.basic.clone(),
        font_size: 12.0,
        color: GENERIC_TEXT_COLOR,
    };

    shop_root.item_name = Some(setup_text_sections(
        commands,
        container,
        vec!["Item name"],
        TextStyle {
            font_size: 24.0,
            ..basic_style.clone()
        },
        "Item name",
    ));

    shop_root.explanation = Some(setup_text_sections(
        commands,
        container,
        vec!["Body"],
        basic_style.clone(),
        "Explanation",
    ));

    shop_root.cost = Some(setup_text_sections(
        commands,
        container,
        vec!["Sell", " for: $", "0"],
        basic_style.clone(),
        "Costs",
    ));

    shop_root.dependencies = Some(setup_text_sections(
        commands,
        container,
        vec!["Depends on: ", " "],
        basic_style,
        "Dependencies",
    ));
}

fn setup_text_sections(
    commands: &mut Commands,
    parent: Entity,
    texts: Vec<impl Into<String>>,
    style: TextStyle,
    name: impl Into<String>,
) -> Entity {
    commands
        .spawn((
            TextBundle::from_sections(
                texts
                    .into_iter()
                    .map(|text| TextSection::new(text, style.clone())),
            ),
            Name::new(name.into()),
        ))
        .set_parent(parent)
        .id()
}

fn setup_inventory(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    fonts: &Fonts,
    player: Player,
) {
    let margin = UiRect::all(Val::Px(3.0));

    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    // size: Size::AUTO,
                    align_items: AlignItems::Center,
                    margin,
                    padding: margin,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Name::new("Inventory root"),
        ))
        .set_parent(parent)
        .id();

    setup_owned_slots(commands, container, shop_root, player);
    shop_root.money_text = Some(setup_text_sections(
        commands,
        container,
        vec!["$", "0"],
        TextStyle {
            font: fonts.basic.clone(),
            font_size: 24.0,
            color: GENERIC_TEXT_COLOR,
        },
        "Money",
    ));
}

fn setup_owned_slots(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    player: Player,
) {
    let margin = UiRect::all(Val::Px(3.0));

    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // size: Size::AUTO,
                    margin,
                    ..default()
                },
                ..default()
            },
            Name::new("Owned slots"),
        ))
        .set_parent(parent)
        .id();

    for i in 0..INVENTORY_SIZE {
        shop_root
            .owned_slots
            .push(create_empty_inventory_slot(commands, container, player, i));
    }
}

fn create_empty_inventory_slot(
    commands: &mut Commands,
    parent: Entity,
    player: Player,
    index: usize,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                background_color: SHOP_LIGHT_BACKGROUND_COLOR.into(),
                style: Style {
                    height: Val::Px(50.0),
                    width: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                ..default()
            },
            if index == 0 {
                ShopSlotState::Highlighted
            } else {
                ShopSlotState::Default
            },
            Owner(player),
            InventorySlot { index, id: None },
            Name::new(format!("Inventory slot {}", index)),
        ))
        .set_parent(parent)
        .id()
}

fn setup_available_items(
    commands: &mut Commands,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    fonts: &Fonts,
    character: &Character,
    player: Player,
) {
    let items = get_prepared_items(character);

    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // size: Size::AUTO,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            },
            Name::new("Available items root"),
        ))
        .set_parent(parent)
        .id();

    shop_root.consumables = setup_category(
        commands,
        container,
        fonts,
        player,
        "Consumables".into(),
        items.consumables,
    );

    shop_root.basics = setup_category(
        commands,
        container,
        fonts,
        player,
        "Basics".into(),
        items.basics,
    );

    shop_root.upgrades = setup_category(
        commands,
        container,
        fonts,
        player,
        "Upgrades".into(),
        items.upgrades,
    );
}

#[derive(Debug)]
struct PreparedItems {
    consumables: Vec<ItemId>,
    basics: Vec<ItemId>,
    upgrades: Vec<ItemId>,
}

fn get_prepared_items(character: &Character) -> PreparedItems {
    let mut consumables = vec![];
    let mut basics = vec![];
    let mut upgrades = vec![];

    let mut items: Vec<(ItemId, Item)> = character
        .items
        .iter()
        .map(|(id, item)| (id.to_owned(), item.to_owned()))
        .collect();

    items.sort_by_key(|(id, item)| (item.cost, id.to_owned()));

    for (id, item) in items {
        match item.category {
            Consumable(_) => consumables.push(id),
            Basic => basics.push(id),
            Upgrade(_) => upgrades.push(id),
        }
    }

    PreparedItems {
        consumables,
        basics,
        upgrades,
    }
}

fn setup_category(
    commands: &mut Commands,
    parent: Entity,
    fonts: &Fonts,
    player: Player,
    title: String,
    items: Vec<ItemId>,
) -> Vec<Entity> {
    let mut item_entities = vec![];

    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    // size: Size::AUTO,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(3.0)),
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Available {} root", &title)),
        ))
        .set_parent(parent)
        .id();

    commands
        .spawn(TextBundle {
            background_color: SHOP_DARK_BACKGROUND_COLOR.into(),

            ..TextBundle::from_section(
                title,
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 18.0,
                    color: GENERIC_TEXT_COLOR,
                },
            )
            .with_style(Style {
                height: Val::Px(30.0),
                width: Val::Auto,
                margin: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            })
        })
        .set_parent(container);

    for id in items.into_iter() {
        item_entities.push(setup_shop_item(commands, container, fonts, player, id));
    }

    item_entities
}

fn setup_shop_item(
    commands: &mut Commands,
    parent: Entity,
    fonts: &Fonts,
    player: Player,
    id: ItemId,
) -> Entity {
    let margin = UiRect::all(Val::Px(3.0));

    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // size: Size::AUTO,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    margin,
                    padding: margin,
                    ..default()
                },
                ..default()
            },
            ShopSlotState::Default,
            ShopItem(id),
            Owner(player),
        ))
        .set_parent(parent)
        .id();

    let base_style = TextStyle {
        font: fonts.basic.clone(),
        font_size: 16.0,
        color: GENERIC_TEXT_COLOR,
    };

    // Icon
    render_item_icon(
        commands,
        container,
        TextStyle {
            font_size: 24.0,
            ..base_style.clone()
        },
        id,
    );

    // Name
    commands
        .spawn(
            TextBundle::from_section(id.display_name(), base_style).with_style(Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    ..default()
                },
                overflow: Overflow::clip(),
                ..default()
            }),
        )
        .set_parent(container);

    container
}

pub fn render_item_icon(
    commands: &mut Commands,
    parent: Entity,
    text_style: TextStyle,
    id: ItemId,
) {
    // Tried for a while, but couldn't figure out a way to get the text box to be wider than it needed to be
    let text = format!(" {} ", id.display_name().chars().next().unwrap());

    commands
        .spawn(TextBundle {
            background_color: SHOP_LIGHT_BACKGROUND_COLOR.into(),
            ..TextBundle::from_section(text, text_style)
        })
        .set_parent(parent);
}
