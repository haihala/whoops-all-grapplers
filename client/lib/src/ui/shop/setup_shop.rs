use bevy::prelude::*;

use characters::Character;
use wag_core::{
    Icon, InMatch, ItemId, MatchState, Owner, Player, Players, GENERIC_TEXT_COLOR,
    ITEM_SLOT_COMPONENT_COLOR, ITEM_SLOT_DEFAULT_COLOR, ITEM_SLOT_DISABLED_COLOR,
    ITEM_SLOT_HIGHLIGHT_COLOR, ITEM_SLOT_OWNED_COLOR, ITEM_SLOT_UPGRADE_COLOR,
    SHOP_DARK_BACKGROUND_COLOR, SHOP_DIVIDER_COLOR, SHOP_TIMER_BACKGROUND_COLOR,
};

use crate::assets::{Fonts, Icons};
use crate::entity_management::VisibleInStates;

use super::shops_resource::{Shop, ShopComponents, ShopComponentsBuilder, Shops};
use super::SHOP_COLUMNS;

#[derive(Debug, Component, Deref, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ShopItem(pub ItemId);

pub fn setup_shop(
    mut commands: Commands,
    characters: Query<&Character>,
    players: Res<Players>,
    fonts: Res<Fonts>,
    icons: Res<Icons>,
) {
    let root = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DIVIDER_COLOR.into(), // This will color the divider between the sides
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(0.5),
                    ..default()
                },
                ..default()
            },
            VisibleInStates(vec![MatchState::Shop]),
            StateScoped(InMatch),
            Name::new("Shop ui root"),
        ))
        .id();

    setup_shop_top_bar(&mut commands, root);

    let container = commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Percent(0.5),
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        })
        .set_parent(root)
        .id();

    let player_one_components = setup_shop_root(
        &mut commands,
        container,
        Player::One,
        characters.get(players.one).unwrap(),
        &icons,
        &fonts,
    );

    let player_two_components = setup_shop_root(
        &mut commands,
        container,
        Player::Two,
        characters.get(players.two).unwrap(),
        &icons,
        &fonts,
    );

    setup_shop_bottom_bar(&mut commands, root);

    commands.insert_resource(Shops {
        player_one: Shop {
            components: player_one_components,
            selected_index: 0,
            max_index: characters.get(players.one).unwrap().items.len() - 1,
            closed: false,
        },
        player_two: Shop {
            components: player_two_components,
            selected_index: 0,
            max_index: characters.get(players.two).unwrap().items.len() - 1,
            closed: false,
        },
    });
}

#[derive(Debug, Component)]
pub struct ShopMoney;
#[derive(Debug, Component)]
pub struct ShopScore;

fn setup_shop_top_bar(commands: &mut Commands, container: Entity) {
    let style = TextStyle {
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Percent(0.5)),
                    ..default()
                },
                ..default()
            },
            Name::new("Shop top bar"),
        ))
        .set_parent(container)
        .with_children(|cb| {
            cb.spawn((
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("$", style.clone()),
                        TextSection::new("0", style.clone()),
                    ]),
                    ..default()
                },
                ShopMoney,
                Owner(Player::One),
            ));

            cb.spawn((
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("0", style.clone()),
                        TextSection::new(" - ", style.clone()),
                        TextSection::new("0", style.clone()),
                    ]),
                    ..default()
                },
                ShopScore,
            ));

            cb.spawn((
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("$", style.clone()),
                        TextSection::new("0", style.clone()),
                    ]),
                    ..default()
                },
                ShopMoney,
                Owner(Player::Two),
            ));
        });
}

