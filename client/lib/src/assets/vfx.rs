use bevy::prelude::*;

use foundation::{CharacterFacing, Clock, Icons, MatchState, VfxRequest, VisualEffect, FPS};

use crate::{
    assets::materials::{BezierSwooshMaterial, JackpotRingMaterial},
    entity_management::DespawnMarker,
    event_spreading::{SpawnRelativeVfx, SpawnVfx},
    movement::Follow,
};

use super::materials::{
    BlankMaterial, BlockEffectMaterial, ClashSparkMaterial, DiagonalWaveMaterial, FlatWaveMaterial,
    FocalPointLinesMaterial, HitSparkMaterial, IconFlashMaterial, LightningBoltMaterial,
    LineFieldMaterial, MidFlashMaterial, PebbleMaterial, RingRippleMaterial, SmokeBombMaterial,
    SparkBurstMaterial,
};

pub fn spawn_vfx<M: Material + std::fmt::Debug>(
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
        follow,
    } = trigger.event();

    let mut ent_cmds = commands.spawn((
        Mesh3d(mesh.clone()),
        *transform,
        MeshMaterial3d(material_asset.add(material.clone())),
        DespawnMarker(clock.frame + frames_to_live),
        StateScoped(MatchState::Combat),
        Name::new("VFX"),
    ));

    if let Some(component) = follow {
        ent_cmds.insert(*component);
    }
}

pub fn start_relative_vfx(
    trigger: Trigger<SpawnRelativeVfx>,
    query: Query<(Entity, &Transform, &CharacterFacing)>,
    mut commands: Commands,
) {
    let (entity, tf, facing) = query.get(trigger.target()).unwrap();
    let mut request = trigger.event().0.clone();
    if facing.visual.to_flipped() {
        request.mirror = !request.mirror;
    }
    request.tf.translation += tf.translation;
    commands.trigger(SpawnVfx(request, Some(entity)));
}

// Bevy allows for a max of N system params, which means we need to split the vfx spawning after we
// have more than N types of vfx (we do)
#[derive(Debug, Event)]
pub struct MaxSystemParamCountFix<T: Material> {
    mesh: Handle<Mesh>,
    transform: Transform,
    material: T,
    frames_to_live: usize,
    follow: Option<Follow>,
}

pub fn start_absolute_vfx(
    trigger: Trigger<SpawnVfx>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    icons: Res<Icons>,
) {
    let SpawnVfx(VfxRequest { effect, tf, mirror }, maybe_player) = trigger.event();

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
            follow: None,
        }),
        VisualEffect::Hit => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: HitSparkMaterial::new(time.elapsed_secs()),
                frames_to_live: 10,
                follow: None,
            });
        }
        VisualEffect::Clash => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: ClashSparkMaterial::new(time.elapsed_secs()),
                frames_to_live: 10,
                follow: None,
            });
        }
        VisualEffect::Block => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: BlockEffectMaterial::new(time.elapsed_secs()),
                frames_to_live: 10,
                follow: None,
            });
        }
        VisualEffect::RingPulse(pulse) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: RingRippleMaterial {
                    base_color: pulse.base_color.into(),
                    edge_color: pulse.edge_color.into(),
                    start_time: time.elapsed_secs(),
                    rings: pulse.rings,
                    duration: pulse.duration,
                    ring_thickness: pulse.thickness,
                    offset: pulse.offset,
                },
                frames_to_live: (pulse.duration * FPS) as usize,
                follow: None,
            });
        }
        VisualEffect::SpeedLines => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: LineFieldMaterial::new(time.elapsed_secs(), *mirror),
                frames_to_live: 20,
                follow: None,
            });
        }
        VisualEffect::ThrowTarget => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: FocalPointLinesMaterial::new(time.elapsed_secs()),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::Lightning => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: LightningBoltMaterial::new(time.elapsed_secs(), *mirror),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::WaveDiagonal(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: DiagonalWaveMaterial::new(time.elapsed_secs(), *color, *mirror),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::WaveFlat(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: FlatWaveMaterial::new(time.elapsed_secs(), *color, *mirror),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::Pebbles => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: PebbleMaterial::new(time.elapsed_secs(), *mirror),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::Sparks => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: SparkBurstMaterial::new(time.elapsed_secs(), *mirror),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::OpenerSpark(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: MidFlashMaterial::new(time.elapsed_secs(), *color),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::SmokeBomb => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: SmokeBombMaterial::new(time.elapsed_secs()),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::Icon(icon) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: IconFlashMaterial::new(
                    time.elapsed_secs(),
                    icons.0.get(icon).unwrap().clone(),
                ),
                frames_to_live: 60,
                follow: None,
            });
        }
        VisualEffect::JackpotRing => {
            commands.trigger(MaxSystemParamCountFix {
                mesh: {
                    let size = VisualEffect::JackpotRing.mesh_size().size();
                    meshes.add(Cylinder::new(size.x / 2.0, size.y).mesh())
                },
                transform,
                material: JackpotRingMaterial {
                    start_time: time.elapsed_secs(),
                    ..default()
                },
                frames_to_live: 120,
                follow: Some(Follow {
                    target: maybe_player.unwrap(),
                    offset: Vec3::Y * 1.5,
                }),
            });
        }
        VisualEffect::Smear(smear) => {
            let padding = std::iter::repeat_n(Vec3::default(), 16 - smear.control_points.len());
            let curves = ((smear.control_points.len() - 4) / 3 + 1) as u32;
            let control_points: [Vec4; 16] = smear
                .control_points
                .clone()
                .into_iter()
                .chain(padding)
                .map(|v| v.extend(0.0))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let inner_duration = smear.duration as f32 / FPS;

            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                material: BezierSwooshMaterial {
                    primary_color: smear.primary_color.into(),
                    secondary_color: smear.secondary_color.into(),
                    duration: inner_duration,
                    curves,
                    control_points,
                    start_time: time.elapsed_secs(),
                },
                frames_to_live: smear.duration,
                follow: None,
            });
        }
    };
}
