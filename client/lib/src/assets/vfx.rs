use bevy::{prelude::*, utils::HashMap};

use wag_core::{Clock, GameState, VisualEffect};

use crate::entity_management::{DespawnMarker, LivesInStates};

use super::materials::{BlockEffectMaterial, ClashSparkMaterial, HitSparkMaterial};

#[derive(Debug)]
pub struct VfxRequest {
    pub effect: VisualEffect,
    pub position: Vec3,
}

#[derive(Debug, Resource)]
pub struct Vfx {
    meshes: HashMap<VisualEffect, Handle<Mesh>>,
    queue: Vec<VfxRequest>,
    clash_spark_material: Handle<ClashSparkMaterial>,
    block_effect_material: Handle<BlockEffectMaterial>,
    hit_spark_material: Handle<HitSparkMaterial>,
}
impl Vfx {
    pub fn new(
        meshes: HashMap<VisualEffect, Handle<Mesh>>,
        clash_spark_material: Handle<ClashSparkMaterial>,
        block_effect_material: Handle<BlockEffectMaterial>,
        hit_spark_material: Handle<HitSparkMaterial>,
    ) -> Vfx {
        Vfx {
            meshes,
            queue: vec![],
            hit_spark_material,
            clash_spark_material,
            block_effect_material,
        }
    }

    pub fn spawn(&mut self, request: VfxRequest) {
        self.queue.push(request);
    }
}

pub fn handle_requests(
    mut commands: Commands,
    mut particles: ResMut<Vfx>,
    clock: Res<Clock>,
    time: Res<Time>,
    mut block_materials: ResMut<Assets<BlockEffectMaterial>>,
    mut clash_materials: ResMut<Assets<ClashSparkMaterial>>,
    mut hit_spark_materials: ResMut<Assets<HitSparkMaterial>>,
) {
    for VfxRequest { effect, position } in particles.queue.drain(..).collect::<Vec<_>>().into_iter()
    {
        let mesh = particles.meshes.get(&effect).unwrap().clone();
        let transform = Transform::from_translation(position.with_y(position.y.max(0.8)) + Vec3::Z);
        match effect {
            VisualEffect::Hit => {
                let material_handle = particles.hit_spark_material.clone();

                hit_spark_materials
                    .get_mut(&material_handle)
                    .unwrap()
                    .reset(time.elapsed_seconds());

                commands.spawn((
                    MaterialMeshBundle {
                        mesh,
                        transform,
                        material: material_handle,
                        ..default()
                    },
                    DespawnMarker(clock.frame + 10),
                    LivesInStates(vec![GameState::Combat]),
                ));
            }
            VisualEffect::Clash => {
                let material_handle = particles.clash_spark_material.clone();

                clash_materials
                    .get_mut(&material_handle)
                    .unwrap()
                    .reset(time.elapsed_seconds());

                commands.spawn((
                    MaterialMeshBundle {
                        mesh,
                        transform,
                        material: material_handle,
                        ..default()
                    },
                    DespawnMarker(clock.frame + 10),
                    LivesInStates(vec![GameState::Combat]),
                ));
            }
            VisualEffect::Block => {
                let material_handle = particles.block_effect_material.clone();

                block_materials
                    .get_mut(&material_handle)
                    .unwrap()
                    .reset(time.elapsed_seconds());

                commands.spawn((
                    MaterialMeshBundle {
                        mesh,
                        transform,
                        material: material_handle,
                        ..default()
                    },
                    DespawnMarker(clock.frame + 10),
                    LivesInStates(vec![GameState::Combat]),
                ));
            }
        };
    }
}
