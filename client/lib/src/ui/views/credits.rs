use bevy::prelude::*;
use foundation::{GameState, InputEvent, InputStream, MenuInput, SoundRequest, StickPosition};

use crate::{assets::Fonts, entity_management::VisibleInStates};

#[derive(Component)]
pub struct CreditsNav;

pub fn setup_credits_menu(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                bottom: Val::Percent(0.0),
                right: Val::Percent(0.0),
                left: Val::Percent(0.0),
                overflow: Overflow::scroll_y(),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(5.0),
                margin: UiRect::all(Val::Percent(1.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            VisibleInStates(vec![GameState::Credits]),
            CreditsNav,
            Name::new("Credits UI"),
        ))
        .with_children(|cb| {
            cb.spawn((
                Text::new("Credits"),
                TextFont {
                    font: fonts.basic.clone(),
                    font_size: 128.0,
                    ..default()
                },
                Name::new("Credits main heading"),
            ));

            for section in credits_sections() {
                cb.spawn((
                    Text::new(section.heading.clone()),
                    Node {
                        margin: UiRect::top(Val::Percent(4.0)),
                        ..default()
                    },
                    TextFont {
                        font: fonts.basic.clone(),
                        font_size: 48.0,
                        ..default()
                    },
                    Name::new(format!("{} credits subheading", section.heading)),
                ));

                for name in section.people {
                    cb.spawn((
                        Text::new(name.clone()),
                        TextFont {
                            font: fonts.basic.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        Name::new(format!("{name} credit for {}", section.heading)),
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
            heading: "Music found through Pixabay".into(),
            people: vec![
                "SigmaMusicArt (Mihail Smusev)".into(),
                "PHANTASTICBEATS (Vincent)".into(),
            ],
        },
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

const SCROLL_SPEED: f32 = 5.0;

pub fn navigate_credits(
    mut commands: Commands,
    input_stream: Res<InputStream>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_root: Query<&mut ScrollPosition, With<CreditsNav>>,
    mut scroll_direction: Local<f32>,
) {
    let mut scroll = ui_root.single_mut().unwrap();

    for ev in input_stream.menu_events.clone() {
        if ev.event == MenuInput::Cancel {
            commands.trigger(SoundRequest::menu_transition());
            scroll.offset_y = 0.0;
            next_state.set(GameState::MainMenu);
        }
    }

    // This uses non-menu inputs, because menu inputs don't get hold info
    // Shouldn't be a problem, as you shouldn't be able to rebind the stick
    for ev in input_stream.events.clone() {
        if let InputEvent::Point(dir) = ev.event {
            match dir {
                StickPosition::N => *scroll_direction = -1.0,
                StickPosition::S => *scroll_direction = 1.0,
                _ => *scroll_direction = 0.0,
            };
        }
    }

    let old = scroll.offset_y;
    let new = old + SCROLL_SPEED * *scroll_direction;

    scroll.offset_y = new;
}
