use bevy::prelude::*;

use foundation::{Clock, Facing, Icons, MatchState, VfxRequest, VisualEffect};

use crate::{
    entity_management::DespawnMarker,
    event_spreading::{SpawnRelativeVfx, SpawnVfx},
};

use super::materials::{
    BlankMaterial, BlockEffectMaterial, ClashSparkMaterial, DiagonalWaveMaterial, FlatWaveMaterial,
    FocalPointLinesMaterial, HitSparkMaterial, IconFlashMaterial, LightningBoltMaterial,
    LineFieldMaterial, MidFlashMaterial, PebbleMaterial, RingRippleMaterial, SmokeBombMaterial,
    SparkBurstMaterial,
};

pub fn spawn_vfx<M: Material>(
    trigger: Trigger<MaxSystemParamCountFix<M>>,
    mut commands: Commands,
    mut material_asset: ResMut<Assets<M>>,
    clock: Res<Clock>,
) {
    let MaxSystemParamCountFix {
        mesh,
        transform,
        material,
        frames_to_live,
    } = trigger.event();

    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: *transform,
            material: material_asset.add(material.clone()),
            ..default()
        },
        DespawnMarker(clock.frame + frames_to_live),
        StateScoped(MatchState::Combat),
        Name::new("VFX"),
    ));
}

pub fn start_relative_vfx(
    trigger: Trigger<SpawnRelativeVfx>,
    query: Query<(&Transform, &Facing)>,
    mut commands: Commands,
) {
    let (tf, facing) = query.get(trigger.entity()).unwrap();
    let mut request = trigger.event().0;
    if facing.to_flipped() {
        request.mirror = !request.mirror;
    }
    request.tf.translation += tf.translation;
    commands.trigger(SpawnVfx(request));
}

// Bevy allows for a max of N system params, which means we need to split the vfx spawning after we
// have more than N types of vfx (we do)
#[derive(Debug, Event)]
pub struct MaxSystemParamCountFix<T: Material> {
    mesh: Handle<Mesh>,
    transform: Transform,
    material: T,
    frames_to_live: usize,
}

pub fn start_absolute_vfx(
    trigger: Trigger<SpawnVfx>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    icons: Res<Icons>,
) {
    let SpawnVfx(VfxRequest { effect, tf, mirror }) = trigger.event();

    let mesh = meshes.add(effect.mesh_size());
    let transform = Transform {
        translation: tf.translation + 0.1 * Vec3::Z,
        ..*tf
    };

    match effect {
        VisualEffect::Blank => commands.trigger(MaxSystemParamCountFix {
            mesh,
            transform,
            material: BlankMaterial::default(),
            frames_to_live: 15,
        }),
        VisualEffect::Hit => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: HitSparkMaterial::new(time.elapsed_seconds()),
                frames_to_live: 10,
            });
        }
        VisualEffect::Clash => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: ClashSparkMaterial::new(time.elapsed_seconds()),
                frames_to_live: 10,
            });
        }
        VisualEffect::Block => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: BlockEffectMaterial::new(time.elapsed_seconds()),
                frames_to_live: 10,
            });
        }
        VisualEffect::RingPulse(c1, c2) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: RingRippleMaterial::new(*c1, *c2, time.elapsed_seconds()),
                frames_to_live: 60,
            });
        }
        VisualEffect::SpeedLines => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: LineFieldMaterial::new(time.elapsed_seconds(), *mirror),
                frames_to_live: 20,
            });
        }
        VisualEffect::ThrowTarget => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: FocalPointLinesMaterial::new(time.elapsed_seconds()),
                frames_to_live: 60,
            });
        }
        VisualEffect::Lightning => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: LightningBoltMaterial::new(time.elapsed_seconds(), *mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::WaveDiagonal(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: DiagonalWaveMaterial::new(time.elapsed_seconds(), *color),
                frames_to_live: 60,
            });
        }
        VisualEffect::WaveFlat(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: FlatWaveMaterial::new(time.elapsed_seconds(), *color),
                frames_to_live: 60,
            });
        }
        VisualEffect::Pebbles => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: PebbleMaterial::new(time.elapsed_seconds(), *mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::Sparks => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: SparkBurstMaterial::new(time.elapsed_seconds(), *mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::OpenerSpark(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: MidFlashMaterial::new(time.elapsed_seconds(), *color),
                frames_to_live: 60,
            });
        }
        VisualEffect::SmokeBomb => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: SmokeBombMaterial::new(time.elapsed_seconds()),
                frames_to_live: 60,
            });
        }
        VisualEffect::Icon(icon) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: IconFlashMaterial::new(
                    time.elapsed_seconds(),
                    icons.0.get(icon).unwrap().clone(),
                ),
                frames_to_live: 60,
            });
        }
    };
}
