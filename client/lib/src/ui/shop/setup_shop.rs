use bevy::prelude::*;

use characters::{Character, Item, ItemCategory::*};
use wag_core::{GameState, ItemId, OnlyShowInGameState, Owner, Player, Players, INVENTORY_SIZE};

use crate::assets::{Colors, Fonts};

use super::shop_usage::{ShopNavigation, ShopSlotState};
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
    colors: Res<Colors>,
) {
    let mut player_one = None;
    let mut player_two = None;

    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    size: Size {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                    },
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            OnlyShowInGameState(vec![GameState::Shop]),
            Name::new("Shop ui container"),
        ))
        .with_children(|child_builder| {
            player_one = Some(setup_shop_root(
                child_builder,
                Player::One,
                characters.get(players.one).unwrap(),
                &colors,
                &fonts,
            ));

            player_two = Some(setup_shop_root(
                child_builder,
                Player::Two,
                characters.get(players.two).unwrap(),
                &colors,
                &fonts,
            ));
        });

    commands.insert_resource(Shops {
        player_one: Shop {
            components: player_one.unwrap(),
            navigation: ShopNavigation::default(),
        },
        player_two: Shop {
            components: player_two.unwrap(),
            navigation: ShopNavigation::default(),
        },
    });
}

fn setup_shop_root(
    root: &mut ChildBuilder,
    owner: Player,
    character: &Character,
    colors: &Colors,
    fonts: &Fonts,
) -> ShopComponents {
    let padding = match owner {
        Player::One => UiRect {
            right: Val::Px(5.0),
            ..default()
        },
        Player::Two => UiRect {
            left: Val::Px(5.0),
            ..default()
        },
    };
    let mut shop_root_builder = ShopComponentsBuilder::default();

    root.spawn((
        NodeBundle {
            background_color: Color::GRAY.into(),
            style: Style {
                size: Size {
                    height: Val::Percent(100.0),
                    width: Val::Percent(50.0),
                },
                flex_direction: FlexDirection::Column,
                padding,
                ..default()
            },
            ..default()
        },
        Name::new(format!("Player {} shop root", &owner)),
    ))
    .with_children(|shop_root| {
        setup_info_panel(shop_root, &mut shop_root_builder, colors, fonts);
        setup_inventory(shop_root, &mut shop_root_builder, colors, fonts, owner);
        setup_available_items(
            shop_root,
            &mut shop_root_builder,
            colors,
            fonts,
            character,
            owner,
        );
    });

    shop_root_builder.build()
}

fn setup_info_panel(
    root: &mut ChildBuilder,
    shop_root: &mut ShopComponentsBuilder,
    colors: &Colors,
    fonts: &Fonts,
) {
    let absolute_margin = 3.0;
    let margin = UiRect::all(Val::Px(absolute_margin));
    let icon_size = 200.0;

    root.spawn((
        NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                size: Size {
                    height: Val::Px(icon_size + absolute_margin * 4.0), // Top and bottom, margin and padding
                    width: Val::Auto,
                },
                margin,
                padding: margin,
                ..default()
            },
            ..default()
        },
        Name::new("Info panel"),
    ))
    .with_children(|info_panel| {
        shop_root.big_icon = Some(big_icon(info_panel, icon_size));
        setup_explanation_box(info_panel, shop_root, colors, fonts);
    });
}

fn big_icon(root: &mut ChildBuilder, size: f32) -> Entity {
    root.spawn((
        ImageBundle {
            style: Style {
                size: Size {
                    height: Val::Percent(100.0),
                    width: Val::Px(size),
                },
                flex_shrink: 0.0,
                ..default()
            },
            ..default()
        },
        Name::new("Big icon"),
    ))
    .id()
}

fn setup_explanation_box(
    root: &mut ChildBuilder,
    shop_root: &mut ShopComponentsBuilder,
    colors: &Colors,
    fonts: &Fonts,
) {
    root.spawn((
        NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    ..default()
                },
                size: Size::AUTO,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("Explanations"),
    ))
    .with_children(|explanation_box| {
        let basic_style = TextStyle {
            font: fonts.basic.clone(),
            font_size: 12.0,
            color: colors.text,
        };

        shop_root.item_name = Some(setup_text_sections(
            explanation_box,
            vec!["Item name"],
            TextStyle {
                font_size: 24.0,
                ..basic_style.clone()
            },
            "Item name",
        ));

        shop_root.explanation = Some(setup_text_sections(
            explanation_box,
            vec!["Body"],
            basic_style.clone(),
            "Explanation",
        ));

        shop_root.cost = Some(setup_text_sections(
            explanation_box,
            vec!["Sell", " for: $", "0"],
            basic_style.clone(),
            "Costs",
        ));

        shop_root.dependencies = Some(setup_text_sections(
            explanation_box,
            vec!["Depends on: ", ""],
            basic_style,
            "Dependencies",
        ));
    });
}

