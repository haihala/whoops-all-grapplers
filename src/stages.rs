use bevy::prelude::*;

pub static LOAD_ASSETS: &str = "assets";
pub static PRE_UPDATE: &str = "pre-update";

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_before(StartupStage::Startup, LOAD_ASSETS, SystemStage::parallel())
            .add_stage_after(CoreStage::PreUpdate, PRE_UPDATE, SystemStage::parallel());
    }
}
