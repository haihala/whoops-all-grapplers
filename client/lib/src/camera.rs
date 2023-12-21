use bevy::{prelude::*, render::view::NoFrustumCulling};
use characters::ActionEvent;
use player_state::PlayerState;
use wag_core::{Facing, GameState, OnlyShowInGameState, Player, WagArgs};

use crate::physics::ARENA_WIDTH;

#[derive(Debug, Component, Default)]
pub struct CameraWrapper;

pub const VIEWPORT_HALFWIDTH: f32 = 4.0; // This is used to control stage border relative to the camera
const CAMERA_CLAMP: f32 = ARENA_WIDTH - VIEWPORT_HALFWIDTH;

// It never quite gets to either extreme because the math is fucked
const MAX_CAMERA_DISTANCE: f32 = 6.0;
const MIN_CAMERA_DISTANCE: f32 = 4.0;

const MAX_CAMERA_HEIGHT: f32 = 2.3;
const MIN_CAMERA_HEIGHT: f32 = 1.6;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .register_type::<RootCameraEffects>()
            .register_type::<ChildCameraEffects>()
            .add_systems(
                Update,
                (center_camera, camera_tilt, child_camera_effects).chain(),
            );
    }
}

fn add_camera(
    mut commands: Commands,
    args: Res<WagArgs>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Cameras"),
            CameraWrapper,
            RootCameraEffects::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Camera3dBundle {
                        transform: Transform::from_xyz(0.0, MAX_CAMERA_HEIGHT, MAX_CAMERA_DISTANCE),
                        projection: PerspectiveProjection::default().into(),
                        ..default()
                    },
                    Name::new("Main Camera"),
                    ChildCameraEffects::default(),
                ))
                .with_children(|main_cam| {
                    if !args.dev {
                        main_cam.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
                                material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
                                transform: Transform::from_xyz(0.0, 0.0, -2.0),
                                ..default()
                            },
                            OnlyShowInGameState(vec![GameState::Loading]),
                            NoFrustumCulling,
                        ));
                    };
                });
        });
}

#[allow(clippy::type_complexity)]
fn center_camera(
    mut queryies: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<CameraWrapper>>,
    )>,
    mut cam_zooms: Query<&mut ChildCameraEffects>,
) {
    let avg_player_x = queryies.p0().iter().map(|tf| tf.translation.x).sum::<f32>() / 2.0;

    let mut cam_zoom = cam_zooms.single_mut();
    cam_zoom.player_midpoint = avg_player_x;
    cam_zoom.player_distance = queryies
        .p0()
        .iter()
        .map(|tf| tf.translation.x)
        .reduce(|a, b| a - b)
        .unwrap()
        .abs();

    // Do some light lerping to make backthrows less jarring
    let mut camquery = queryies.p1();
    let mut tf = camquery.single_mut();
    let target = Vec3 {
        x: avg_player_x.clamp(-CAMERA_CLAMP, CAMERA_CLAMP),
        ..tf.translation
    };
    tf.translation = tf.translation.lerp(target, 0.1);
}

#[derive(Debug, Component, Default, Reflect)]
struct RootCameraEffects {
    tilt_velocity: Vec2,
}

const TILT_DAMPENING: f32 = 0.9;
const TILT_GRAVITY: f32 = 0.01;

fn camera_tilt(
    mut players: Query<(&mut PlayerState, &Facing)>,
    mut cams: Query<(&mut Transform, &mut RootCameraEffects), With<CameraWrapper>>,
) {
    let event_tilt = players
        .iter_mut()
        .flat_map(|(mut ps, facing)| {
            ps.drain_matching_actions(|a| {
                if let ActionEvent::CameraTilt(amount) = a {
                    Some(facing.mirror_vec2(amount.to_owned()))
                } else {
                    None
                }
            })
        })
        .fold(Vec2::ZERO, |acc, tilt| acc + tilt);

    let (mut tf, mut tilt) = cams.single_mut();

    tilt.tilt_velocity *= TILT_DAMPENING;
    tilt.tilt_velocity += event_tilt;

    let current_euler_tuple = tf.rotation.to_euler(EulerRot::XYZ);
    let current_euler = Vec2::new(current_euler_tuple.1, current_euler_tuple.0);
    tilt.tilt_velocity -= current_euler * TILT_GRAVITY;

    tf.rotation = Quat::from_euler(
        EulerRot::XYZ,
        current_euler_tuple.0 + tilt.tilt_velocity.y,
        current_euler_tuple.1 + tilt.tilt_velocity.x,
        0.0,
    );
}

#[derive(Debug, Component, Default, Reflect)]
struct ChildCameraEffects {
    last_shake_start: f32,
    player_distance: f32,
    player_midpoint: f32,
    pivot: Option<Vec3>,
}

const SHAKE_INITIAL_MAGNITUDE: f32 = 0.2;
const SHAKE_DURATION: f32 = 0.1;
const SHAKE_TWIST: f32 = 1000.0;

fn child_camera_effects(
    mut players: Query<&mut PlayerState>,
    mut cams: Query<(&mut Transform, &GlobalTransform, &mut ChildCameraEffects)>,
    time: Res<Time>,
) {
    let (mut tf, gtf, mut childcam_fx) = cams.single_mut();

    if childcam_fx.pivot.is_some() {
        // This does NOT go from 0-1, because various factors
        let ratio = childcam_fx.player_distance / ARENA_WIDTH;

        // These could live in a different system, but as the code here is quite simple,
        // I think using one function for all child cam things is easier to reason about (system execution order).
        let root_pos = gtf.translation() - tf.translation;

        childcam_fx.pivot = Some(Vec3::new(
            childcam_fx.player_midpoint - root_pos.x,
            MIN_CAMERA_HEIGHT * (1.0 - ratio) + MAX_CAMERA_HEIGHT * ratio,
            MIN_CAMERA_DISTANCE * (1.0 - ratio) + MAX_CAMERA_DISTANCE * ratio,
        ));
    } else {
        childcam_fx.pivot = Some(tf.translation);
    }

    if players
        .iter_mut()
        .flat_map(|mut ps| {
            ps.drain_matching_actions(|a| {
                if &ActionEvent::CameraShake == a {
                    Some(())
                } else {
                    None
                }
            })
        })
        .next()
        .is_some()
    {
        // Done after to avoid division by zero.
        childcam_fx.last_shake_start = time.elapsed_seconds();
    }

    let progress = (time.elapsed_seconds() - childcam_fx.last_shake_start) / SHAKE_DURATION;

    let magnitude = SHAKE_INITIAL_MAGNITUDE * (1.0 - progress.min(1.0));
    let angle = time.elapsed_seconds() * SHAKE_TWIST;
    let offset = magnitude * Vec3::new(angle.sin(), angle.cos(), 0.0);

    tf.translation = childcam_fx.pivot.unwrap() + offset;
}
