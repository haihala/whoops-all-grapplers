mod cinematic_locks;
mod condition_management;
mod force_stand;
mod move_activation;
mod move_advancement;
mod player_flash;
mod recovery;
mod side_switcher;
mod size_adjustment;

use characters::{dummy, mizku, Hurtboxes, Inventory, WAGResources};
use input_parsing::{InputParser, PadBundle};
use player_state::PlayerState;
use wag_core::{
    AnimationType, AvailableCancels, CharacterId, Characters, Clock, Facing, InMatch, MatchState,
    Player, Players, RollbackSchedule, Stats, WAGStage,
};

use crate::{
    assets::{AnimationHelper, AnimationHelperSetup, Models, PlayerModelHook},
    damage::HitboxSpawner,
    event_spreading,
    movement::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub use move_activation::MoveBuffer;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)

pub struct PlayerStateManagementPlugin;

impl Plugin for PlayerStateManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MatchState::Loading), setup_players)
            .add_systems(OnEnter(MatchState::PreRound), setup_combat)
            .add_systems(
                RollbackSchedule,
                side_switcher::sideswitcher.in_set(WAGStage::HouseKeeping),
            )
            .add_systems(
                RollbackSchedule,
                condition_management::update_combined_status_effect
                    .after(WAGStage::MovePipeline)
                    .before(WAGStage::PlayerUpdates),
            )
            .add_systems(
                RollbackSchedule,
                (
                    move_activation::manage_buffer,
                    move_activation::plain_start,
                    move_activation::cancel_start,
                    move_activation::move_activator,
                    move_advancement::move_advancement,
                )
                    .chain()
                    .in_set(WAGStage::MovePipeline),
            )
            .add_systems(
                RollbackSchedule,
                (
                    cinematic_locks::handle_cinematics, // This being the first system after hit move advancement is important
                    recovery::stun_recovery,
                    recovery::ground_recovery,
                    size_adjustment::update_box_sizes_from_state,
                    size_adjustment::remove_old_hurtbox_expansions,
                )
                    .chain()
                    .in_set(WAGStage::PlayerUpdates),
            );
    }
}

fn setup_players(mut commands: Commands, characters: Res<Characters>, models: Res<Models>) {
    let players = Players {
        one: spawn_player(
            &mut commands,
            &models,
            -PLAYER_SPAWN_DISTANCE,
            Player::One,
            characters.p1,
        ),
        two: spawn_player(
            &mut commands,
            &models,
            PLAYER_SPAWN_DISTANCE,
            Player::Two,
            characters.p2,
        ),
    };

    commands.insert_resource(players);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    inventory: Inventory,
    spawner: HitboxSpawner,
    player_velocity: PlayerVelocity,
    move_buffer: MoveBuffer,
    status_effects: Stats,
    available_cancels: AvailableCancels,
    state: PlayerState,
}

fn spawn_player(
    commands: &mut Commands,
    models: &Models,
    offset: f32,
    player: Player,
    character_id: CharacterId,
) -> Entity {
    let character = match character_id {
        CharacterId::Dummy => dummy(),
        CharacterId::Mizku => mizku(),
    };

    let colors = character.colors[&player].clone();
    let model = character.model;

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation((offset, GROUND_PLANE_HEIGHT, 0.0).into()),
                ..default()
            },
            WAGResources::from_stats(&character.base_stats, character.special_properties.clone()),
            PlayerDefaults::default(),
            PadBundle::new(character.get_inputs()),
            Name::new(format!("Player {player}")),
            AnimationHelperSetup(character.generic_animations[&AnimationType::Default]),
            Facing::from_flipped(offset.is_sign_positive()),
            Pushbox(character.boxes.standing.pushbox),
            Hurtboxes::from(character.boxes.standing),
            character,
            player,
            StateScoped(InMatch),
        ))
        .with_children(move |parent| {
            parent.spawn((
                PlayerModelHook(colors.clone()),
                SceneBundle {
                    scene: models[&model].clone(),
                    ..default()
                },
            ));
        })
        .add_rollback()
        .observe(event_spreading::spread_events)
        .observe(cinematic_locks::start_lock)
        .observe(condition_management::manage_conditions)
        .observe(force_stand::force_stand)
        .observe(move_activation::automatic_activation)
        .observe(move_activation::manage_cancel_windows)
        .observe(move_advancement::end_moves)
        .observe(player_flash::handle_flash_events)
        .observe(size_adjustment::expand_hurtboxes)
        .observe(crate::assets::start_animation)
        .observe(crate::assets::start_relative_vfx)
        .observe(crate::assets::play_voiceline)
        .observe(crate::camera::tilt_camera)
        .observe(crate::damage::snap_and_switch)
        .observe(crate::damage::hitstun_events)
        .observe(crate::damage::blockstun_events)
        .observe(crate::damage::launch_events)
        .observe(crate::damage::spawn_hitbox)
        .observe(crate::movement::clear_movement)
        .observe(crate::movement::add_movement)
        .observe(crate::resources::modify_properties)
        .observe(crate::resources::clear_properties)
        .id()
}

#[allow(clippy::type_complexity)]
fn setup_combat(
    mut query: Query<(
        &Player,
        &Stats,
        &mut WAGResources,
        &mut Transform,
        &mut PlayerState,
        &mut MoveBuffer,
        &mut InputParser,
        &mut PlayerVelocity,
        &mut AnimationHelper,
    )>,
    mut clock: ResMut<Clock>,
    bevy_time: Res<Time>,
) {
    println!("Reset");
    clock.reset(bevy_time.elapsed_seconds_f64());

    for (
        player,
        stats,
        mut resources,
        mut tf,
        mut player_state,
        mut buffer,
        mut parser,
        mut velocity,
        mut animation_helper,
    ) in &mut query
    {
        resources.reset(stats);
        player_state.reset();
        buffer.clear_all();
        parser.clear();
        velocity.reset();
        animation_helper.reset();

        tf.translation = Vec3::new(
            match *player {
                Player::One => -PLAYER_SPAWN_DISTANCE,
                Player::Two => PLAYER_SPAWN_DISTANCE,
            },
            GROUND_PLANE_HEIGHT,
            0.0,
        );
    }
}
