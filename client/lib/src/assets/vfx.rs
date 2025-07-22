use std::marker::PhantomData;

use bevy::prelude::*;

use foundation::{
    CharacterClock, CharacterFacing, Clock, Icons, MatchState, Owner, Players, RollbackSchedule,
    SystemStep, VfxRequest, VisualEffect, FPS,
};

use crate::{
    assets::materials::{AnimatedMaterial, BezierSwooshMaterial, JackpotRingMaterial},
    camera::FOREGROUND,
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

#[derive(Debug)]
pub struct VfxMaterialPlugin<M: Material + AnimatedMaterial + std::fmt::Debug> {
    pd: PhantomData<M>,
}

impl<M: Material + AnimatedMaterial + std::fmt::Debug> Default for VfxMaterialPlugin<M> {
    fn default() -> Self {
        Self {
            pd: Default::default(),
        }
    }
}
impl<M> Plugin for VfxMaterialPlugin<M>
where
    M: Material + AnimatedMaterial + std::fmt::Debug,
    M::Data: PartialEq + Eq + std::hash::Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<M>::default())
            .add_observer(spawn_vfx::<M>)
            .add_systems(
                RollbackSchedule,
                update_animated_material::<M>.in_set(SystemStep::Presentation),
            );
    }
}

fn spawn_vfx<M: Material + std::fmt::Debug>(
    trigger: Trigger<MaxSystemParamCountFix<M>>,
    mut commands: Commands,
    mut material_asset: ResMut<Assets<M>>,
    rel_clocks: Query<&CharacterClock>,
    abs_clock: Res<Clock>,
    players: Res<Players>,
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
        StateScoped(MatchState::Combat),
        Name::new("VFX"),
        FOREGROUND,
    ));

    if let Some(component) = follow {
        let player = players.get_inverse(component.target);
        let rel_clock = rel_clocks.get(component.target).unwrap();
        ent_cmds.insert((
            *component,
            Owner(player),
            DespawnMarker::Relative(rel_clock.frame + frames_to_live, player),
        ));
    } else {
        ent_cmds.insert(DespawnMarker::Absolute(abs_clock.frame + frames_to_live));
    }
}

fn update_animated_material<M: Material + AnimatedMaterial + std::fmt::Debug>(
    mut material_assets: ResMut<Assets<M>>,
    materials: Query<(Option<&Owner>, &MeshMaterial3d<M>)>,
    characters: Query<&CharacterClock>,
    players: Res<Players>,
) {
    for (id, asset) in material_assets.iter_mut() {
        for (maybe_owner, mesh_mat) in materials {
            if mesh_mat.0.id() == id {
                if let Some(owner) = maybe_owner {
                    let player_ent = players.get(**owner);
                    let clock = characters.get(player_ent).unwrap();
                    if clock.hitstop_frames > 0 {
                        // Owner of this material is in hitstop, don't update
                        break;
                    }
                }

                asset.bump_frame();
            }
        }
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
    request.tf.translation = tf.translation + facing.visual.mirror_vec3(request.tf.translation);
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
    mut meshes: ResMut<Assets<Mesh>>,
    icons: Res<Icons>,
) {
    let SpawnVfx(
        VfxRequest {
            effect,
            tf,
            mirror,
            follow_offset,
        },
        maybe_player,
    ) = trigger.event();

    let mesh = meshes.add(effect.mesh_size());
    let transform = Transform {
        translation: tf.translation + 0.1 * Vec3::Z,
        ..*tf
    };
    let follow = follow_offset.map(|offset| Follow {
        target: maybe_player.unwrap(),
        offset,
    });

    match effect {
        VisualEffect::Blank => commands.trigger(MaxSystemParamCountFix {
            mesh,
            transform,
            follow,
            material: BlankMaterial::default(),
            frames_to_live: 15,
        }),
        VisualEffect::Hit => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: HitSparkMaterial::default(),
                frames_to_live: 10,
            });
        }
        VisualEffect::Clash => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: ClashSparkMaterial::default(),
                frames_to_live: 10,
            });
        }
        VisualEffect::Block => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: BlockEffectMaterial::default(),
                frames_to_live: 10,
            });
        }
        VisualEffect::RingPulse(pulse) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: RingRippleMaterial {
                    base_color: pulse.base_color.into(),
                    edge_color: pulse.edge_color.into(),
                    rings: pulse.rings,
                    duration: pulse.duration,
                    ring_thickness: pulse.thickness,
                    offset: pulse.offset,
                    ..default()
                },
                frames_to_live: (pulse.duration * FPS) as usize,
            });
        }
        VisualEffect::SpeedLines => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: LineFieldMaterial::new(*mirror),
                frames_to_live: 20,
            });
        }
        VisualEffect::ThrowTarget => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: FocalPointLinesMaterial::default(),
                frames_to_live: 60,
            });
        }
        VisualEffect::Lightning => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: LightningBoltMaterial::new(*mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::WaveDiagonal(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: DiagonalWaveMaterial::new(*color, *mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::WaveFlat(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: FlatWaveMaterial::new(*color, *mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::Pebbles => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: PebbleMaterial::new(*mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::Sparks => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: SparkBurstMaterial::new(*mirror),
                frames_to_live: 60,
            });
        }
        VisualEffect::OpenerSpark(color) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: MidFlashMaterial::new(*color),
                frames_to_live: 60,
            });
        }
        VisualEffect::SmokeBomb => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: SmokeBombMaterial::default(),
                frames_to_live: 60,
            });
        }
        VisualEffect::Icon(icon) => {
            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: IconFlashMaterial::new(icons.0.get(icon).unwrap().clone()),
                frames_to_live: 60,
            });
        }
        VisualEffect::JackpotRing => {
            commands.trigger(MaxSystemParamCountFix {
                mesh: {
                    let size = VisualEffect::JackpotRing.mesh_size().size();
                    meshes.add(Cylinder::new(size.x / 2.0, size.y).mesh())
                },
                transform,
                follow,
                material: JackpotRingMaterial::default(),
                frames_to_live: 120,
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
                .map(|v| {
                    if *mirror {
                        Vec3::new(-v.x, v.y, v.z)
                    } else {
                        v
                    }
                    .extend(0.0)
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            commands.trigger(MaxSystemParamCountFix {
                mesh,
                transform,
                follow,
                material: BezierSwooshMaterial {
                    primary_color: smear.primary_color.into(),
                    secondary_color: smear.secondary_color.into(),
                    duration: smear.duration,
                    overlap: smear.overlap,
                    curves,
                    control_points,
                    ..default()
                },
                frames_to_live: smear.duration as usize,
            });
        }
    };
}
