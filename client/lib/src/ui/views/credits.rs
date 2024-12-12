pub use bevy::prelude::*;
use foundation::{GameButton, GameState, InputEvent, InputStream, StickPosition};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Component)]
pub struct CreditsNav;

pub fn setup_credits_menu(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    padding: UiRect::all(Val::Percent(20.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            VisibleInStates(vec![GameState::Credits]),
            CreditsNav,
            Name::new("Credits UI"),
        ))
        .with_children(|cb| {
            cb.spawn((
                TextBundle::from_section(
                    "Credits",
                    TextStyle {
                        font: fonts.basic.clone(),
                        font_size: 128.0,
                        ..default()
                    },
                ),
                Name::new("Credits main heading"),
            ));

            for section in credits_sections() {
                cb.spawn((
                    TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Percent(4.0)),
                            ..default()
                        },
                        ..TextBundle::from_section(
                            section.heading.clone(),
                            TextStyle {
                                font: fonts.basic.clone(),
                                font_size: 48.0,
                                ..default()
                            },
                        )
                    },
                    Name::new(format!("{} credits subheading", section.heading)),
                ));

                for name in section.people {
                    cb.spawn((
                        TextBundle::from_section(
                            name.clone(),
                            TextStyle {
                                font: fonts.basic.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                        ),
                        Name::new(format!("{} credit for {}", name, section.heading)),
                    ));
                }
            }
        });
}

struct CreditSection {
    heading: String,
    people: Vec<String>,
}

fn credits_sections() -> Vec<CreditSection> {
    vec![
        CreditSection {
            heading: "Playtesting".into(),
            people: vec![
                "Friends".into(),
                "Family".into(),
                "And especially the Tampere FGC, yall rock".into(),
            ],
        },
        CreditSection {
            heading: "And everything else".into(),
            people: vec!["Eero Häihälä".into()],
        },
    ]
}

const SCROLL_SPEED: f32 = 1.0;

pub fn navigate_credits(
    input_stream: Res<InputStream>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_root: Query<&mut Style, With<CreditsNav>>,
    mut scroll_direction: Local<f32>,
) {
    let mut style = ui_root.single_mut();

    for ev in input_stream.events.clone() {
        match ev.event {
            InputEvent::Press(GameButton::Strong) => {
                style.top = Val::Percent(0.0);
                next_state.set(GameState::MainMenu);
            }
            InputEvent::Point(dir) => {
                match dir {
                    StickPosition::N => *scroll_direction = 1.0,
                    StickPosition::S => *scroll_direction = -1.0,
                    _ => *scroll_direction = 0.0,
                };
            }
            _ => {}
        }
    }

    let Val::Percent(old) = style.top else {
        panic!()
    };
    let new = old + SCROLL_SPEED * *scroll_direction;
    style.top = Val::Percent(new.clamp(-10.0, 0.0));
}
