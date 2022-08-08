use bevy::prelude::*;

pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
pub const ZERO: Val = Val::Percent(0.0);
pub const FULL: Val = Val::Percent(100.0);

pub(super) fn div_style() -> Style {
    Style {
        size: Size::new(FULL, FULL),
        position: UiRect {
            top: ZERO,
            left: ZERO,
            ..default()
        },
        ..default()
    }
}

pub(super) fn div() -> NodeBundle {
    NodeBundle {
        style: div_style(),
        color: TRANSPARENT.into(),
        ..default()
    }
}
