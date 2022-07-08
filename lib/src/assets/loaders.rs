use bevy::prelude::*;
use bevy_hanabi::*;
use types::VisualEffect;

use super::{
    sounds::{get_sound_paths, Sounds},
    Animations, Colors, Fonts, Model, Models, Particles, Sprites,
};

pub fn colors(mut commands: Commands) {
    commands.insert_resource(Colors {
        health: Color::rgb(0.9, 0.0, 0.0),
        meter: Color::rgb(0.04, 0.5, 0.55),
        charge_default: Color::rgb(0.05, 0.4, 0.55),
        charge_full: Color::rgb(0.9, 0.1, 0.3),
        hitbox: Color::rgba(1.0, 0.0, 0.0, 0.5),
        hurtbox: Color::rgba(0.0, 1.0, 0.0, 0.5),
        pushbox: Color::rgba(0.0, 0.0, 1.0, 0.5),
        text: Color::WHITE,
    })
}

pub fn fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    commands.insert_resource(Fonts { basic })
}

pub fn sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sprites {
        background_image: asset_server.load("CPT-2018-Stage.png"),
    });
}

pub fn models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models(
        vec![(Model::Dummy, asset_server.load("dummy-character.glb"))]
            .into_iter()
            .collect(),
    ));
}

pub fn animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut collector = vec![];

    for kit in kits::all_kits() {
        collector.append(&mut kit.get_animations());
    }

    commands.insert_resource(Animations(
        collector
            .into_iter()
            .map(|key| (key, asset_server.load(key)))
            .collect(),
    ));
}

pub fn sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sounds::new(
        get_sound_paths()
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
            .collect(),
    ))
}

pub fn particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let particles = Particles::new(
        vec![
            (
                VisualEffect::Block,
                block_entity(&mut commands, &mut effects),
            ),
            (VisualEffect::Hit, hit_entity(&mut commands, &mut effects)),
        ]
        .into_iter()
        .collect(),
    );

    commands.insert_resource(particles);
}

fn block_entity(commands: &mut Commands, effects: &mut ResMut<Assets<EffectAsset>>) -> Entity {
    particle_explosion(
        commands,
        effects,
        "block",
        Gradient::constant(Vec4::new(0.1, 0.2, 1.0, 1.0)),
        vanishing_size_gradient(Vec2::new(0.1, 0.1), 0.1),
        2.0,
    )
}

fn hit_entity(commands: &mut Commands, effects: &mut ResMut<Assets<EffectAsset>>) -> Entity {
    particle_explosion(
        commands,
        effects,
        "hit",
        Gradient::constant(Vec4::new(1.0, 0.7, 0.0, 1.0)),
        vanishing_size_gradient(Vec2::new(0.1, 0.1), 0.2),
        3.0,
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
    effects: &mut ResMut<Assets<EffectAsset>>,
    name: &'static str,
    color_gradient: Gradient<Vec4>,
    size_gradient: Gradient<Vec2>,
    speed: f32,
) -> Entity {
    let spawner = Spawner::once(100.0.into(), false);
    let effect = effects.add(
        EffectAsset {
            name: name.into(),
            capacity: 1000,
            spawner,
            ..default()
        }
        .init(PositionSphereModifier {
            dimension: ShapeDimension::Surface,
            radius: 0.2,
            speed: speed.into(),
            ..default()
        })
        .update(AccelModifier {
            accel: Vec3::new(0.0, -2.0, 0.0),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
        }),
    );

    commands
        .spawn_bundle(ParticleEffectBundle::new(effect).with_spawner(spawner))
        .id()
}
