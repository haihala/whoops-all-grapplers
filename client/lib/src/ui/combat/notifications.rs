use bevy::prelude::*;
use wag_core::{Clock, GameState, OnlyShowInGameState, Player};

use crate::assets::{Colors, Fonts};

#[derive(Debug)]
struct Notification {
    created_at: usize,
    entity: Entity,
}

#[derive(Debug, Resource, Default)]
pub struct Notifications {
    spawned: Vec<Notification>,
    requests: Vec<(Player, String)>,
}
impl Notifications {
    pub fn add(&mut self, player: Player, content: String) {
        self.requests.push((player, content));
    }
}

#[derive(Debug, Component, Deref)]
pub struct NotificationContainer(Player);

pub fn setup_toasts(root: &mut ChildBuilder, player: Player) {
    root.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: match player {
                    // Align towards the edge of the screen
                    Player::One => AlignItems::FlexStart,
                    Player::Two => AlignItems::FlexEnd,
                },
                size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                ..default()
            },
            ..default()
        },
        OnlyShowInGameState(vec![GameState::Combat, GameState::PostRound]),
        NotificationContainer(player),
    ));
}

const TIME_TO_LIVE: usize = 3 * wag_core::FPS as usize;

pub fn update_notifications(
    mut commands: Commands,
    fonts: Res<Fonts>,
    colors: Res<Colors>,
    mut toasts: ResMut<Notifications>,
    containers: Query<(Entity, &NotificationContainer)>,
    clock: Res<Clock>,
) {
    for expired_toast in toasts
        .spawned
        .drain_filter(|notification| notification.created_at + TIME_TO_LIVE < clock.frame)
    {
        commands.entity(expired_toast.entity).despawn_recursive();
    }

    for (player, content) in toasts.requests.drain(..).collect::<Vec<_>>().into_iter() {
        for (container, marker) in &containers {
            if player == **marker {
                commands.entity(container).with_children(|parent| {
                    toasts.spawned.push(Notification {
                        created_at: clock.frame,
                        entity: spawn_notification(
                            parent,
                            fonts.basic.clone(),
                            colors.notification_background,
                            colors.notification_text,
                            content.clone(), // Not necessary technically, but the compiler can't know that each message will only be used once
                        ),
                    })
                });
            }
        }
    }
}

fn spawn_notification(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    bg_color: Color,
    text_color: Color,
    content: String,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(40.0), Val::Auto),
                margin: UiRect::all(Val::Px(7.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: bg_color.into(),
            ..default()
        })
        .with_children(|container| {
            container.spawn(TextBundle {
                text: Text::from_section(
                    content,
                    TextStyle {
                        font,
                        font_size: 18.0,
                        color: text_color,
                    },
                ),
                style: Style {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id()
}
