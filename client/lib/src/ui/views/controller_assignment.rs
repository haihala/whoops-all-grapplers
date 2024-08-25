use bevy::{ecs::system::SystemId, input::gamepad::GamepadEvent, prelude::*};
use wag_core::{
    Controllers, GameState, Player, CONTROLLER_ASSIGNMENT_SIDE_COLOR,
    MAIN_MENU_HIGHLIGHT_TEXT_COLOR,
};

use crate::{assets::Fonts, entity_management::VisibleInStates};

use super::{setup_view_title, MenuInputs};

#[derive(Debug, Resource, Default)]
pub struct ControllerAssignment {
    p1: Option<Gamepad>,
    p2: Option<Gamepad>,
}
impl ControllerAssignment {
    fn left(&mut self, pad: Gamepad) {
        if self.p2 == Some(pad) {
            self.p2 = None;
        } else if self.p1.is_none() {
            self.p1 = Some(pad);
        }
    }

    fn right(&mut self, pad: Gamepad) {
        if self.p1 == Some(pad) {
            self.p1 = None;
        } else if self.p2.is_none() {
            self.p2 = Some(pad);
        }
    }

    fn is_complete(&self) -> bool {
        self.p1.is_some() && self.p2.is_some()
    }
}

#[derive(Debug, Resource)]
pub struct SubmitCallback(SystemId);

#[derive(Debug, Component)]
pub struct SelectedController(Player);

#[derive(Debug, Component)]
pub struct FreeControllers;

pub fn setup_controller_assignment(mut commands: Commands, fonts: Res<Fonts>) {
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
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Percent(0.5),
                    padding: UiRect::all(Val::Percent(10.0)),
                    ..default()
                },
                ..default()
            },
            VisibleInStates(vec![GameState::ControllerAssignment]),
            Name::new("Controller assignment UI"),
        ))
        .with_children(|cb| {
            setup_view_title(cb, &fonts, "Pick your controllers");
            setup_areas(cb, &fonts);
        });

    let system_id = commands.register_one_shot_system(go_to_character_select);
    commands.insert_resource(SubmitCallback(system_id));

    commands.insert_resource(ControllerAssignment::default());
}

fn go_to_character_select(
    mut commands: Commands,
    ca: Res<ControllerAssignment>,
    mut state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Controllers {
        p1: ca.p1.unwrap(),
        p2: ca.p2.unwrap(),
    });

    state.set(GameState::CharacterSelect);
}

fn setup_areas(root: &mut ChildBuilder, fonts: &Fonts) {
    root.spawn((
        NodeBundle {
            background_color: MAIN_MENU_HIGHLIGHT_TEXT_COLOR.into(),
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Stretch,
                ..default()
            },
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
        NodeBundle {
            background_color: CONTROLLER_ASSIGNMENT_SIDE_COLOR.into(),
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                column_gap: Val::Percent(2.0),
                ..default()
            },
            ..default()
        },
        SelectedController(player),
    ))
    .with_children(|cb| {
        cb.spawn(TextBundle::from_section(
            format!("Player {}", player),
            TextStyle {
                font: fonts.basic.clone(),
                font_size: 60.0,
                ..default()
            },
        ));
    });
}

fn create_unused_controller_area(fonts: &Fonts, root: &mut ChildBuilder) {
    root.spawn((
        NodeBundle {
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                column_gap: Val::Percent(2.0),
                ..default()
            },
            ..default()
        },
        FreeControllers,
    ))
    .with_children(|cb| {
        cb.spawn(TextBundle::from_section(
            "Unassigned controllers",
            TextStyle {
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
    mut events: ResMut<MenuInputs>,
    callback: Res<SubmitCallback>,
    mut state: ResMut<NextState<GameState>>,
) {
    // TODO: Analog stick
    while let Some(ev) = events.pop_front() {
        match ev {
            GamepadEvent::Button(ev_btn) if ev_btn.value == 1.0 => match ev_btn.button_type {
                GamepadButtonType::DPadLeft => ca.left(ev_btn.gamepad),
                GamepadButtonType::DPadRight => ca.right(ev_btn.gamepad),
                GamepadButtonType::South => {
                    if ca.is_complete() {
                        commands.run_system(callback.0);
                    }
                }
                GamepadButtonType::East => {
                    state.set(GameState::MainMenu);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Debug, Component)]
pub struct ControllerIcon;

pub fn update_controller_assignment_menu_visuals(
    mut commands: Commands,
    ca: Res<ControllerAssignment>,
    pads: Res<Gamepads>,
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

    for pad in pads.iter() {
        if ca.p1 == Some(pad) {
            commands
                .entity(p1_selected)
                .with_children(create_icon(pad.id, &fonts));
        } else if ca.p2 == Some(pad) {
            commands
                .entity(p2_selected)
                .with_children(create_icon(pad.id, &fonts));
        } else {
            commands
                .entity(unused)
                .with_children(create_icon(pad.id, &fonts));
        }
    }
}

fn create_icon(id: usize, fonts: &Fonts) -> impl Fn(&mut ChildBuilder) {
    let font = fonts.basic.clone();
    move |cb: &mut ChildBuilder| {
        cb.spawn((
            TextBundle::from_section(
                id.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
            ),
            ControllerIcon,
        ));
    }
}
