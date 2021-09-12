use bevy::prelude::*;

#[derive(StageLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum StartupStageLabel {
    LoadAssets,
}

#[derive(SystemLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum SystemSetLabel {
    Input,
    Characters,
}

#[derive(SystemLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum InputSystemLabel {
    Clear,
    Collect,
    Parse,
    Execute,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_before(
            StartupStage::Startup,
            StartupStageLabel::LoadAssets,
            SystemStage::parallel(),
        );
    }
}
