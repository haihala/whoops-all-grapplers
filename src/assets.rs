use bevy::prelude::*;

pub struct AssetsPlugin;

pub struct Materials {
    pub hitbox_color: Handle<ColorMaterial>,
    pub hurtbox_color: Handle<ColorMaterial>,
    pub collision_box_color: Handle<ColorMaterial>,
    pub background_image: Handle<StandardMaterial>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(
            crate::labels::StartupStageLabel::LoadAssets,
            colors.system(),
        );
    }
}

fn colors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    mut sprite_assets: ResMut<Assets<StandardMaterial>>,
) {
    let texture = asset_server.load("CPT-2018-Stage.png");

    commands.insert_resource(Materials {
        hitbox_color: color_assets.add(Color::rgb(1.0, 0.0, 0.0).into()),
        hurtbox_color: color_assets.add(Color::rgb(0.0, 1.0, 0.0).into()),
        collision_box_color: color_assets.add(Color::rgb(0.0, 0.0, 1.0).into()),
        background_image: sprite_assets.add(StandardMaterial {
            base_color_texture: Some(texture),
            unlit: true,
            ..Default::default()
        }),
    })
}
