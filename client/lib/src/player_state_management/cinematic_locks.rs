use bevy::prelude::*;
use player_state::PlayerState;
use wag_core::{Clock, Joint, Joints};

use crate::event_spreading::LockPlayer;

pub fn handle_cinematics(
    mut players: Query<(&mut PlayerState, Entity, &Joints)>,
    clock: Res<Clock>,
    mut tfs: Query<&mut Transform>,
    gtfs: Query<&GlobalTransform>,
) {
    for (mut state, player_entity, joints) in &mut players {
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
    }
}

pub fn start_lock(
    trigger: Trigger<LockPlayer>,
    clock: Res<Clock>,
    mut players: Query<&mut PlayerState>,
) {
    let mut state = players.get_mut(trigger.entity()).unwrap();
    state.start_cinematic(trigger.event().0 + clock.frame);
}
