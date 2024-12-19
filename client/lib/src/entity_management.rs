use bevy::prelude::*;
use foundation::{Clock, GameState, InMatch, MatchState, RollbackSchedule, SystemStep};

#[derive(Component, Copy, Clone)]
pub struct DespawnMarker(pub usize);

#[derive(Debug, Component, Deref)]
pub struct VisibleInStates<T: States>(pub Vec<T>);

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                update_visibility_on_state_change::<GameState>,
                update_visibility_on_state_change::<MatchState>,
                despawn_marked,
            )
                .chain()
                .in_set(SystemStep::EntityManagement),
        )
        .enable_state_scoped_entities::<GameState>()
        .enable_state_scoped_entities::<MatchState>()
        .enable_state_scoped_entities::<InMatch>();
    }
}

fn despawn_marked(
    mut commands: Commands,
    clock: Res<Clock>,
    marks: Query<(Entity, &DespawnMarker)>,
) {
    for (marked, marker) in &marks {
        if marker.0 < clock.frame {
            commands.entity(marked).despawn_recursive();
        }
    }
}

fn update_visibility_on_state_change<T: States>(
    state: Res<State<T>>,
    mut query: Query<(&mut Visibility, &VisibleInStates<T>)>,
) {
    if state.is_changed() {
        debug!(
            "State transitioned to: {:?}, updating visibilities",
            state.get()
        );
        for (mut visibility, restriction) in &mut query {
            *visibility = if restriction.contains(state.get()) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
