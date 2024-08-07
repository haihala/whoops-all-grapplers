mod player_velocity;
pub use player_velocity::PlayerVelocity;

use bevy::{ecs::query::QueryData, prelude::*};

use characters::{ActionEvent, Character};
use player_state::PlayerState;
use wag_core::{Area, Clock, Facing, Players, Stats, WAGStage};

use crate::{
    camera::{CameraWrapper, VIEWPORT_HALFWIDTH},
    damage::{HitTracker, HitboxSpawner},
};

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 9.5;

#[derive(Debug, Default, Reflect, Component)]
pub struct ConstantVelocity {
    pub shift: Vec3,
    pub speed: Vec3,
}
impl ConstantVelocity {
    pub fn new(speed: Vec3) -> ConstantVelocity {
        ConstantVelocity {
            speed,
            shift: speed / wag_core::FPS,
        }
    }
}

#[derive(Debug, Default, Reflect, Component, Deref, DerefMut)]
pub struct Pushbox(pub Area);

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, sideswitcher.before(WAGStage::Physics))
            .add_systems(
                FixedUpdate,
                (
                    player_input,
                    move_players,
                    push_players,
                    clamp_players,
                    move_constants,
                    player_gravity,
                )
                    .chain()
                    .in_set(WAGStage::Physics),
            );
    }
}

fn sideswitcher(players: Res<Players>, mut query: Query<(&Transform, &mut Facing)>) {
    if let Ok([(tf1, mut facing1), (tf2, mut facing2)]) =
        query.get_many_mut([players.one, players.two])
    {
        let p1_flipped = tf1.translation.x > tf2.translation.x;
        if facing1.to_flipped() != p1_flipped {
            facing1.set_flipped(p1_flipped);
            facing2.set_flipped(!p1_flipped);
        }
    }
}

fn player_gravity(
    clock: Res<Clock>,
    mut players: Query<(
        &mut PlayerVelocity,
        &mut PlayerState,
        &mut HitboxSpawner,
        &Transform,
        &Stats,
    )>,
) {
    for (mut velocity, mut state, mut spawner, tf, stats) in &mut players {
        if state.active_cinematic().is_some() {
            continue;
        }

        let is_airborne = tf.translation.y > GROUND_PLANE_HEIGHT;

        if is_airborne {
            velocity.add_impulse(-Vec2::Y * stats.gravity);

            if state.is_grounded() {
                state.jump();
            }
        } else if !state.is_grounded() && velocity.get_shift().y <= 0.0 {
            // Velocity check ensures that we don't call land on the frame we're being launched
            state.land(clock.frame);
            spawner.despawn_on_landing();
        }
    }
}

