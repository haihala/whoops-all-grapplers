use bevy::{prelude::*, utils::HashMap};

use wag_core::{Animation, Icon, Model, SoundEffect, VisualEffect};

use super::{
    animations::animation_paths,
    materials::{
        BlockEffectMaterial, ClashSparkMaterial, FocalPointLinesMaterial, HitSparkMaterial,
        LineFieldMaterial, RingRippleMaterial,
    },
    models::model_paths,
    sounds::Sounds,
    Animations, AssetsLoading, Fonts, Icons, Models, Vfx,
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

pub fn icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let handles: HashMap<Icon, Handle<Image>> = Icon::paths()
        .into_iter()
        .map(|(key, path)| (key, asset_server.load(path)))
        .collect();

    commands.insert_resource(Icons(handles.clone()));

    loading_assets
        .0
        .extend(handles.values().cloned().map(|h| h.untyped()));
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
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let handles: HashMap<Animation, Handle<AnimationClip>> = animation_paths()
        .into_iter()
        .map(|(key, path)| (key, asset_server.load(path)))
        .collect();

    loading_assets
        .0
        .extend(handles.values().cloned().map(|h| h.untyped()));

    commands.insert_resource(Animations::new(handles, &mut animation_graphs));
}

pub fn sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    let handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>> = SoundEffect::paths()
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

pub fn vfx(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clash_spark_materials: ResMut<Assets<ClashSparkMaterial>>,
    mut block_effect_materials: ResMut<Assets<BlockEffectMaterial>>,
    mut hit_spark_materials: ResMut<Assets<HitSparkMaterial>>,
    mut ring_ripple_materials: ResMut<Assets<RingRippleMaterial>>,
    mut speed_lines_materials: ResMut<Assets<LineFieldMaterial>>,
    mut focal_point_line_materials: ResMut<Assets<FocalPointLinesMaterial>>,
) {
    let mesh_handles = vec![
        (VisualEffect::Block, meshes.add(Rectangle::new(1.1, 2.0))),
        (VisualEffect::Hit, meshes.add(Rectangle::new(1.1, 1.1))),
        (VisualEffect::Clash, meshes.add(Rectangle::new(1.5, 1.5))),
        (
            VisualEffect::ThrowTech,
            meshes.add(Rectangle::new(2.0, 2.0)),
        ),
        (
            VisualEffect::SpeedLines,
            meshes.add(Rectangle::new(1.0, 1.0)),
        ),
        (
            VisualEffect::ThrowTarget,
            meshes.add(Rectangle::new(2.0, 2.0)),
        ),
    ]
    .into_iter()
    .collect();

    let clash_spark_material = clash_spark_materials.add(ClashSparkMaterial::default());
    let block_effect_material = block_effect_materials.add(BlockEffectMaterial::default());
    let hit_spark_material = hit_spark_materials.add(HitSparkMaterial::default());
    let throw_tech_material = ring_ripple_materials.add(RingRippleMaterial::default());
    let speed_lines_material = speed_lines_materials.add(LineFieldMaterial::default());
    let throw_target_material = focal_point_line_materials.add(FocalPointLinesMaterial::default());

    commands.insert_resource(Vfx::new(
        mesh_handles,
        clash_spark_material,
        block_effect_material,
        hit_spark_material,
        throw_tech_material,
        speed_lines_material,
        throw_target_material,
    ));
}
