use bevy::prelude::*;

#[derive(StageLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum StartupStageLabel {
    LoadAssets,
    UI,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_before(
            StartupStage::Startup,
            StartupStageLabel::LoadAssets,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            StartupStage::Startup,
            StartupStageLabel::UI,
            SystemStage::parallel(),
        );
    }
}
