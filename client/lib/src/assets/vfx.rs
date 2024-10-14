use bevy::prelude::*;

use wag_core::{Clock, Facing, MatchState, VfxRequest, VisualEffect};

use crate::{
    entity_management::DespawnMarker,
    event_spreading::{SpawnRelativeVfx, SpawnVfx},
};

use super::materials::{
    BlankMaterial, BlockEffectMaterial, ClashSparkMaterial, DiagonalWaveMaterial, FlatWaveMaterial,
    FocalPointLinesMaterial, HitSparkMaterial, LightningBoltMaterial, LineFieldMaterial,
    MidFlashMaterial, PebbleMaterial, RingRippleMaterial,
};

fn spawn_vfx<M>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    transform: Transform,
    material_asset: &mut ResMut<Assets<M>>,
    material: M,
    despawn_frame: usize,
) where
    M: Material,
{
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            transform,
            material: material_asset.add(material),
            ..default()
        },
        DespawnMarker(despawn_frame),
        StateScoped(MatchState::Combat),
        Name::new("VFX"),
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
    if facing.to_flipped() {
        request.mirror = !request.mirror;
    }
    request.tf.translation += tf.translation;
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
    mut diagonal_wave_materials: ResMut<Assets<DiagonalWaveMaterial>>,
    mut flat_wave_materials: ResMut<Assets<FlatWaveMaterial>>,
    mut pebble_material: ResMut<Assets<PebbleMaterial>>,
    mut mid_flash_materials: ResMut<Assets<MidFlashMaterial>>,
) {
    let SpawnVfx(VfxRequest { effect, tf, mirror }) = trigger.event();

    let mesh = meshes.add(effect.mesh_size());
    let transform = Transform {
        translation: tf.translation + 0.1 * Vec3::Z,
        ..*tf
    };

    match effect {
        VisualEffect::Blank => spawn_vfx(
            &mut commands,
            mesh,
            transform,
            &mut blank_materials,
            BlankMaterial {},
            clock.frame + 15,
        ),
        VisualEffect::Hit => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut hit_spark_materials,
                HitSparkMaterial::new(time.elapsed_seconds()),
                clock.frame + 10,
            );
        }
        VisualEffect::Clash => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut clash_materials,
                ClashSparkMaterial::new(time.elapsed_seconds()),
                clock.frame + 10,
            );
        }
        VisualEffect::Block => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut block_materials,
                BlockEffectMaterial::new(time.elapsed_seconds()),
                clock.frame + 10,
            );
        }
        VisualEffect::ThrowTech => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut throw_tech_materials,
                RingRippleMaterial::new(time.elapsed_seconds()),
                clock.frame + 60,
            );
        }
        VisualEffect::SpeedLines => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut speed_lines_materials,
                LineFieldMaterial::new(time.elapsed_seconds(), *mirror),
                clock.frame + 20,
            );
        }
        VisualEffect::ThrowTarget => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut throw_target_materials,
                FocalPointLinesMaterial::new(time.elapsed_seconds()),
                clock.frame + 60,
            );
        }
        VisualEffect::Lightning => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut lightning_materials,
                LightningBoltMaterial::new(time.elapsed_seconds(), *mirror),
                clock.frame + 60,
            );
        }
        VisualEffect::WaveDiagonal => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut diagonal_wave_materials,
                DiagonalWaveMaterial::new(time.elapsed_seconds()),
                clock.frame + 60,
            );
        }
        VisualEffect::WaveFlat => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut flat_wave_materials,
                FlatWaveMaterial::new(time.elapsed_seconds()),
                clock.frame + 60,
            );
        }
        VisualEffect::Pebbles => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut pebble_material,
                PebbleMaterial::new(time.elapsed_seconds(), *mirror),
                clock.frame + 60,
            );
        }
        VisualEffect::MidFlash => {
            spawn_vfx(
                &mut commands,
                mesh,
                transform,
                &mut mid_flash_materials,
                MidFlashMaterial::new(time.elapsed_seconds()),
                clock.frame + 60,
            );
        }
    };
}
