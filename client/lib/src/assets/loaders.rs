use bevy::prelude::*;
use bevy_hanabi::*;

use wag_core::VisualEffect;

use super::{
    animations::animation_paths,
    models::model_paths,
    sounds::{get_sound_paths, Sounds},
    Animations, Colors, Fonts, Models, Particles,
};

pub fn colors(mut commands: Commands) {
    commands.insert_resource(Colors {
        notification_background: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 0.3,
        },
        notification_text: Color::BLACK,
        hitbox: Color::rgba(1.0, 0.0, 0.0, 0.5),
        hurtbox: Color::rgba(0.0, 1.0, 0.0, 0.5),
        pushbox: Color::rgba(0.0, 0.0, 1.0, 0.5),
        text: Color::WHITE,
        default_item_slot: Color::GRAY,
        highlighted_item_slot: Color::rgb_u8(195, 24, 24), // WWE Raw red
        disabled_item_slot: Color::BISQUE,
        shop_timer_background: Color::rgba(0.0, 0.0, 0.0, 0.8),
    })
}

pub fn fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    commands.insert_resource(Fonts { basic })
}

pub fn models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models(
        model_paths()
            .into_iter()
            .map(|(key, path)| (key, asset_server.load(path)))
            .collect(),
    ));
}

pub fn animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animations::new(
        animation_paths()
            .into_iter()
            .map(|(key, path)| (key, asset_server.load(path)))
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

// Typed like this so it can be ignored in unit tests
pub fn particles(mut commands: Commands, effects: Option<ResMut<Assets<EffectAsset>>>) {
    let resource = Particles::new(if let Some(mut effects) = effects {
        vec![
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
        .collect()
    } else {
        default()
    });
    commands.insert_resource(resource);
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
