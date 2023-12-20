mod asset_updater;
mod charge_accumulator;
mod condition_management;
mod dynamic_colliders;
mod move_activation;
mod move_advancement;
mod movement;
mod player_flash;
mod recovery;
mod root_mover;
mod size_adjustment;

use characters::{dummy, mizku, Character, Inventory, WAGResources};
use input_parsing::{InputParser, PadBundle};
use player_state::PlayerState;
use wag_core::{
    AnimationType, CharacterId, Clock, Facing, GameState, Joints, Player, Players, Stats, WAGStage,
    WagArgs,
};

use crate::{
    assets::{AnimationHelper, AnimationHelperSetup, Models},
    damage::{Defense, HitboxSpawner},
    physics::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::{
    ecs::query::WorldQuery, pbr::ExtendedMaterial, prelude::*, render::view::NoFrustumCulling,
};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

pub use move_activation::MoveBuffer;

use player_flash::FlashMaterial;
use root_mover::RootMover;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

#[derive(WorldQuery)]
#[world_query(mutable)]
struct PlayerQuery<'a> {
    state: &'a mut PlayerState,
    spawner: &'a mut HitboxSpawner,
    character: &'a Character,
    tf: &'a Transform,
    buffer: &'a mut MoveBuffer,
    properties: &'a mut WAGResources,
    inventory: &'a mut Inventory,
    input_parser: &'a mut InputParser,
    player: &'a Player,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::PreRound), setup_combat)
            // This is here so it's up to date when the round starts
            .add_systems(
                Update,
                (
                    condition_management::update_combined_status_effect
                        .before(WAGStage::PlayerUpdates),
                    player_flash::customize_scene_materials,
                    player_flash::handle_flash_events,
                ),
            )
            .add_systems(
                Update,
                (
                    move_activation::manage_buffer,
                    move_activation::automatic_activation,
                    move_activation::raw_or_link,
                    move_activation::special_cancel,
                    move_activation::move_activator,
                    move_advancement::move_advancement,
                    recovery::stun_recovery,
                    recovery::ground_recovery,
                    movement::movement,
                    size_adjustment::size_adjustment,
                    charge_accumulator::manage_charge,
                    condition_management::manage_conditions,
                    asset_updater::update_animation,
                    asset_updater::update_audio,
                    root_mover::update_root_transform,
                )
                    .chain()
                    .in_set(WAGStage::PlayerUpdates),
            )
            // There is a max of 15 systems per call to add_systems
            .add_systems(
                Update,
                (
                    dynamic_colliders::create_colliders,
                    dynamic_colliders::update_colliders,
                )
                    .in_set(WAGStage::PlayerUpdates),
            )
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, FlashMaterial>,
            >::default());
    }
}

fn setup(mut commands: Commands, models: Res<Models>, args: Res<WagArgs>) {
    let players = Players {
        one: spawn_player(
            &mut commands,
            &models,
            -PLAYER_SPAWN_DISTANCE,
            Player::One,
            args.character1,
        ),
        two: spawn_player(
            &mut commands,
            &models,
            PLAYER_SPAWN_DISTANCE,
            Player::Two,
            args.character2,
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
        ))
        .with_children(move |parent| {
            parent.spawn((
                HookedSceneBundle {
                    scene: SceneBundle {
                        scene: models[&character.model].clone(),
                        ..default()
                    },
                    hook: SceneHook::new(move |_, cmds| {
                        cmds.insert((
                            player_flash::UpdateMaterial(colors.clone()),
                            NoFrustumCulling,
                        ));

                        // TODO: Use this for attaching to joints and flipping animations
                    }),
                },
                RootMover,
            ));
        })
        .id()
}

#[allow(clippy::type_complexity)]
fn setup_combat(
    mut query: Query<(
        &Player,
        &Stats,
        &Inventory,
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
    dbg!("Reset");
    clock.reset(bevy_time.elapsed_seconds_f64());

    for (
        player,
        stats,
        inventory,
        mut resources,
        mut tf,
        mut player_state,
        mut buffer,
        mut parser,
        mut velocity,
        mut animation_helper,
    ) in &mut query
    {
        resources.reset(stats, inventory);
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
