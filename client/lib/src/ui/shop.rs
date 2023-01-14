use bevy::prelude::*;

use wag_core::{GameState, OnlyShowInGameState, Player};

#[derive(Debug, Component)]
struct ShopRoot(Player);

pub(super) fn setup_shop(mut commands: Commands) {
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
            setup_shop_root(child_builder, Player::One);
            setup_shop_root(child_builder, Player::Two);
        });
}

fn setup_shop_root(root: &mut ChildBuilder, owner: Player) {
    let margin = match owner {
        Player::One => UiRect {
            right: Val::Px(5.0),
            ..default()
        },
        Player::Two => UiRect {
            left: Val::Px(5.0),
            ..default()
        },
    };

    root.spawn((
        NodeBundle {
            background_color: Color::GRAY.into(),
            style: Style {
                size: Size {
                    height: Val::Percent(100.0),
                    width: Val::Percent(50.0),
                },
                margin,
                ..default()
            },
            ..default()
        },
        ShopRoot(owner),
    ));
}
