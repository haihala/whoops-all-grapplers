use std::time::Duration;

use bevy::prelude::*;
use wag_core::Hitstop;

use crate::event_spreading::StartHitstop;

const HITSTOP_DURATION: Duration = Duration::from_millis(100);

pub fn start_hitstop(
    _trigger: Trigger<StartHitstop>,
    mut commands: Commands,
    real_time: Res<Time<Real>>,
) {
    commands.insert_resource(Hitstop(real_time.last_update().unwrap() + HITSTOP_DURATION));
}

pub fn clear_hitstop(
    mut commands: Commands,
    maybe_hitstop: Option<ResMut<Hitstop>>,
    real_time: Res<Time<Real>>,
) {
    let Some(hitstop) = maybe_hitstop else {
        return;
    };

    if hitstop.0 < real_time.last_update().unwrap() {
        commands.remove_resource::<Hitstop>();
    }
}
