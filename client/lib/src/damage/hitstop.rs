use std::time::Duration;

use bevy::{prelude::*, utils::Instant};
use characters::ActionEvent;
use player_state::PlayerState;

#[derive(Debug, Resource, Deref)]
pub struct Hitstop(Instant);

const HITSTOP_DURATION: Duration = Duration::from_millis(30);

pub fn handle_hitstop_events(
    mut commands: Commands,
    mut query: Query<&mut PlayerState>,
    mut game_time: ResMut<Time<Virtual>>,
    real_time: Res<Time<Real>>,
) {
    for mut state in &mut query {
        for _ in state.drain_matching_actions(|action| {
            if matches!(*action, ActionEvent::Hitstop) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            game_time.set_relative_speed(0.001);
            commands.insert_resource(Hitstop(real_time.last_update().unwrap() + HITSTOP_DURATION));
        }
    }
}

pub fn clear_hitstop(
    mut commands: Commands,
    maybe_hitstop: Option<ResMut<Hitstop>>,
    mut game_time: ResMut<Time<Virtual>>,
    real_time: Res<Time<Real>>,
) {
    let Some(hitstop) = maybe_hitstop else {
        return;
    };

    if hitstop.0 < real_time.last_update().unwrap() {
        commands.remove_resource::<Hitstop>();
        game_time.set_relative_speed(1.0);
    }
}
