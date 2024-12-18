use bevy::prelude::*;
use foundation::{Clock, Hitstop};

use crate::{assets::AnimationHelper, event_spreading::StartHitstop};

pub fn start_hitstop(
    trigger: Trigger<StartHitstop>,
    mut commands: Commands,
    clock: Res<Clock>,
    helpers: Query<&AnimationHelper>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    let StartHitstop(duration) = trigger.event();
    commands
        .entity(trigger.entity())
        .insert(Hitstop(clock.frame + *duration));

    dbg!("Starting hitstop");
    anim_players
        .get_mut(helpers.get(trigger.entity()).unwrap().player_entity)
        .unwrap()
        .pause_all();
}

pub fn clear_hitstop(
    mut commands: Commands,
    clock: Res<Clock>,
    hitstops: Query<(Entity, &AnimationHelper, Option<&Hitstop>)>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    for (entity, helper, maybe_hitstop) in &hitstops {
        if let Some(hs) = maybe_hitstop {
            if hs.0 <= clock.frame {
                commands.entity(entity).remove::<Hitstop>();
                anim_players
                    .get_mut(helper.player_entity)
                    .unwrap()
                    .resume_all();
            }
        }
    }
}
