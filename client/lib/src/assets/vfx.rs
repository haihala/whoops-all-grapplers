use bevy::{prelude::*, utils::HashMap};

use wag_core::{Clock, InCombat, VfxRequest, VisualEffect};

use crate::entity_management::DespawnMarker;

use super::materials::{
    BlankMaterial, BlockEffectMaterial, ClashSparkMaterial, FocalPointLinesMaterial,
    HitSparkMaterial, LineFieldMaterial, Reset, RingRippleMaterial,
};

#[derive(Debug, Resource)]
pub struct Vfx {
    meshes: HashMap<VisualEffect, Handle<Mesh>>,
    queue: Vec<VfxRequest>,
    blank_material: Handle<BlankMaterial>,
    clash_spark_material: Handle<ClashSparkMaterial>,
    block_effect_material: Handle<BlockEffectMaterial>,
    hit_spark_material: Handle<HitSparkMaterial>,
    throw_tech_material: Handle<RingRippleMaterial>,
    speed_lines_material: Handle<LineFieldMaterial>,
    throw_target_material: Handle<FocalPointLinesMaterial>,
}
impl Vfx {
    pub fn new(
        meshes: HashMap<VisualEffect, Handle<Mesh>>,
        blank_material: Handle<BlankMaterial>,
        clash_spark_material: Handle<ClashSparkMaterial>,
        block_effect_material: Handle<BlockEffectMaterial>,
        hit_spark_material: Handle<HitSparkMaterial>,
        throw_tech_material: Handle<RingRippleMaterial>,
        speed_lines_material: Handle<LineFieldMaterial>,
        throw_target_material: Handle<FocalPointLinesMaterial>,
    ) -> Vfx {
        Vfx {
            meshes,
            queue: vec![],
            blank_material,
            hit_spark_material,
            clash_spark_material,
            block_effect_material,
            throw_tech_material,
            speed_lines_material,
            throw_target_material,
        }
    }

    pub fn spawn(&mut self, request: VfxRequest) {
        self.queue.push(request);
    }
}

fn spawn_vfx<M>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    transform: Transform,
    material_handle: Handle<M>,
    material_asset: &mut ResMut<Assets<M>>,
    elapsed_seconds: f32,
    despawn_frame: usize,
) where
    M: Material + Reset,
{
    material_asset
        .get_mut(&material_handle)
        .unwrap()
        .reset(elapsed_seconds);
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            transform,
            material: material_handle,
            ..default()
        },
        DespawnMarker(despawn_frame),
        StateScoped(InCombat),
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn handle_requests(
    mut commands: Commands,
    mut vfx: ResMut<Vfx>,
    clock: Res<Clock>,
    time: Res<Time>,
    mut blank_materials: ResMut<Assets<BlankMaterial>>,
    mut block_materials: ResMut<Assets<BlockEffectMaterial>>,
    mut clash_materials: ResMut<Assets<ClashSparkMaterial>>,
    mut hit_spark_materials: ResMut<Assets<HitSparkMaterial>>,
    mut throw_tech_materials: ResMut<Assets<RingRippleMaterial>>,
    mut speed_lines_materials: ResMut<Assets<LineFieldMaterial>>,
    mut throw_target_materials: ResMut<Assets<FocalPointLinesMaterial>>,
) {
    for VfxRequest {
        effect,
        position,
        rotation,
    } in vfx.queue.drain(..).collect::<Vec<_>>().into_iter()
    {
        let mesh = vfx.meshes.get(&effect).unwrap().clone();
        let mut transform = Transform::from_translation(position + 0.1 * Vec3::Z);

        if let Some(angle) = rotation {
            transform.rotate_z(angle);
        }

        match effect {
            VisualEffect::Blank => spawn_vfx(
                &mut commands,
                mesh,
                transform,
                vfx.blank_material.clone(),
                &mut blank_materials,
                time.elapsed_seconds(),
                clock.frame + 15,
            ),
            VisualEffect::Hit => {
                spawn_vfx(
                    &mut commands,
                    mesh,
                    transform,
                    vfx.hit_spark_material.clone(),
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
                    vfx.clash_spark_material.clone(),
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
                    vfx.block_effect_material.clone(),
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
                    vfx.throw_tech_material.clone(),
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
                    vfx.speed_lines_material.clone(),
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
                    vfx.throw_target_material.clone(),
                    &mut throw_target_materials,
                    time.elapsed_seconds(),
                    clock.frame + 60,
                );
            }
        };
    }
}