fn player_input(
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &mut PlayerVelocity, &Stats, &Facing)>,
) {
    for (mut state, mut velocity, status_effects, facing) in &mut query {
        if state.active_cinematic().is_some() {
            continue;
        }

        for _ in state.drain_matching_actions(|action| {
            if ActionEvent::ClearMovement == *action {
                Some(())
            } else {
                None
            }
        }) {
            velocity.clear_movements();
        }

        for movement in state.drain_matching_actions(|action| {
            if let ActionEvent::Movement(movement) = action {
                Some(movement.to_owned())
            } else {
                None
            }
        }) {
            velocity.handle_movement(clock.frame, *facing, movement);
        }

        if let Some(walk_direction) = state.get_walk_direction() {
            velocity.handle_walking_velocity(status_effects.walk_speed, *facing, walk_direction);
        } else if state.is_grounded() {
            velocity.drag();
        }

        velocity.cleanup_movements(clock.frame);
        velocity.sum_movements();
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct PlayerMovingQuery<'a> {
    character: &'a Character,
    tf: &'a mut Transform,
    state: &'a PlayerState,
    velocity: &'a mut PlayerVelocity,
    push_box: &'a Pushbox,
    facing: &'a Facing,
}

fn move_players(mut query: Query<PlayerMovingQuery>) {
    for mut p in &mut query {
        if p.state.active_cinematic().is_some() {
            continue;
        }
        p.tf.translation += p.velocity.get_shift().extend(0.0);
    }
}

fn push_players(mut query: Query<PlayerMovingQuery>, players: Res<Players>) {
    if let Ok([p1, p2]) = query.get_many_mut([players.one, players.two]) {
        if p1.state.active_cinematic().is_some() || p2.state.active_cinematic().is_some() {
            return;
        }

        if let Some(overlap) = p1
            .push_box
            .with_offset(p1.tf.translation.truncate())
            .intersection(&p2.push_box.with_offset(p2.tf.translation.truncate()))
        {
            // There is overlap.
            // The push amount is a very complicated bit of math that boils down to a rather simple code.
            // The idea is that the tops of the colliders are "slanted" in a 45 degree angle
            // After some triangle math with right angles, it turns out this is how much you need to move,
            // Given a certain overlap and collider sizes
            // This will work for the same size colliders, but it should work for different sizes as well.
            // Optimal x shift turns out to be:
            // overlap width+height-lower collider short side (usually width) halved
            // May god have mercy on the soul of whoever needs to rework this.

            let common_shift = overlap.width() + overlap.height();
            let average_collider_width = (p1.push_box.width() + p2.push_box.width()) / 2.0;

            for mut p in [p1, p2] {
                p.velocity.pushing = true;
                // This was originally +=, still seems like it ought to be, but this works now
                p.tf.translation -= Vec3::X
                    * p.facing.mirror_f32(
                        (common_shift - (average_collider_width / 2.0)).min(overlap.width()) / 2.0,
                    );
            }
        } else {
            for mut p in [p1, p2] {
                p.velocity.pushing = false;
            }
        }
    }
}

const CAMERA_EDGE_COLLISION_PADDING: f32 = 0.5;

#[allow(clippy::type_complexity)]
fn clamp_players(
    players: Res<Players>,
    mut queries: ParamSet<(
        Query<PlayerMovingQuery>,
        Query<&Transform, With<CameraWrapper>>,
    )>,
) {
    let camera_x = queries.p1().get_single().unwrap().translation.x;
    let left_border = camera_x - VIEWPORT_HALFWIDTH + CAMERA_EDGE_COLLISION_PADDING;
    let right_border = camera_x + VIEWPORT_HALFWIDTH - CAMERA_EDGE_COLLISION_PADDING;

    if let Ok([mut p1, mut p2]) = queries.p0().get_many_mut([players.one, players.two]) {
        // Clamp y (prevent falling through floor)
        for p in [&mut p1, &mut p2] {
            if p.tf.translation.y < GROUND_PLANE_HEIGHT {
                p.tf.translation.y = GROUND_PLANE_HEIGHT;
                p.velocity.y_collision();
            }
        }

        if p1.state.active_cinematic().is_some() || p2.state.active_cinematic().is_some() {
            return;
        }

        // Either neither or both should be pushing
        assert!(p1.velocity.pushing == p2.velocity.pushing);
        let pushing = p1.velocity.pushing || p2.velocity.pushing;

        // Clamp x
        let p1_x_clamp = get_x_clamp(
            p1.push_box.with_offset(p1.tf.translation.truncate()),
            left_border,
            right_border,
        );
        let p2_x_clamp = get_x_clamp(
            p2.push_box.with_offset(p2.tf.translation.truncate()),
            left_border,
            right_border,
        );

        if pushing && (p1_x_clamp.is_some() || p2_x_clamp.is_some()) {
            // Apply shift to both, one jumped over the other into a corner
            let amount = if let Some(value) = p1_x_clamp {
                value
            } else {
                p2_x_clamp.unwrap()
            };

            for mut p in [p1, p2] {
                p.tf.translation.x += amount;
                p.velocity.x_collision()
            }
        } else {
            // apply shifts individually
            if let Some(amount) = p1_x_clamp {
                p1.tf.translation.x += amount
            }
            if let Some(amount) = p2_x_clamp {
                p2.tf.translation.x += amount
            }
        }
    }
}

const WALL_SLOPE: f32 = 0.01;

fn get_x_clamp(collider: Area, left_border: f32, right_border: f32) -> Option<f32> {
    // Borders are a tad diagonal to permit jumping over someone into the corner
    let left_target = left_border - collider.bottom() * WALL_SLOPE;
    let right_target = right_border + collider.bottom() * WALL_SLOPE;

    if collider.left() < left_target {
        Some(left_target - collider.left())
    } else if collider.right() > right_target {
        Some(right_target - collider.right())
    } else {
        None
    }
}

fn move_constants(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        Entity,
        &ConstantVelocity,
        Option<&HitTracker>,
        &mut Transform,
    )>,
) {
    for (entity, velocity, hit_tracker, mut transform) in &mut query {
        if hit_tracker
            .map(|tracker| !tracker.active(clock.frame))
            .unwrap_or(false)
        {
            continue;
        }

        transform.translation += velocity.shift;

        // Despawn the thing if it's outside of the arena
        if transform.translation.length() > ARENA_WIDTH + 10.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
