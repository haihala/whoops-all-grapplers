use bevy::prelude::*;
use wag_core::{Clock, GameState, InMatch, InMenu};

#[derive(Component)]
pub struct DespawnMarker(pub usize);

#[derive(Debug, Component, Deref)]
pub struct VisibleInStates(pub Vec<GameState>);

#[derive(Debug, Component, Deref)]
pub struct LivesInStates(pub Vec<GameState>);

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            despawn_marked.after(crate::damage::handle_despawn_flags),
        )
        .add_systems(
            Update,
            (despawn_on_state_change, update_visibility_on_state_change),
        )
        .enable_state_scoped_entities::<GameState>()
        .enable_state_scoped_entities::<InMenu>()
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

fn despawn_on_state_change(
    state: Res<State<GameState>>,
    mut commands: Commands,
    query: Query<(Entity, &LivesInStates)>,
) {
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
