use bevy::prelude::*;
use foundation::{
    CharacterClock, Clock, GameState, InMatch, MatchState, Player, Players, RollbackSchedule,
    SystemStep,
};

#[derive(Component, Copy, Clone)]
pub enum DespawnMarker {
    Absolute(usize),
    Relative(usize, Player),
}

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
    abs_clock: Res<Clock>,
    rel_clocks: Query<&CharacterClock>,
    marks: Query<(Entity, &DespawnMarker)>,
    maybe_players: Option<Res<Players>>,
) {
    let Some(players) = maybe_players else {
        // Not present on frame 0 I think?
        return;
    };

    for (marked, marker) in &marks {
        let should_despawn = match marker {
            DespawnMarker::Absolute(frame) => *frame < abs_clock.frame,
            DespawnMarker::Relative(frame, player) => {
                *frame < rel_clocks.get(players.get(*player)).unwrap().frame
            }
        };

        if should_despawn {
            commands.entity(marked).despawn();
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
