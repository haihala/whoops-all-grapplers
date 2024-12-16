use bevy::prelude::*;
use characters::{GaugeType, Gauges};
use foundation::{
    Area, CharacterFacing, Clock, MatchState, Owner, Pickup, PickupRequest, Player,
    RollbackSchedule, SystemStep,
};

use crate::{
    assets::Models,
    entity_management::DespawnMarker,
    movement::{ObjectVelocity, Pushbox},
};

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            pick_up_pickups.in_set(SystemStep::HouseKeeping),
        );
    }
}

pub fn pick_up_pickups(
    mut commands: Commands,
    pickups: Query<(Entity, &Pickup, &Area, &Owner, &Transform)>,
    mut picker_uppers: Query<(&mut Gauges, &Player, &Pushbox, &Transform)>,
) {
    for (entity, pickup, size, owner, pickup_tf) in &pickups {
        let pickup_target = size.with_center(pickup_tf.translation.truncate());
        for (mut resources, player, pushbox, player_tf) in &mut picker_uppers {
            let player_target = pushbox.0.with_center(player_tf.translation.truncate());
            let overlaps = pickup_target.intersects(&player_target);

            let is_owner = *player == owner.0;
            let can_pick_up = pickup.allow_pickup_by(is_owner);

            if overlaps && can_pick_up {
                // We have a hit!

                match pickup {
                    Pickup::Kunai => {
                        let res = resources.get_mut(GaugeType::KunaiCounter).unwrap();
                        res.gain(1);
                    }
                }

                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn spawn_pickups(
    trigger: Trigger<PickupRequest>,
    mut commands: Commands,
    clock: Res<Clock>,
    models: Res<Models>,
    query: Query<(&Transform, &Player, &CharacterFacing)>,
) {
    let PickupRequest {
        pickup,
        size,
        spawn_point,
        spawn_velocity,
        gravity,
        lifetime,
    } = trigger.event();
    let (model, transform) = pickup.spawn_info();

    let (player_tf, player, facing) = query.get(trigger.entity()).unwrap();

    let mut entity_commands = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(spawn_point.extend(0.0) + player_tf.translation),
            ..default()
        },
        *pickup,
        *size,
        Owner(player.other()), // FIXME: This is here because it is spawned on hit
        StateScoped(MatchState::Combat),
        ObjectVelocity {
            speed: facing.visual.mirror_vec3(spawn_velocity.extend(0.0)),
            acceleration: -Vec3::Y * *gravity,
            face_forward: false,
            floor_despawns: false,
        },
    ));

    if let Some(frames) = lifetime {
        entity_commands.insert(DespawnMarker(frames + clock.frame));
    };

    entity_commands.with_children(|cb| {
        cb.spawn(SceneBundle {
            transform,
            scene: models[&model].clone(),
            ..default()
        });
    });
}
