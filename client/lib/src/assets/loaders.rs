use bevy::{prelude::*, utils::HashMap};

use foundation::{Animation, Icon, Icons, Model, SoundEffect};

use super::{
    animations::animation_paths, models::model_paths, sounds::Sounds, Animations, AssetsLoading,
    Fonts, Models,
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
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let handles: HashMap<Animation, Handle<AnimationClip>> = animation_paths()
        .into_iter()
        .map(|(key, path)| (key, asset_server.load(path)))
        .collect();

    loading_assets
        .0
        .extend(handles.values().cloned().map(|h| h.untyped()));

    let mut monograph = AnimationGraph::new();

    let mut indices: HashMap<Animation, AnimationNodeIndex> = HashMap::new();

    for (anim, handle) in handles.into_iter() {
        let index = monograph.add_clip(handle, 1.0, monograph.root);
        indices.insert(anim, index);
    }

    commands.insert_resource(Animations::new(graphs.add(monograph), indices));
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

    commands.insert_resource(Sounds {
        handles: handles.clone(),
    });

    loading_assets.0.extend(
        handles
            .values()
            .cloned()
            .flat_map(|audio_type| audio_type.into_iter().map(|h| h.untyped())),
    );
}
