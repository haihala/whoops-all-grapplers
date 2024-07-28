use bevy::prelude::*;
use wag_core::Clock;

#[derive(Component)]
pub struct DespawnMarker(pub usize);

pub(super) fn despawn_marked(
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
