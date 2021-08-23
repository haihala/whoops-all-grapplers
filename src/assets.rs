use bevy::prelude::*;

pub struct AssetsPlugin;

pub struct Materials {
    pub hitbox_color: Handle<ColorMaterial>,
    pub hurtbox_color: Handle<ColorMaterial>,
    pub collision_box_color: Handle<ColorMaterial>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        static ASSETS: &str = "assets";
        app.add_startup_stage_before(StartupStage::Startup, ASSETS, SystemStage::parallel())
            .add_startup_system_to_stage(ASSETS, colors.system());
    }
}

fn colors(mut commands: Commands, mut assets: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        hitbox_color: assets.add(Color::rgb(1., 0., 0.).into()),
        hurtbox_color: assets.add(Color::rgb(0., 1., 0.).into()),
        collision_box_color: assets.add(Color::rgb(0., 0., 1.).into()),
    })
}
