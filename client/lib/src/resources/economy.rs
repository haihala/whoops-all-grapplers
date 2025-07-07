use bevy::prelude::*;
use characters::{GaugeType, Gauges};
use foundation::{Sound, SoundRequest, StatusFlag};
use player_state::PlayerState;

use crate::event_spreading::{ClearResource, ModifyResource};

pub fn clear_properties(trigger: Trigger<ClearResource>, mut query: Query<&mut Gauges>) {
    let mut properties = query.get_mut(trigger.target()).unwrap();
    properties.get_mut(trigger.event().0).unwrap().clear();
}

pub fn modify_properties(
    trigger: Trigger<ModifyResource>,
    mut commands: Commands,
    mut query: Query<(&mut Gauges, &PlayerState)>,
) {
    let (mut properties, state) = query.get_mut(trigger.target()).unwrap();
    let mut ev = *trigger.event();

    if ev.resource == GaugeType::Health && state.has_flag(StatusFlag::Weaken) {
        ev.amount *= 2;
        commands.trigger(SoundRequest::from(Sound::PaperCrumple));
    }

    properties.get_mut(ev.resource).unwrap().change(ev.amount);
}
