use bevy::prelude::*;

use characters::{Character, Hurtboxes};
use foundation::Clock;
use player_state::PlayerState;

use crate::{event_spreading::ExpandHurtbox, movement::Pushbox};

pub fn update_box_sizes_from_state(
    mut query: Query<(&mut PlayerState, &mut Pushbox, &mut Hurtboxes, &Character)>,
) {
    for (state, mut pushbox, mut hurtboxes, character) in &mut query {
        let boxes = state.get_boxes(character);

        **pushbox = boxes.pushbox;

        let extras = hurtboxes.extra.clone();
        *hurtboxes = Hurtboxes::from(boxes);
        hurtboxes.extra = extras;
    }
}

pub fn expand_hurtboxes(
    trigger: Trigger<ExpandHurtbox>,
    clock: Res<Clock>,
    mut query: Query<&mut Hurtboxes>,
) {
    let mut hurtboxes = query.get_mut(trigger.target()).unwrap();
    hurtboxes
        .extra
        .push((trigger.event().area, trigger.event().duration + clock.frame));
}

pub fn remove_old_hurtbox_expansions(clock: Res<Clock>, mut query: Query<&mut Hurtboxes>) {
    for mut hurtboxes in &mut query {
        hurtboxes.expire(clock.frame);
    }
}