fn setup_shop_bottom_bar(commands: &mut Commands, container: Entity) {
    let style = TextStyle {
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Percent(2.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Shop bottom bar"),
        ))
        .set_parent(container)
        .with_children(|cb| {
            for (text, color) in [
                ("A/Cross to buy", GENERIC_TEXT_COLOR),
                ("Selected", ITEM_SLOT_HIGHLIGHT_COLOR),
                ("Component", ITEM_SLOT_COMPONENT_COLOR),
                ("Upgrade", ITEM_SLOT_UPGRADE_COLOR),
                ("Owned", ITEM_SLOT_OWNED_COLOR),
                ("Purchasable", ITEM_SLOT_DEFAULT_COLOR),
                ("Not purchasable", ITEM_SLOT_DISABLED_COLOR),
                ("X/Square to sell", GENERIC_TEXT_COLOR),
            ] {
                cb.spawn(TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            color,
                            ..style.clone()
                        },
                    ),
                    ..default()
                });
            }
        });
}

fn setup_shop_root(
    commands: &mut Commands,
    parent: Entity,
    owner: Player,
    character: &Character,
    icons: &Icons,
    fonts: &Fonts,
) -> ShopComponents {
    let mut shop_root_builder = ShopComponentsBuilder::default();

    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DIVIDER_COLOR.into(),
                style: Style {
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Percent(0.5),

                    flex_basis: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Player {} shop root", &owner)),
        ))
        .set_parent(parent)
        .id();

    setup_info_panel(commands, container, &mut shop_root_builder, fonts);
    setup_shop_grid(
        commands,
        icons,
        container,
        &mut shop_root_builder,
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
    let container = commands
        .spawn((
            NodeBundle {
                background_color: SHOP_DARK_BACKGROUND_COLOR.into(),
                style: Style {
                    padding: UiRect::all(Val::Px(3.0)),
                    height: Val::Percent(25.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Info panel"),
        ))
        .set_parent(parent)
        .id();

    shop_root.big_icon = Some(big_icon(commands, container));
    setup_explanation_box(commands, container, shop_root, fonts);
}

fn big_icon(commands: &mut Commands, parent: Entity) -> Entity {
    commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    max_width: Val::Px(200.0),
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
        font_size: 30.0,
        color: GENERIC_TEXT_COLOR,
    };

    shop_root.item_name = Some(setup_text_sections(
        commands,
        container,
        vec!["Item name"],
        TextStyle {
            font_size: 48.0,
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

fn setup_shop_grid(
    commands: &mut Commands,
    icons: &Icons,
    parent: Entity,
    shop_root: &mut ShopComponentsBuilder,
    character: &Character,
    player: Player,
) {
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(SHOP_COLUMNS as u16, 1.0),
                    row_gap: Val::Px(5.0),
                    column_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Available items root"),
        ))
        .set_parent(parent)
        .id();

    shop_root.grid_items = fill_item_grid(commands, icons, container, player, character);
}

fn fill_item_grid(
    commands: &mut Commands,
    icons: &Icons,
    parent: Entity,
    player: Player,
    character: &Character,
) -> Vec<Entity> {
    let mut pairs = character.items.iter().collect::<Vec<_>>();

    pairs.sort_by_key(|(id, _)| (recursive_cost(character, **id), **id));

    pairs
        .into_iter()
        .map(|(id, item)| setup_shop_item(commands, icons, parent, player, *id, item.icon))
        .collect()
}

fn recursive_cost(character: &Character, item_id: ItemId) -> usize {
    let item = character.items.get(&item_id).unwrap();
    match item.category {
        characters::ItemCategory::Upgrade(ref components) => {
            item.cost
                + components
                    .iter()
                    .map(|i| recursive_cost(character, *i))
                    .sum::<usize>()
        }
        _ => item.cost,
    }
}

fn setup_shop_item(
    commands: &mut Commands,
    icons: &Icons,
    parent: Entity,
    player: Player,
    id: ItemId,
    icon: Icon,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    aspect_ratio: Some(1.0),
                    max_height: Val::Px(100.0),
                    max_width: Val::Px(100.0),
                    ..default()
                },
                ..default()
            },
            ShopItem(id),
            Owner(player),
        ))
        .set_parent(parent)
        .with_children(|cb| {
            cb.spawn(ImageBundle {
                image: UiImage::new(icons.0.get(&icon).unwrap().clone()),
                ..default()
            });
        })
        .id()
}
