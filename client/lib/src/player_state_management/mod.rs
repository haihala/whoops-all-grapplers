mod cinematic_locks;
mod condition_management;
mod move_activation;
mod move_advancement;
mod player_flash;
mod recovery;
mod side_switcher;
mod size_adjustment;

use characters::{dummy, mizku, Inventory, WAGResources};
use input_parsing::{InputParser, PadBundle};
use player_state::PlayerState;
use wag_core::{
    AnimationType, CharacterId, Characters, Clock, Facing, GameState, InMatch, Joints, Player,
    Players, Stats, WAGStage,
};

use crate::{
    assets::{AnimationHelper, AnimationHelperSetup, Models, PlayerModelHook},
    damage::{Defense, HitboxSpawner},
    movement::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::prelude::*;

pub use move_activation::MoveBuffer;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

pub struct PlayerStateManagementPlugin;

impl Plugin for PlayerStateManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_players)
            .add_systems(OnEnter(GameState::PreRound), setup_combat)
            // This is here so it's up to date when the round starts
            .add_systems(
                FixedUpdate,
                (
                    condition_management::update_combined_status_effect
                        .before(WAGStage::PlayerUpdates),
                    player_flash::handle_flash_events,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    move_activation::manage_buffer,
                    move_activation::automatic_activation,
                    move_activation::plain_start,
                    move_activation::special_cancel,
                    move_activation::move_activator,
                    move_advancement::move_advancement,
                    cinematic_locks::handle_cinematics, // This being the first system after hit move advancement is important
                    recovery::stun_recovery,
                    recovery::ground_recovery,
                    size_adjustment::size_adjustment,
                    condition_management::manage_conditions,
                    crate::assets::update_animation,
                    crate::assets::update_audio,
                    side_switcher::sideswitcher,
                )
                    .chain()
                    .in_set(WAGStage::PlayerUpdates),
            );
        // There is a max of 15 systems per call to add_systems
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
    defense: Defense,
    inventory: Inventory,
    spawner: HitboxSpawner,
    player_velocity: PlayerVelocity,
    move_buffer: MoveBuffer,
    joints: Joints,
    status_effects: Stats,
}

fn spawn_player(
    commands: &mut Commands,
    models: &Models,
    offset: f32,
    player: Player,
    character: CharacterId,
) -> Entity {
    let character = match character {
        CharacterId::Dummy => dummy(),
        CharacterId::Mizku => mizku(),
    };

    let colors = character.colors[&player].clone();

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation((offset, PLAYER_SPAWN_HEIGHT, 0.0).into()),
                ..default()
            },
            WAGResources::from_stats(&character.base_stats, character.special_properties.clone()),
            PlayerDefaults::default(),
            PadBundle::new(character.get_inputs()),
            Name::new(format!("Player {player}")),
            AnimationHelperSetup(character.generic_animations[&AnimationType::Default]),
            Facing::from_flipped(offset.is_sign_positive()),
            Pushbox(character.standing_pushbox),
            character.clone(),
            PlayerState::default(),
            player,
            StateScoped(InMatch),
        ))
        .with_children(move |parent| {
            parent.spawn((
                PlayerModelHook(colors.clone()),
                SceneBundle {
                    scene: models[&character.model].clone(),
                    ..default()
                },
            ));
        })
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
            PLAYER_SPAWN_HEIGHT,
            0.0,
        );
    }
}