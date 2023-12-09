use bevy::prelude::*;
use characters::ActionEvent;
use player_state::PlayerState;
use wag_core::{Facing, Player};

use crate::physics::ARENA_WIDTH;

#[derive(Debug, Component, Default)]
pub struct CameraWrapper;

pub const VIEWPORT_HALFWIDTH: f32 = 4.0; // This is used to control stage border relative to the camera
const CAMERA_CLAMP: f32 = ARENA_WIDTH - VIEWPORT_HALFWIDTH;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Update, (center_camera, camera_tilt).chain());
    }
}

fn add_camera(mut commands: Commands) {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Cameras"),
            CameraWrapper,
            CameraTilt::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 1.8, 5.0),
                    projection: PerspectiveProjection { ..default() }.into(),
                    ..default()
                },
                Name::new("Main Camera"),
            ));
        });
}

#[allow(clippy::type_complexity)]
fn center_camera(
    mut queryies: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<CameraWrapper>>,
    )>,
) {
    let player_pos_sum = queryies
        .p0()
        .iter()
        .fold(0.0, |acc, tf| acc + tf.translation.x)
        / 2.0;

    // Do some light lerping to make backthrows less jarring
    let mut camquery = queryies.p1();
    let mut tf = camquery.single_mut();
    let target = Vec3 {
        x: player_pos_sum.clamp(-CAMERA_CLAMP, CAMERA_CLAMP),
        ..tf.translation
    };
    tf.translation = tf.translation.lerp(target, 0.1);
}

#[derive(Debug, Component, Default, Deref, DerefMut)]
struct CameraTilt {
    velocity: Vec2,
}

const DAMPENING: f32 = 0.9;
const GRAVITY: f32 = 0.01;

fn camera_tilt(
    mut players: Query<(&mut PlayerState, &Facing)>,
    mut cams: Query<(&mut Transform, &mut CameraTilt), With<CameraWrapper>>,
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

    for (mut tf, mut tilt) in &mut cams {
        tilt.velocity *= DAMPENING;
        tilt.velocity += event_tilt;

        let current_euler_tuple = tf.rotation.to_euler(EulerRot::XYZ);
        let current_euler = Vec2::new(current_euler_tuple.1, current_euler_tuple.0);
        tilt.velocity -= current_euler * GRAVITY;

        tf.rotation = Quat::from_euler(
            EulerRot::XYZ,
            current_euler_tuple.0 + tilt.velocity.y,
            current_euler_tuple.1 + tilt.velocity.x,
            0.0,
        );
    }
}
