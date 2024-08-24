use bevy::prelude::*;
use wag_core::{Clock, GameState};

#[derive(Component)]
pub struct DespawnMarker(pub usize);

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            despawn_marked.after(crate::damage::handle_despawn_flags),
        )
        .add_systems(OnExit(GameState::Combat), despawn_all_timed);
    }
}

pub fn despawn_marked(
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

pub fn despawn_all_timed(mut commands: Commands, mut targets: Query<Entity, With<DespawnMarker>>) {
    for entity in &mut targets {
        commands.entity(entity).despawn_recursive();
    }
}
