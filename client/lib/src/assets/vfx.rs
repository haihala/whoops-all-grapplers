use bevy::prelude::*;

use wag_core::{Clock, Facing, MatchState, VfxRequest, VisualEffect};

use crate::{
    entity_management::DespawnMarker,
    event_spreading::{SpawnRelativeVfx, SpawnVfx},
};

use super::materials::{
    BlankMaterial, BlockEffectMaterial, ClashSparkMaterial, FocalPointLinesMaterial, FromTime,
    HitSparkMaterial, LightningBoltMaterial, LineFieldMaterial, RingRippleMaterial,
};

fn spawn_vfx<M>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    transform: Transform,
    material_asset: &mut ResMut<Assets<M>>,
    current_time: f32,
    despawn_frame: usize,
) where
    M: Material + FromTime,
{
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            transform,
            material: material_asset.add(M::from_time(current_time)),
            ..default()
        },
        DespawnMarker(despawn_frame),
        StateScoped(MatchState::Combat),
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn start_relative_vfx(
    trigger: Trigger<SpawnRelativeVfx>,
    query: Query<(&Transform, &Facing)>,
    mut commands: Commands,
) {
    // from previous trigger
    let (tf, facing) = query.get(trigger.entity()).unwrap();
    let mut request = trigger.event().0;
    request.position += tf.translation;
    request.mirror ^= facing.to_flipped();
    commands.trigger(SpawnVfx(request));
}

#[allow(clippy::too_many_arguments)]
pub fn start_absolute_vfx(
    trigger: Trigger<SpawnVfx>,
    mut commands: Commands,
    clock: Res<Clock>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut blank_materials: ResMut<Assets<BlankMaterial>>,
    mut block_materials: ResMut<Assets<BlockEffectMaterial>>,
    mut clash_materials: ResMut<Assets<ClashSparkMaterial>>,
    mut hit_spark_materials: ResMut<Assets<HitSparkMaterial>>,
    mut throw_tech_materials: ResMut<Assets<RingRippleMaterial>>,
    mut speed_lines_materials: ResMut<Assets<LineFieldMaterial>>,
    mut throw_target_materials: ResMut<Assets<FocalPointLinesMaterial>>,
    mut lightning_materials: ResMut<Assets<LightningBoltMaterial>>,
) {
    let SpawnVfx(VfxRequest {
        effect,
        position,
        rotation,
        mirror,
    }) = trigger.event();

    let mesh = meshes.add(effect.mesh_size());
    let mut transform = Transform::from_translation(*position + 0.1 * Vec3::Z);

    if let Some(angle) = rotation {
        transform.rotate_z(*angle);
    }

    if *mirror {
        transform.scale.x *= -1.0;
    }

    match effect {
        VisualEffect::Blank => spawn_vfx(
            &mut commands,
            mesh,
            transform,
            &mut blank_materials,
            time.elapsed_seconds(),
            clock.frame + 15,
        ),
        VisualEffect::Hit => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut hit_spark_materials,
                time.elapsed_seconds(),
                clock.frame + 10,
            );
        }
        VisualEffect::Clash => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut clash_materials,
                time.elapsed_seconds(),
                clock.frame + 10,
            );
        }
        VisualEffect::Block => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut block_materials,
                time.elapsed_seconds(),
                clock.frame + 10,
            );
        }
        VisualEffect::ThrowTech => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut throw_tech_materials,
                time.elapsed_seconds(),
                clock.frame + 60,
            );
        }
        VisualEffect::SpeedLines => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut speed_lines_materials,
                time.elapsed_seconds(),
                clock.frame + 20,
            );
        }
        VisualEffect::ThrowTarget => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut throw_target_materials,
                time.elapsed_seconds(),
                clock.frame + 60,
            );
        }
        VisualEffect::Lightning => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut lightning_materials,
                time.elapsed_seconds(),
                clock.frame + 60,
            );
        }
    };
}
