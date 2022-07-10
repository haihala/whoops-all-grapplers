use bevy::prelude::*;
use time::Clock;
use types::Player;

use crate::assets::{Colors, Fonts};

use super::utils::{div, div_style, FULL};

#[derive(Debug)]
struct Notification {
    created_at: usize,
    entity: Entity,
}

#[derive(Debug)]
pub struct Notifications {
    spawned: Vec<Notification>,
    requests: Vec<(Player, &'static str)>,
    p1_container: Entity,
    p2_container: Entity,
}
impl Notifications {
    pub fn add(&mut self, player: Player, content: &'static str) {
        self.requests.push((player, content));
    }

    fn get_parent(&self, parent: Player) -> Entity {
        match parent {
            Player::One => self.p1_container,
            Player::Two => self.p2_container,
        }
    }
}

pub fn setup_toasts(mut commands: &mut Commands) {
    let p1_container = create_notification_container(&mut commands, Player::One);
    let p2_container = create_notification_container(&mut commands, Player::Two);

    commands.insert_resource(Notifications {
        spawned: vec![],
        requests: vec![],
        p1_container,
        p2_container,
    });
}

fn create_notification_container(commands: &mut Commands, side: Player) -> Entity {
    let top_margin = 15.0;
    let top = Val::Percent(top_margin);
    let size = Size::new(Val::Percent(15.0), Val::Percent(100.0 - top_margin));
    let position = match side {
        Player::One => Rect {
            left: Val::Px(0.0),
            top,
            ..default()
        },
        Player::Two => Rect {
            right: Val::Px(0.0),
            top,
            ..default()
        },
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                position_type: PositionType::Absolute,
                position,
                size,
                ..div_style()
            },
            ..div()
        })
        .id()
}

const TIME_TO_LIVE: usize = 3 * constants::FPS as usize;

pub fn update(
    mut commands: Commands,
    fonts: Res<Fonts>,
    colors: Res<Colors>,
    mut toasts: ResMut<Notifications>,
    clock: Res<Clock>,
) {
    for expired_toast in toasts
        .spawned
        .drain_filter(|notification| notification.created_at + TIME_TO_LIVE < clock.frame)
    {
        commands.entity(expired_toast.entity).despawn_recursive();
    }

    for (player, content) in toasts.requests.drain(..).collect::<Vec<_>>().into_iter() {
        commands
            .entity(toasts.get_parent(player))
            .with_children(|parent| {
                toasts.spawned.push(Notification {
                    created_at: clock.frame,
                    entity: spawn_notification(
                        parent,
                        fonts.basic.clone(),
                        colors.notification_background,
                        colors.notification_text,
                        content,
                    ),
                })
            });
    }
}

fn spawn_notification(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    bg_color: Color,
    text_color: Color,
    content: &'static str,
) -> Entity {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(FULL, Val::Auto),
                margin: Rect::all(Val::Px(7.0)),
                justify_content: JustifyContent::Center,
                ..div_style()
            },
            color: bg_color.into(),
            ..div()
        })
        .with_children(|container| {
            container.spawn_bundle(TextBundle {
                text: Text::with_section(
                    content.to_string(),
                    TextStyle {
                        font,
                        font_size: 18.0,
                        color: text_color,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
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
