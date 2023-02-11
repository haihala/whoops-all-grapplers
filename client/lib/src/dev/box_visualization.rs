use bevy::prelude::*;
use characters::{Hitbox, Hurtbox};

use crate::{assets::Colors, physics::Pushbox};

#[derive(Debug, Component)]
pub(super) enum BoxVisual {
    Hurtbox,
    Hitbox,
    Pushbox,
}

pub(super) fn spawn_boxes(
    mut commands: Commands,
    colors: Res<Colors>,
    hitboxes: Query<(Entity, &Hitbox, Option<&Children>)>,
    hurtboxes: Query<(Entity, &Hurtbox, Option<&Children>)>,
    pushboxes: Query<(Entity, &Pushbox, Option<&Children>)>,
    sprites: Query<&Sprite, With<BoxVisual>>,
) {
    for (entity, hitbox, maybe_children) in &hitboxes {
        handle_box_spawning(
            &mut commands,
            &sprites,
            entity,
            maybe_children,
            BoxVisual::Hitbox,
            colors.hitbox,
            hitbox.center(),
            Some(hitbox.size()),
        );
    }
    for (entity, hurtbox, maybe_children) in &hurtboxes {
        handle_box_spawning(
            &mut commands,
            &sprites,
            entity,
            maybe_children,
            BoxVisual::Hurtbox,
            colors.hurtbox,
            hurtbox.center(),
            Some(hurtbox.size()),
        );
    }
    for (entity, pushbox, maybe_children) in &pushboxes {
        handle_box_spawning(
            &mut commands,
            &sprites,
            entity,
            maybe_children,
            BoxVisual::Pushbox,
            colors.pushbox,
            pushbox.center(),
            Some(pushbox.size()),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_box_spawning(
    commands: &mut Commands,
    sprites: &Query<&Sprite, With<BoxVisual>>,
    entity: Entity,
    maybe_children: Option<&Children>,
    marker: BoxVisual,
    color: Color,
    offset: Vec2,
    custom_size: Option<Vec2>,
) {
    if maybe_children.is_none()
        || !maybe_children
            .unwrap()
            .iter()
            .any(|e| sprites.get(*e).is_ok())
    {
        // If there is no child entity that has the marker component and a sprite (has been handled)
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(offset.extend(0.0)),
                    sprite: Sprite {
                        color,
                        custom_size,
                        ..default()
                    },
                    ..default()
                },
                marker,
            ));
        });
    }
}

pub(super) fn size_adjustment(
    players: Query<&Pushbox>,
    hitboxes: Query<&Hitbox>,
    hurtboxes: Query<&Hurtbox>,
    mut sprites: Query<(&mut Sprite, &mut Transform, &BoxVisual, &Parent)>,
) {
    for (mut sprite, mut tf, box_type, parent) in &mut sprites {
        let area = match box_type {
            BoxVisual::Hurtbox => hurtboxes.get(**parent).unwrap().0,
            BoxVisual::Hitbox => hitboxes.get(**parent).unwrap().0,
            BoxVisual::Pushbox => players.get(**parent).unwrap().0,
        };
        sprite.custom_size = Some(area.size());
        tf.translation = area.center().extend(0.0);
    }
}
