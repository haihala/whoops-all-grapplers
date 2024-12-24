use bevy::prelude::*;
use characters::{GaugeType, Gauges};
use foundation::{
    Clock, Combo, MatchState, Player, Players, COMBO_COUNTER_TEXT_COLOR, FPS,
    NOTIFICATION_BACKGROUND_COLOR, NOTIFICATION_TEXT_COLOR,
};

use crate::{assets::Fonts, entity_management::VisibleInStates};

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

#[derive(Debug, Component, Deref)]
pub struct ComboCounter(Player);

#[derive(Debug, Component)]
pub struct ComboHitsMarker;

#[derive(Debug, Component)]
pub struct ComboDamageMarker;

pub fn setup_toasts(commands: &mut Commands, parent: Entity, player: Player) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: match player {
                    // Align towards the edge of the screen
                    Player::One => AlignItems::FlexStart,
                    Player::Two => AlignItems::FlexEnd,
                },
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                ..default()
            },
            VisibleInStates(vec![MatchState::Combat, MatchState::PostRound]),
            NotificationContainer(player),
        ))
        .set_parent(parent);
}

pub fn setup_combo_counter(commands: &mut Commands, parent: Entity, player: Player, fonts: &Fonts) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                padding: UiRect::new(
                    Val::Percent(10.0),
                    Val::Percent(10.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                ),
                ..default()
            },
            VisibleInStates(vec![MatchState::Combat, MatchState::PostRound]),
            Visibility::Inherited,
            Name::new("Combo counter"),
        ))
        .set_parent(parent)
        .with_children(|cb| {
            // This exists so that we can use both the generic visibility system and the more
            // fine-grained model that hides the combo counter when not in a combo
            cb.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: match player {
                        // Align towards the edge of the screen
                        Player::One => AlignItems::FlexStart,
                        Player::Two => AlignItems::FlexEnd,
                    },
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ComboCounter(player),
                Visibility::Inherited,
            ))
            .with_children(|mb| {
                let style_bundle = (
                    TextFont {
                        font: fonts.basic.clone(),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(COMBO_COUNTER_TEXT_COLOR),
                );

                mb.spawn((Text::new("Combo!"), style_bundle.clone()));
                mb.spawn((Text::new("Hits 0"), style_bundle.clone(), ComboHitsMarker));
                mb.spawn((
                    Text::new("Damage 0"),
                    style_bundle.clone(),
                    ComboDamageMarker,
                ));
            });
        });
}

const TIME_TO_LIVE: usize = 3 * FPS as usize;

pub fn update_notifications(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut toasts: ResMut<Notifications>,
    containers: Query<(Entity, &NotificationContainer)>,
    clock: Res<Clock>,
) {
    for expired_toast in toasts.spawned.extract_if(|notification| {
        notification.created_at + TIME_TO_LIVE < clock.frame
            || notification.created_at > clock.frame // Previous round
    }) {
        // This structure needs to be here, as the entity gets despawned sometimes
        if let Some(ent) = commands.get_entity(expired_toast.entity) {
            ent.despawn_recursive();
        }
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
                            NOTIFICATION_BACKGROUND_COLOR,
                            NOTIFICATION_TEXT_COLOR,
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
        .spawn((
            Node {
                width: Val::Percent(40.0),
                height: Val::Auto,
                margin: UiRect::all(Val::Px(7.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|container| {
            container.spawn((
                Text::new(content),
                TextFont {
                    font,
                    font_size: 18.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));
        })
        .id()
}

#[allow(clippy::type_complexity)]
pub fn update_combo_counters(
    player_query: Query<(&Combo, &Player, &Gauges)>,
    players: Res<Players>,
    mut roots: Query<(&mut Visibility, &ComboCounter, &Children)>,
    mut texts: ParamSet<(
        Query<(&mut Text, Entity), With<ComboHitsMarker>>,
        Query<(&mut Text, Entity), With<ComboDamageMarker>>,
    )>,
) {
    if roots.iter().count() == 0 {
        // Combo counters don't always exist by this time
        // They get spawned in when entering combat
        return;
    }

    for player in [Player::One, Player::Two] {
        let entity = players.get(player);
        let [(combo, _, _), (_, _, resources)] = player_query
            .get_many([entity, players.get_other_entity(entity)])
            .unwrap();

        let (mut root_visibility, _, children) =
            roots.iter_mut().find(|(_, cc, _)| cc.0 == player).unwrap();

        if !combo.ongoing() {
            *root_visibility = Visibility::Hidden;
            continue;
        };
        *root_visibility = Visibility::Inherited;

        let mut hit_query = texts.p0();
        let mut hit_text = hit_query
            .iter_mut()
            .find(|(_, entity)| children.contains(entity))
            .unwrap();
        hit_text.0 .0 = format!("Hits: {}", combo.hits);

        let mut damages_query = texts.p1();
        let mut damage_text = damages_query
            .iter_mut()
            .find(|(_, entity)| children.contains(entity))
            .unwrap();
        let current_health = resources.get(GaugeType::Health).unwrap().current;
        damage_text.0 .0 = format!("Total damage: {}", combo.old_health - current_health);
    }
}
