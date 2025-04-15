use bevy::prelude::*;
use foundation::CharacterClock;

use crate::{assets::AnimationHelper, event_spreading::StartHitstop};

pub fn start_hitstop(
    trigger: Trigger<StartHitstop>,
    mut helpers: Query<(&mut CharacterClock, &AnimationHelper)>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    let (mut clock, anim_player) = helpers.get_mut(trigger.entity()).unwrap();
    let StartHitstop(duration) = trigger.event();
    clock.hitstop_frames = *duration;
    anim_players
        .get_mut(anim_player.player_entity)
        .unwrap()
        .pause_all();
}

pub fn update_hitstop(
    mut hitstops: Query<(&mut CharacterClock, &AnimationHelper)>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    for (mut clock, helper) in &mut hitstops {
        if clock.hitstop_frames == 1 {
            anim_players
                .get_mut(helper.player_entity)
                .unwrap()
                .resume_all();
            clock.hitstop_frames = 0;
        } else if clock.hitstop_frames > 0 {
            clock.hitstop_frames -= 1;
        }
    }
}
