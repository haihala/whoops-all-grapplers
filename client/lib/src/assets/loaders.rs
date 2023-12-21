use bevy::{prelude::*, utils::HashMap};
use bevy_hanabi::*;

use wag_core::{Animation, Model, SoundEffect, VisualEffect};

use super::{
    animations::animation_paths,
    models::model_paths,
    sounds::{get_sound_paths, Sounds},
    Animations, AssetsLoading, Fonts, Models, Particles,
};

pub fn fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    loading_assets.0.push(basic.clone().untyped());
    commands.insert_resource(Fonts { basic });
}

pub fn models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let handles: HashMap<Model, Handle<Scene>> = model_paths()
        .into_iter()
        .map(|(key, path)| (key, asset_server.load(path)))
        .collect();

    commands.insert_resource(Models(handles.clone()));

    loading_assets
        .0
        .extend(handles.values().cloned().map(|h| h.untyped()));
}

pub fn animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let handles: HashMap<Animation, Handle<AnimationClip>> = animation_paths()
        .into_iter()
        .map(|(key, path)| (key, asset_server.load(path)))
        .collect();

    loading_assets
        .0
        .extend(handles.values().cloned().map(|h| h.untyped()));

    commands.insert_resource(Animations::new(handles));
}

pub fn sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>> = get_sound_paths()
        .into_iter()
        .map(|(id, paths)| {
            (
                id,
                paths
                    .into_iter()
                    .map(|path| asset_server.load(path))
                    .collect(),
            )
        })
        .collect();

    commands.insert_resource(Sounds::new(handles.clone()));

    loading_assets.0.extend(
        handles
            .values()
            .cloned()
            .flat_map(|audio_type| audio_type.into_iter().map(|h| h.untyped())),
    );
}

// Typed like this so it can be ignored in unit tests
pub fn particles(mut commands: Commands, effects: Option<ResMut<Assets<EffectAsset>>>) {
    if let Some(mut effects) = effects {
        let handles = vec![
            (
                VisualEffect::Block,
                block_entity(&mut commands, &mut effects),
            ),
            (VisualEffect::Hit, hit_entity(&mut commands, &mut effects)),
            (
                VisualEffect::Clash,
                clash_entity(&mut commands, &mut effects),
            ),
        ]
        .into_iter()
        .collect();

        commands.insert_resource(Particles::new(handles));
    };
}

fn block_entity(commands: &mut Commands, effects: &mut Assets<EffectAsset>) -> Entity {
    particle_explosion(
        commands,
        effects,
        "block",
        Gradient::constant(Vec4::new(0.1, 0.2, 1.0, 1.0)),
        vanishing_size_gradient(Vec2::new(0.1, 0.1), 0.1),
        2.0,
        50.0,
    )
}

fn hit_entity(commands: &mut Commands, effects: &mut Assets<EffectAsset>) -> Entity {
    particle_explosion(
        commands,
        effects,
        "hit",
        Gradient::constant(Vec4::new(1.0, 0.7, 0.0, 1.0)),
        vanishing_size_gradient(Vec2::new(0.1, 0.1), 0.2),
        3.0,
        100.0,
    )
}

fn clash_entity(commands: &mut Commands, effects: &mut Assets<EffectAsset>) -> Entity {
    particle_explosion(
        commands,
        effects,
        "clash",
        Gradient::constant(Vec4::new(0.2, 0.3, 0.5, 1.0)),
        vanishing_size_gradient(Vec2::new(0.1, 0.1), 0.1),
        8.0,
        30.0,
    )
}

fn vanishing_size_gradient(start: Vec2, duration: f32) -> Gradient<Vec2> {
    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, start);
    size_gradient.add_key(duration, Vec2::splat(0.0));
    size_gradient
}

fn particle_explosion(
    commands: &mut Commands,
    effects: &mut Assets<EffectAsset>,
    name: &'static str,
    color_gradient: Gradient<Vec4>,
    size_gradient: Gradient<Vec2>,
    speed: f32,
    amount: f32,
) -> Entity {
    let spawner = Spawner::once(amount.into(), false);
    let mut module = Module::default();

    let position_modifier = SetPositionSphereModifier {
        dimension: ShapeDimension::Surface,
        radius: module.lit(0.2),
        center: module.lit(Vec3::ZERO),
    };

    let velocity_modifier = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(speed),
    };

    let lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(1.0));

    let gravity = AccelModifier::new(module.lit(Vec3::new(0.0, -2.0, 0.0)));

    let effect = effects.add(
        EffectAsset::new(1000, spawner, module)
            .init(position_modifier)
            .init(velocity_modifier)
            .init(lifetime)
            .update(gravity)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                ..default()
            }),
    );

    commands
        .spawn((
            ParticleEffectBundle::new(effect).with_spawner(spawner),
            Name::new(format!("Particle system '{name}'")),
        ))
        .id()
}
