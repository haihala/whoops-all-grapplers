mod player_velocity;
pub use player_velocity::PlayerVelocity;

use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_inspector_egui::Inspectable;

use constants::PLAYER_GRAVITY_PER_FRAME;
use kits::Kit;
use player_state::PlayerState;
use time::{once_per_combat_frame, WAGStage};
use types::{Area, Facing, Players};

use crate::{
    camera::{WorldCamera, VIEWPORT_HALFWIDTH},
    spawner::Spawner,
};

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 10.0;

#[derive(Debug, Default, Inspectable, Component)]
pub struct ConstantVelocity {
    pub shift: Vec3,
    pub speed: Vec3,
}
impl ConstantVelocity {
    pub fn new(speed: Vec3) -> ConstantVelocity {
        ConstantVelocity {
            speed,
            shift: speed / constants::FPS,
        }
    }
}

#[derive(Debug, Default, Inspectable, Component, Deref, DerefMut)]
pub struct PushBox(pub Area);

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            WAGStage::Physics,
            SystemSet::new()
                .with_run_criteria(once_per_combat_frame)
                .with_system(sideswitcher)
                .with_system(player_gravity.after(sideswitcher))
                .with_system(player_input.after(player_gravity))
                .with_system(move_players.after(player_input))
                .with_system(push_players.after(move_players))
                .with_system(clamp_players.after(push_players))
                .with_system(move_constants.after(clamp_players)),
        );
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct SideswitcherQuery<'a> {
    tf: &'a Transform,
    state: &'a PlayerState,
    direction: &'a mut Facing,
}
fn sideswitcher(players: Res<Players>, mut query: Query<SideswitcherQuery>) {
    if let Ok([mut p1, mut p2]) = query.get_many_mut([players.one, players.two]) {
        if p1.state.is_grounded() && p2.state.is_grounded() {
            let p1_flipped = p1.tf.translation.x > p2.tf.translation.x;
            p1.direction.set_flipped(p1_flipped);
            p2.direction.set_flipped(!p1_flipped);
        }
    }
}

fn player_gravity(
    mut commands: Commands,
    mut players: Query<(
        &mut PlayerVelocity,
        &mut PlayerState,
        &mut Spawner,
        &Transform,
    )>,
) {
    for (mut velocity, mut state, mut spawner, tf) in players.iter_mut() {
        let is_airborne = tf.translation.y > GROUND_PLANE_HEIGHT;

        if is_airborne {
            velocity.add_impulse(-Vec3::Y * PLAYER_GRAVITY_PER_FRAME);

            if state.is_grounded() {
                state.jump();
            }
        } else if !state.is_grounded() {
            state.land();
            spawner.despawn_on_phase_change(&mut commands);
        }
    }
}

fn player_input(mut query: Query<(&PlayerState, &mut PlayerVelocity, &Kit, &Facing)>) {
    for (state, mut velocity, kit, facing) in query.iter_mut() {
        if let Some((move_id, start_frame, mobility)) =
            state.get_move_state().and_then(|move_state| {
                // Some if a move is happening
                kit.get_move(move_state.move_id)
                    .get_action(move_state)
                    .unwrap()
                    .0
                    .get_mobility()
                    .map(|mobility| (move_state.move_id, move_state.start_frame, mobility))
            })
        {
            velocity.handle_move_velocity(move_id, start_frame, mobility, facing);
        } else {
            velocity.current_move = None;

            if let Some(walk_direction) = state.get_walk_direction() {
                velocity.handle_walking_velocity(walk_direction);
            }
        }

        if state.is_grounded() {
            velocity.drag();
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct PlayerMovingQuery<'a> {
    kit: &'a Kit,
    tf: &'a mut Transform,
    state: &'a PlayerState,
    velocity: &'a mut PlayerVelocity,
    push_box: &'a PushBox,
    facing: &'a Facing,
}

fn move_players(mut query: Query<PlayerMovingQuery>) {
    for mut p in query.iter_mut() {
        p.tf.translation += p.velocity.get_shift();
    }
}

fn push_players(mut query: Query<PlayerMovingQuery>, players: Res<Players>) {
    if let Ok([p1, p2]) = query.get_many_mut([players.one, players.two]) {
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
                        ((common_shift - (average_collider_width / 2.0)) / 2.0)
                            .min(overlap.width()),
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
        Query<&Transform, With<WorldCamera>>,
    )>,
) {
    let camera_x = queries.p1().get_single().unwrap().translation.x;
    let left_border = camera_x - VIEWPORT_HALFWIDTH + CAMERA_EDGE_COLLISION_PADDING;
    let right_border = camera_x + VIEWPORT_HALFWIDTH - CAMERA_EDGE_COLLISION_PADDING;

    if let Ok([mut p1, mut p2]) = queries.p0().get_many_mut([players.one, players.two]) {
        // Either neither or both should be pushing
        assert!(p1.velocity.pushing == p2.velocity.pushing);
        let pushing = p1.velocity.pushing || p2.velocity.pushing;

        // Clamp y
        for p in [&mut p1, &mut p2] {
            if p.tf.translation.y < GROUND_PLANE_HEIGHT {
                p.tf.translation.y = GROUND_PLANE_HEIGHT;
                p.velocity.y_collision();
            }
        }

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
fn get_x_clamp(collider: Area, left_border: f32, right_border: f32) -> Option<f32> {
    if collider.left() < left_border {
        Some(left_border - collider.left())
    } else if collider.right() > right_border {
        Some(right_border - collider.right())
    } else {
        None
    }
}

fn move_constants(
    mut commands: Commands,
    mut query: Query<(Entity, &ConstantVelocity, &mut Transform)>,
) {
    // Handle static collision
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.shift;

        // Despawn the thing if it's outside of the arena
        if transform.translation.length() > ARENA_WIDTH + 10.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