fn setup_text_sections(
    root: &mut ChildBuilder,
    texts: Vec<impl Into<String>>,
    style: TextStyle,
    name: impl Into<String>,
) -> Entity {
    root.spawn((
        TextBundle::from_sections(
            texts
                .into_iter()
                .map(|text| TextSection::new(text, style.clone())),
        ),
        Name::new(name.into()),
    ))
    .id()
}

fn setup_inventory(
    root: &mut ChildBuilder,
    shop_root: &mut ShopComponentsBuilder,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
) {
    let margin = UiRect::all(Val::Px(3.0));

    root.spawn((
        NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                size: Size::AUTO,
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
    .with_children(|inventory_container| {
        setup_owned_slots(inventory_container, shop_root, player);
        shop_root.money_text = Some(setup_text_sections(
            inventory_container,
            vec!["$", "0"],
            TextStyle {
                font: fonts.basic.clone(),
                font_size: 24.0,
                color: colors.text,
            },
            "Money",
        ));
    });
}

fn setup_owned_slots(
    root: &mut ChildBuilder,
    shop_root: &mut ShopComponentsBuilder,
    player: Player,
) {
    let margin = UiRect::all(Val::Px(3.0));

    root.spawn((
        NodeBundle {
            style: Style {
                size: Size::AUTO,
                margin,
                ..default()
            },
            ..default()
        },
        Name::new("Owned slots"),
    ))
    .with_children(|owned_container| {
        for i in 0..INVENTORY_SIZE {
            shop_root
                .owned_slots
                .push(create_empty_inventory_slot(owned_container, player, i));
        }
    });
}

fn create_empty_inventory_slot(root: &mut ChildBuilder, player: Player, index: usize) -> Entity {
    root.spawn((
        NodeBundle {
            background_color: Color::GRAY.into(),
            style: Style {
                size: Size {
                    height: Val::Px(50.0),
                    width: Val::Px(50.0),
                },
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
    .id()
}

fn setup_available_items(
    root: &mut ChildBuilder,
    shop_root: &mut ShopComponentsBuilder,
    colors: &Colors,
    fonts: &Fonts,
    character: &Character,
    player: Player,
) {
    let items = get_prepared_items(character);

    root.spawn((
        NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                size: Size::AUTO,
                justify_content: JustifyContent::SpaceBetween,
                flex_grow: 1.0,
                margin: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Available items root"),
    ))
    .with_children(|available_container| {
        shop_root.consumables = setup_category(
            available_container,
            colors,
            fonts,
            player,
            "Consumables".into(),
            items.consumables,
        );

        shop_root.basics = setup_category(
            available_container,
            colors,
            fonts,
            player,
            "Basics".into(),
            items.basics,
        );

        shop_root.upgrades = setup_category(
            available_container,
            colors,
            fonts,
            player,
            "Upgrades".into(),
            items.upgrades,
        );
    });
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

    items.sort_by(|(_, item1), (_, item2)| item1.cost.cmp(&item2.cost));

    for (id, item) in items {
        match item.category {
            Consumable => consumables.push(id),
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
    root: &mut ChildBuilder,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
    title: String,
    items: Vec<ItemId>,
) -> Vec<Entity> {
    let mut item_entities = vec![];

    root.spawn((
        NodeBundle {
            background_color: Color::GRAY.into(),
            style: Style {
                size: Size::AUTO,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(3.0)),
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        },
        Name::new(format!("Available {} root", &title)),
    ))
    .with_children(|category_root| {
        category_root.spawn((
            TextBundle::from_section(
                title,
                TextStyle {
                    font: fonts.basic.clone(),
                    font_size: 18.0,
                    color: colors.text,
                },
            )
            .with_style(Style {
                size: Size {
                    height: Val::Px(30.0),
                    width: Val::Auto,
                },
                ..default()
            }),
            BackgroundColor(Color::DARK_GRAY),
        ));

        for id in items.into_iter() {
            item_entities.push(setup_shop_item(category_root, colors, fonts, player, id));
        }
    });

    item_entities
}

fn setup_shop_item(
    root: &mut ChildBuilder,
    colors: &Colors,
    fonts: &Fonts,
    player: Player,
    id: ItemId,
) -> Entity {
    let margin = UiRect::all(Val::Px(3.0));

    root.spawn((
        NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                size: Size::AUTO,
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
    .with_children(|item_root| {
        let base_style = TextStyle {
            font: fonts.basic.clone(),
            font_size: 16.0,
            color: colors.text,
        };

        // Icon
        render_item_icon(
            item_root,
            TextStyle {
                font_size: 24.0,
                ..base_style.clone()
            },
            id,
        );

        // Name
        item_root.spawn(
            TextBundle::from_section(id.display_name(), base_style).with_style(Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    ..default()
                },
                overflow: Overflow::Hidden,
                ..default()
            }),
        );
    })
    .id()
}

pub fn render_item_icon(root: &mut ChildBuilder, style: TextStyle, id: ItemId) {
    root.spawn((
        TextBundle::from_section(id.display_name().chars().next().unwrap(), style),
        BackgroundColor(Color::GRAY),
    ));
}
