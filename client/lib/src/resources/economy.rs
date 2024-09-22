use bevy::prelude::*;
use characters::WAGResources;

use crate::event_spreading::{ClearResource, ModifyResource};

pub fn clear_properties(trigger: Trigger<ClearResource>, mut query: Query<&mut WAGResources>) {
    let mut properties = query.get_mut(trigger.entity()).unwrap();
    properties.get_mut(trigger.event().0).unwrap().clear();
}

pub fn modify_properties(trigger: Trigger<ModifyResource>, mut query: Query<&mut WAGResources>) {
    let mut properties = query.get_mut(trigger.entity()).unwrap();

    properties
        .get_mut(trigger.event().resource)
        .unwrap()
        .change(trigger.event().amount);
}
