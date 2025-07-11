use bevy::{prelude::*, render::view::NoFrustumCulling};
use foundation::{
    CharacterFacing, InMatch, MatchState, Player, RollbackSchedule, SystemStep, WagArgs,
    LOADING_SCREEN_BACKGROUND,
};

use crate::{
    entity_management::VisibleInStates,
    event_spreading::{ShakeCamera, TiltCamera, ZoomCamera},
    movement::{ARENA_WIDTH, MAX_PLAYER_DISTANCE},
};

#[derive(Debug, Component, Default)]
#[require(Transform, Visibility)]
pub struct CameraWrapper;

pub const VIEWPORT_HALFWIDTH: f32 = MAX_PLAYER_DISTANCE / 2.0; // This is used to control stage border relative to the camera

// It never quite gets to either extreme because the math is fucked
const MAX_CAMERA_DISTANCE: f32 = 6.0;
const MIN_CAMERA_DISTANCE: f32 = 4.0;

const MAX_CAMERA_HEIGHT: f32 = 2.3;
const MIN_CAMERA_HEIGHT: f32 = 1.6;

const CAMERA_CLAMP: f32 =
    ARENA_WIDTH - VIEWPORT_HALFWIDTH * (MIN_CAMERA_DISTANCE / MAX_CAMERA_DISTANCE);

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RootCameraEffects>()
            .register_type::<ChildCameraEffects>()
            .add_systems(Startup, add_camera)
            .add_systems(
                RollbackSchedule,
                (center_camera, reset_camera_tilt, child_camera_effects)
                    .chain()
                    .in_set(SystemStep::Camera)
                    .run_if(in_state(InMatch)),
            )
            .add_observer(shake_camera)
            .add_observer(zoom_camera);
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
            Name::new("Cameras"),
            CameraWrapper,
            RootCameraEffects::default(),
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Camera3d::default(),
                    Transform::from_xyz(0.0, MAX_CAMERA_HEIGHT, MAX_CAMERA_DISTANCE),
                    Projection::Perspective(PerspectiveProjection::default()),
                    Name::new("Main Camera"),
                    ChildCameraEffects::default(),
                    InheritedVisibility::VISIBLE,
                ))
                .with_children(|main_cam| {
                    if args.dev.is_none() {
                        // This blocks the view while game is loading
                        main_cam.spawn((
                            Mesh3d(meshes.add(Mesh::from(Cuboid {
                                half_size: Vec3::splat(3.0),
                            }))),
                            MeshMaterial3d(materials.add(LOADING_SCREEN_BACKGROUND)),
                            Transform::from_xyz(0.0, 0.0, -2.0),
                            VisibleInStates(vec![MatchState::Loading]),
                            NoFrustumCulling,
                        ));
                    };
                });
        });
}

#[allow(clippy::type_complexity)]
fn center_camera(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<CameraWrapper>>,
    )>,
    mut cam_zooms: Query<&mut ChildCameraEffects>,
) {
    let avg_player_x = queries.p0().iter().map(|tf| tf.translation.x).sum::<f32>() / 2.0;

    let mut cam_zoom = cam_zooms.single_mut().unwrap();
    cam_zoom.player_midpoint = avg_player_x;
    cam_zoom.player_distance = queries
        .p0()
        .iter()
        .map(|tf| tf.translation.x)
        .reduce(|a, b| a - b)
        .unwrap_or_default()
        .abs();

    // Do some light lerping to make backthrows less jarring
    let mut camquery = queries.p1();
    let mut tf = camquery.single_mut().unwrap();
    let target = Vec3 {
        x: avg_player_x.clamp(-CAMERA_CLAMP, CAMERA_CLAMP),
        ..tf.translation
    };
    tf.translation = tf.translation.lerp(target, 0.1);
}

#[derive(Debug, Component, Default, Reflect, Clone, Copy)]
pub struct RootCameraEffects {
    tilt_velocity: Vec2,
}

const TILT_DAMPENING: f32 = 0.9;
const TILT_GRAVITY: f32 = 0.01;

