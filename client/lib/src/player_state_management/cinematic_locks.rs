use bevy::prelude::*;
use characters::{ActionEvent, ActionEvents};
use player_state::PlayerState;
use wag_core::{Clock, Joint, Joints};

pub fn handle_cinematics(
    mut players: Query<(&mut PlayerState, &ActionEvents, Entity, &Joints)>,
    clock: Res<Clock>,
    mut tfs: Query<&mut Transform>,
    gtfs: Query<&GlobalTransform>,
) {
    for (mut state, events, player_entity, joints) in &mut players {
        let mut player_tf = tfs.get_mut(player_entity).unwrap();
        if let Some(unlock_frame) = state.active_cinematic() {
            if unlock_frame <= clock.frame {
                let abdomen = joints.nodes.get(&Joint::Abdomen).unwrap();
                let foot_l = joints.nodes.get(&Joint::FootL).unwrap();
                let foot_r = joints.nodes.get(&Joint::FootR).unwrap();
                let [abdomen_gtf, foot_l_gtf, foot_r_gtf] =
                    gtfs.get_many([*abdomen, *foot_l, *foot_r]).unwrap();

                let lowest_foot = f32::min(foot_l_gtf.translation().y, foot_r_gtf.translation().y);

                player_tf.translation = Vec3 {
                    y: if lowest_foot < 0.1 { 0.0 } else { lowest_foot },
                    x: abdomen_gtf.translation().x,
                    z: 0.0,
                };

                state.end_cinematic();
            }
        }

        for duration in events.get_matching_events(|action| {
            if let ActionEvent::Lock(frames) = action {
                Some(frames.to_owned())
            } else {
                None
            }
        }) {
            state.start_cinematic(duration + clock.frame);
        }
    }
}
