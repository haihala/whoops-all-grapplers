use std::time::Duration;

use bevy::prelude::*;
use characters::ActionEvent;
use player_state::PlayerState;
use wag_core::Hitstop;

const HITSTOP_DURATION: Duration = Duration::from_millis(50);

pub fn handle_hitstop_events(
    mut commands: Commands,
    mut players: Query<&mut PlayerState>,
    mut animation_players: Query<&mut AnimationPlayer>,
    real_time: Res<Time<Real>>,
) {
    for mut state in &mut players {
        for _ in state.drain_matching_actions(|action| {
            if matches!(*action, ActionEvent::Hitstop) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            commands.insert_resource(Hitstop(real_time.last_update().unwrap() + HITSTOP_DURATION));
            animation_players.for_each_mut(|mut player| {
                player.pause();
            });
        }
    }
}

pub fn clear_hitstop(
    mut commands: Commands,
    maybe_hitstop: Option<ResMut<Hitstop>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    real_time: Res<Time<Real>>,
) {
    let Some(hitstop) = maybe_hitstop else {
        return;
    };

    if hitstop.0 < real_time.last_update().unwrap() {
        commands.remove_resource::<Hitstop>();
        animation_players.for_each_mut(|mut player| {
            player.resume();
        });
    }
}