pub fn tilt_camera(
    trigger: Trigger<TiltCamera>,
    mut cams: Query<&mut RootCameraEffects>,
    players: Query<&CharacterFacing>,
) {
    let mut tilt = cams.single_mut().unwrap();
    let facing = players.get(trigger.target()).unwrap();
    tilt.tilt_velocity += facing.visual.mirror_vec2(trigger.event().0);
}

fn reset_camera_tilt(
    mut cams: Query<(&mut Transform, &mut RootCameraEffects), With<CameraWrapper>>,
) {
    let (mut tf, mut tilt) = cams.single_mut().unwrap();

    tilt.tilt_velocity *= TILT_DAMPENING;

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

#[derive(Debug, Component, Default, Reflect, Clone, Copy)]
pub struct ChildCameraEffects {
    last_shake_start: f32,
    player_distance: f32,
    player_midpoint: f32,
    zoom_until: f32,
    pivot: Option<Vec3>,
}

const SHAKE_INITIAL_MAGNITUDE: f32 = 0.2;
const SHAKE_DURATION: f32 = 0.1;
const SHAKE_TWIST: f32 = 1000.0;

fn shake_camera(
    _trigger: Trigger<ShakeCamera>,
    mut cams: Query<&mut ChildCameraEffects>,
    time: Res<Time>,
) {
    let mut childcam_fx = cams.single_mut().unwrap();
    // Done after to avoid division by zero.
    childcam_fx.last_shake_start = time.elapsed_secs();
}

fn zoom_camera(
    trigger: Trigger<ZoomCamera>,
    mut cams: Query<&mut ChildCameraEffects>,
    time: Res<Time>,
) {
    let mut childcam_fx = cams.single_mut().unwrap();
    childcam_fx.zoom_until = time.elapsed_secs() + trigger.event().0;
}

fn child_camera_effects(
    mut cams: Query<(&mut Transform, &mut ChildCameraEffects)>,
    time: Res<Time>,
) {
    let (mut tf, mut childcam_fx) = cams.single_mut().unwrap();

    let translation = if childcam_fx.pivot.is_some() {
        let zoomed = childcam_fx.zoom_until > time.elapsed_secs();

        // TODO: Smoother transitions
        if zoomed {
            Vec3::new(0.0, MIN_CAMERA_HEIGHT, MIN_CAMERA_DISTANCE) * 0.7
        } else {
            // This does NOT go from 0-1, because various factors
            let ratio = childcam_fx.player_distance / ARENA_WIDTH;

            Vec3::new(
                0.0,
                MIN_CAMERA_HEIGHT * (1.0 - ratio) + MAX_CAMERA_HEIGHT * ratio,
                MIN_CAMERA_DISTANCE * (1.0 - ratio) + MAX_CAMERA_DISTANCE * ratio,
            )
        }
    } else {
        tf.translation
    };

    childcam_fx.pivot = Some(translation);

    let progress = (time.elapsed_secs() - childcam_fx.last_shake_start) / SHAKE_DURATION;
    let magnitude = SHAKE_INITIAL_MAGNITUDE * (1.0 - progress).max(0.0);
    let angle = time.elapsed_secs() * SHAKE_TWIST;
    let offset = magnitude * Vec3::new(angle.sin(), angle.cos(), 0.0);

    tf.translation = childcam_fx.pivot.unwrap() + offset;
}

#[allow(clippy::type_complexity)]
pub fn reset_camera(
    mut queries: ParamSet<(
        Single<(&mut Transform, &mut RootCameraEffects), With<CameraWrapper>>,
        Single<(&mut Transform, &mut ChildCameraEffects)>,
    )>,
) {
    let (mut root_tf, mut root_cam_effects) = queries.p0().into_inner();
    *root_tf = Transform::default();
    *root_cam_effects = RootCameraEffects::default();

    let (mut child_tf, mut child_cam_effects) = queries.p1().into_inner();
    *child_tf = Transform::from_xyz(0.0, MAX_CAMERA_HEIGHT, MAX_CAMERA_DISTANCE);
    *child_cam_effects = ChildCameraEffects::default();
}
