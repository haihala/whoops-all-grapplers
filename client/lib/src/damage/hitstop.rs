use std::time::Duration;

use bevy::prelude::*;
use characters::{ActionEvent, ActionEvents};
use wag_core::Hitstop;

const HITSTOP_DURATION: Duration = Duration::from_millis(100);

pub fn handle_hitstop_events(
    mut commands: Commands,
    players: Query<&ActionEvents>,
    real_time: Res<Time<Real>>,
) {
    for events in &players {
        for _ in events.get_matching_events(|action| {
            if matches!(*action, ActionEvent::Hitstop) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            commands.insert_resource(Hitstop(real_time.last_update().unwrap() + HITSTOP_DURATION));
        }
    }
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
