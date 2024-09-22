use bevy::prelude::*;
use wag_core::{RollbackSchedule, WAGStage};

mod charge_accumulator;
mod economy;
mod meter_over_time;

pub use economy::{clear_properties, modify_properties};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                charge_accumulator::manage_charge,
                meter_over_time::meter_over_time,
            )
                .chain()
                .in_set(WAGStage::ResourceUpdates),
        );
    }
}
