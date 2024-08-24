use bevy::prelude::*;
use wag_core::{Clock, GameState, InMatch};

#[derive(Component)]
pub struct DespawnMarker(pub usize);

#[derive(Debug, Component, Deref)]
pub struct VisibleInStates(pub Vec<GameState>);

#[derive(Debug, Component, Deref)]
pub struct LivesInStates<T: States>(pub Vec<T>);

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            despawn_marked.after(crate::damage::handle_despawn_flags),
        )
        // TODO: Check this actually works and it runs on state transitions
        // System may get new or old state, but code assumes new
        .add_systems(
            Update,
            (
                despawn_on_state_change::<GameState>,
                despawn_on_state_change::<InMatch>,
                update_visibility_on_state_change,
            ),
        );
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

fn despawn_on_state_change<T: States>(
    maybe_state: Option<Res<State<T>>>,
    mut commands: Commands,
    query: Query<(Entity, &LivesInStates<T>)>,
) {
    let Some(state) = maybe_state else { return };
    if state.is_changed() {
        for (entity, restriction) in &query {
            if !restriction.contains(state.get()) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn update_visibility_on_state_change(
    state: Res<State<GameState>>,
    mut query: Query<(&mut Visibility, &VisibleInStates)>,
) {
    if state.is_changed() {
        dbg!(state.get());
        for (mut visibility, restriction) in &mut query {
            *visibility = if restriction.contains(state.get()) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
