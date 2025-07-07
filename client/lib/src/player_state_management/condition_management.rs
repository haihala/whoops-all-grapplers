use bevy::prelude::*;
use characters::{Character, Inventory};
use foundation::{CharacterClock, Stats, StatusCondition, StatusFlag};
use player_state::PlayerState;

use crate::event_spreading::{ClearStatus, ColorShift};

pub fn activate_conditions(
    trigger: Trigger<StatusCondition>,
    mut commands: Commands,
    mut query: Query<(&mut PlayerState, &CharacterClock)>,
) {
    let entity = trigger.target();
    let new_condition = trigger.event().clone();
    let (mut state, clock) = query.get_mut(entity).unwrap();

    if let Some(color) = new_condition.flag.display_color() {
        commands.trigger_targets(
            ColorShift(color, new_condition.expiration.unwrap_or(1000000)),
            entity,
        );
    }

    // Start invuln -> Vulnerable until back to neutral
    if new_condition.flag == StatusFlag::Intangible {
        commands.trigger_targets(
            StatusCondition {
                flag: StatusFlag::Weaken,
                ..default()
            },
            entity,
        );
    }

    state.add_condition(StatusCondition {
        expiration: new_condition
            .expiration
            .map(|duration| clock.frame + duration),
        ..new_condition
    });
}

pub fn clear_conditions(trigger: Trigger<ClearStatus>, mut query: Query<&mut PlayerState>) {
    let mut state = query.get_mut(trigger.target()).unwrap();
    state.clear_conditions(trigger.event().0.clone());
}

pub fn expire_conditions(mut query: Query<(&mut PlayerState, &CharacterClock)>) {
    for (mut state, clock) in &mut query {
        state.expire_conditions(clock.frame);
    }
}

pub fn update_combined_status_effect(
    mut query: Query<(&mut Stats, &PlayerState, &Inventory, &Character)>,
) {
    for (mut effect, state, inventory, character) in &mut query {
        *effect = state
            .combined_status_effects()
            .combine(&inventory.get_effects(character))
            .combine(&character.base_stats);
    }
}
