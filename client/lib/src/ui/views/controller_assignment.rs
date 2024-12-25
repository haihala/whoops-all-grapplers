use bevy::prelude::*;
use foundation::{
    Controllers, GameState, InputDevice, InputStream, LocalState, MenuInput, Player, SoundRequest,
    CONTROLLER_ASSIGNMENT_SIDE_COLOR, KEYBOARD_MAGIC_CONSTANT,
};

use crate::{assets::Fonts, entity_management::VisibleInStates};

use super::setup_view_title;

#[derive(Debug, Resource, Default)]
pub struct ControllerAssignment {
    p1: Option<InputDevice>,
    p2: Option<InputDevice>,
}
impl ControllerAssignment {
    fn left(&mut self, player_handle: InputDevice) {
        if self.p2 == Some(player_handle) {
            self.p2 = None;
        } else if self.p1.is_none() {
            self.p1 = Some(player_handle);
        }
    }

    fn right(&mut self, player_handle: InputDevice) {
        if self.p1 == Some(player_handle) {
            self.p1 = None;
        } else if self.p2.is_none() {
            self.p2 = Some(player_handle);
        }
    }

    fn is_complete(&self) -> bool {
        self.p1.is_some() && self.p2.is_some()
    }
}

#[derive(Debug, Component)]
pub struct SelectedController(Player);

#[derive(Debug, Component)]
pub struct FreeControllers;

pub fn setup_controller_assignment(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: Val::Percent(0.5),
                padding: UiRect::all(Val::Percent(10.0)),
                ..default()
            },
            VisibleInStates(vec![GameState::Local(LocalState::ControllerAssignment)]),
            Name::new("Controller assignment UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, "Pick your controllers");
            setup_areas(cb, &fonts);
        });

    commands.insert_resource(ControllerAssignment::default());
}

fn setup_areas(root: &mut ChildBuilder, fonts: &Fonts) {
    root.spawn((
        Node {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Stretch,
            ..default()
        },
        Name::new("Area container"),
    ))
    .with_children(|cb| {
        create_selected_controller_area(fonts, cb, Player::One);
        create_unused_controller_area(fonts, cb);
        create_selected_controller_area(fonts, cb, Player::Two);
    });
}

fn create_selected_controller_area(fonts: &Fonts, root: &mut ChildBuilder, player: Player) {
    root.spawn((
        Node {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            column_gap: Val::Percent(2.0),
            ..default()
        },
        BackgroundColor(CONTROLLER_ASSIGNMENT_SIDE_COLOR),
        SelectedController(player),
    ))
    .with_children(|cb| {
        cb.spawn((
            Text::from(format!("Player {}", player)),
            TextFont {
                font: fonts.basic.clone(),
                font_size: 60.0,
                ..default()
            },
        ));
    });
}

fn create_unused_controller_area(fonts: &Fonts, root: &mut ChildBuilder) {
    root.spawn((
        Node {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            column_gap: Val::Percent(2.0),
            ..default()
        },
        FreeControllers,
    ))
    .with_children(|cb| {
        cb.spawn((
            Text::from("Unassigned controllers"),
            TextFont {
                font: fonts.basic.clone(),
                font_size: 60.0,
                ..default()
            },
        ));
    });
}

pub fn navigate_controller_assignment_menu(
    mut commands: Commands,
    mut ca: ResMut<ControllerAssignment>,
    input_stream: ResMut<InputStream>,
    mut state: ResMut<NextState<GameState>>,
) {
    for ev in input_stream.menu_events.clone() {
        match ev.event {
            MenuInput::Right => ca.right(ev.player_handle),
            MenuInput::Left => ca.left(ev.player_handle),
            MenuInput::Accept => {
                if ca.is_complete() {
                    commands.insert_resource(Controllers {
                        p1: ca.p1.unwrap(),
                        p2: ca.p2.unwrap(),
                    });

                    state.set(GameState::Local(LocalState::CharacterSelect));
                    commands.trigger(SoundRequest::menu_transition());
                }
            }
            MenuInput::Cancel => {
                state.set(GameState::MainMenu);
                commands.trigger(SoundRequest::menu_transition());
            }
            _ => {}
        }
    }
}

#[derive(Debug, Component)]
pub struct ControllerIcon;

pub fn update_controller_assignment_menu_visuals(
    mut commands: Commands,
    ca: Res<ControllerAssignment>,
    pads: Query<Entity, With<Gamepad>>,
    existing_icons: Query<Entity, With<ControllerIcon>>,
    free_container: Query<Entity, With<FreeControllers>>,
    selected_controllers: Query<(Entity, &SelectedController)>,
    fonts: Res<Fonts>,
) {
    if !ca.is_changed() {
        return;
    }

    for old in &existing_icons {
        commands.entity(old).despawn_recursive();
    }

    let p1_selected = selected_controllers
        .iter()
        .find(|(_, sc)| sc.0 == Player::One)
        .unwrap()
        .0;

    let p2_selected = selected_controllers
        .iter()
        .find(|(_, sc)| sc.0 == Player::Two)
        .unwrap()
        .0;

    let unused = free_container.get_single().unwrap();
    for (index, pad_id) in [(KEYBOARD_MAGIC_CONSTANT, InputDevice::Keyboard)]
        .into_iter()
        .chain(pads.iter().map(InputDevice::Controller).enumerate())
    {
        if ca.p1 == Some(pad_id) {
            commands
                .entity(p1_selected)
                .with_children(create_icon(index, &fonts));
        } else if ca.p2 == Some(pad_id) {
            commands
                .entity(p2_selected)
                .with_children(create_icon(index, &fonts));
        } else {
            commands
                .entity(unused)
                .with_children(create_icon(index, &fonts));
        }
    }
}

fn create_icon(id: usize, fonts: &Fonts) -> impl Fn(&mut ChildBuilder) {
    let font = fonts.basic.clone();
    move |cb: &mut ChildBuilder| {
        cb.spawn((
            Text::from(if id == KEYBOARD_MAGIC_CONSTANT {
                "keyboard".into()
            } else {
                id.to_string()
            }),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..default()
            },
            ControllerIcon,
        ));
    }
}
