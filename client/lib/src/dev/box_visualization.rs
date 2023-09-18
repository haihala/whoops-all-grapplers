use bevy::prelude::{shape::Quad, *};
use characters::{Hitbox, Hurtbox};

use crate::{assets::Colors, physics::Pushbox};

#[derive(Debug, Component)]
pub(super) enum BoxVisual {
    Hurtbox,
    Hitbox,
    Pushbox,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn spawn_boxes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    colors: Res<Colors>,
    hitboxes: Query<(Entity, &Hitbox, Option<&Children>)>,
    hurtboxes: Query<(Entity, &Hurtbox, Option<&Children>)>,
    pushboxes: Query<(Entity, &Pushbox, Option<&Children>)>,
    spawned: Query<(), With<BoxVisual>>,
) {
    for (entity, hitbox, maybe_children) in &hitboxes {
        handle_box_spawning(
            &mut commands,
            &mut mesh_assets,
            &mut material_assets,
            &spawned,
            entity,
            maybe_children,
            BoxVisual::Hitbox,
            colors.hitbox,
            hitbox.center(),
            hitbox.size(),
        );
    }
    for (entity, hurtbox, maybe_children) in &hurtboxes {
        handle_box_spawning(
            &mut commands,
            &mut mesh_assets,
            &mut material_assets,
            &spawned,
            entity,
            maybe_children,
            BoxVisual::Hurtbox,
            colors.hurtbox,
            hurtbox.center(),
            hurtbox.size(),
        );
    }
    for (entity, pushbox, maybe_children) in &pushboxes {
        handle_box_spawning(
            &mut commands,
            &mut mesh_assets,
            &mut material_assets,
            &spawned,
            entity,
            maybe_children,
            BoxVisual::Pushbox,
            colors.pushbox,
            pushbox.center(),
            pushbox.size(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_box_spawning(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    material_assets: &mut ResMut<Assets<StandardMaterial>>,
    spawned: &Query<(), With<BoxVisual>>,
    entity: Entity,
    maybe_children: Option<&Children>,
    marker: BoxVisual,
    color: Color,
    offset: Vec2,
    custom_size: Vec2,
) {
    // Only add visual to components with no children with the marker
    if maybe_children.is_none()
        || !maybe_children
            .unwrap()
            .iter()
            .any(|e| spawned.get(*e).is_ok())
    {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    transform: Transform::from_translation(offset.extend(0.0)),
                    material: material_assets.add(color.into()),
                    mesh: mesh_assets.add(Quad::new(custom_size).into()),
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
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut spawned: Query<(&Handle<Mesh>, &mut Transform, &BoxVisual, &Parent)>,
) {
    for (mesh_handle, mut tf, box_type, parent) in &mut spawned {
        let area = match box_type {
            BoxVisual::Hurtbox => hurtboxes.get(**parent).unwrap().0,
            BoxVisual::Hitbox => hitboxes.get(**parent).unwrap().0,
            BoxVisual::Pushbox => players.get(**parent).unwrap().0,
        };
        tf.translation = area.center().extend(0.0);
        if let Some(mesh) = mesh_assets.get_mut(mesh_handle) {
            *mesh = Quad::new(area.size()).into();
        }
    }
}
