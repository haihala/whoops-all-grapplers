use bevy::prelude::*;
use characters::ActionEvent;
use player_state::PlayerState;
use wag_core::{Clock, Joint, Joints};

pub fn handle_locks(
    mut players: Query<(&mut PlayerState, Entity, &Joints)>,
    clock: Res<Clock>,
    mut tfs: Query<&mut Transform>,
) {
    for (mut state, player_entity, joints) in &mut players {
        if let Some(unlock_frame) = state.unlock_frame() {
            if unlock_frame <= clock.frame {
                // Move the player by abdomen joint transform to snap the character where the model is
                let model = joints.nodes.get(&Joint::Abdomen).unwrap();
                let [model_tf, mut player_tf] = tfs.get_many_mut([*model, player_entity]).unwrap();
                player_tf.translation -= Vec3 {
                    // This is a hack, as there is no good bone to base the offset on
                    x: model_tf.translation.x,
                    y: model_tf.translation.y - 0.95,
                    z: 0.0,
                };

                state.unlock(player_tf.translation.y > 0.1);
            }
        }

        for duration in state.drain_matching_actions(|action| {
            if let ActionEvent::Lock(frames) = action {
                Some(frames.to_owned())
            } else {
                None
            }
        }) {
            state.lock(duration + clock.frame);
        }
    }
}
