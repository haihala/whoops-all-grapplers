use bevy::{gltf::Gltf, prelude::*};

pub struct Colors {
    pub health: Color,
    pub meter: Color,
    pub charge_default: Color,
    pub charge_full: Color,
    pub hitbox: Color,
    pub hurtbox: Color,
    pub collision_box: Color,
    pub text: Color,
}

pub struct Fonts {
    pub basic: Handle<Font>,
}

pub struct Sprites {
    pub background_image: Handle<Image>,
}

pub struct Models {
    pub ryan: Handle<Gltf>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, colors)
            .add_startup_system_to_stage(StartupStage::PreStartup, fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, sprites)
            .add_startup_system_to_stage(StartupStage::PreStartup, models)
            .add_system(model_spawner)
            .add_system(animation_starter);
    }
}

fn colors(mut commands: Commands) {
    commands.insert_resource(Colors {
        health: Color::rgb(0.9, 0.0, 0.0),
        meter: Color::rgb(0.04, 0.5, 0.55),
        charge_default: Color::rgb(0.05, 0.4, 0.55),
        charge_full: Color::rgb(0.9, 0.1, 0.3),
        hitbox: Color::rgb(1.0, 0.0, 0.0),
        hurtbox: Color::rgb(0.0, 1.0, 0.0),
        collision_box: Color::rgba(0.0, 0.0, 1.0, 0.75),
        text: Color::WHITE,
    })
}

fn fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    commands.insert_resource(Fonts { basic })
}

fn sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sprites {
        background_image: asset_server.load("CPT-2018-Stage.png"),
    });
}

fn models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models {
        ryan: asset_server.load("dummy-character.glb"),
    });
}

/// You need to wait for gltf models to be loaded before they can be spawned.
/// Instead of doing that in sync, you can add this component
/// Component is removed when the spawning is done, so it's an easy way to see if all models have been loaded.
#[derive(Debug, Component)]
pub struct ModelRequest {
    pub model: Handle<Gltf>,
    pub animation: Option<(&'static str, bool)>,
}

#[derive(Debug, Component)]
pub struct AnimationRequest {
    pub animation: Handle<AnimationClip>,
    pub looping: bool,
}

fn model_spawner(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    query: Query<(Entity, &ModelRequest)>,
) {
    for (entity, request) in query.iter() {
        if let Some(gltf) = assets.get(&request.model) {
            let mut e = commands.entity(entity);
            e.with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
            e.remove::<ModelRequest>();

            if let Some((animation_name, looping)) = request.animation {
                e.insert(AnimationRequest {
                    looping,
                    animation: gltf.named_animations[animation_name].clone(),
                });
            }
        }
    }
}

fn animation_starter(
    mut commands: Commands,
    requests: Query<(Entity, &AnimationRequest)>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (master, request) in requests.iter() {
        let mut player = find_player(master, &children, &mut players).unwrap();
        player.play(request.animation.clone());
        if request.looping {
            player.repeat();
        }

        commands.entity(master).remove::<AnimationRequest>();
    }
}

fn find_player<'a>(
    parent: Entity,
    children: &Query<&Children>,
    players: &'a mut Query<&mut AnimationPlayer>,
) -> Option<Mut<'a, AnimationPlayer>> {
    // NGL this shit makes me want to puke, but it ought to safely and recursively find an AnimationPlayer under a parent if one exists
    if let Ok(candidates) = children.get(parent) {
        let mut next_candidates: Vec<Entity> =
            candidates.into_iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                if players.get(candidate).is_ok() {
                    return Some(players.get_mut(candidate).unwrap());
                } else {
                    next_candidates.extend(children.get(candidate).unwrap().into_iter());
                }
            }
        }
    }
    None
}
