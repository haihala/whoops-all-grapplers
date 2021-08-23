use bevy::prelude::*;

use crate::Materials;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(detect_new_pads.system())
            .add_system(move_players.system());
    }
}

pub struct Player;
#[derive(Debug)]
pub struct Controller(Gamepad);

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let width = 10.;
    let height = 15.;

    commands
        .spawn_bundle(SpriteBundle {
            material: assets.collision_box_color.clone(),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(Player);
}

fn detect_new_pads(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut controlled: Query<(Entity, &mut Controller)>,
    uncontrolled: Query<Entity, (With<Player>, Without<Controller>)>,
    mut unused_pads: Option<ResMut<Vec<Controller>>>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);
                match uncontrolled.single() {
                    Ok(entity) => {
                        commands.entity(entity).insert(Controller(*id));
                    }
                    Err(_) => {
                        let new_controller = Controller(*id);
                        match unused_pads {
                            Some(ref mut queue) => {
                                queue.push(new_controller);
                            }
                            None => {
                                commands.insert_resource(vec![new_controller]);
                            }
                        };
                    }
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);
                for (entity, mut controller) in controlled.iter_mut() {
                    if controller.0 == *id {
                        match unused_pads {
                            Some(ref mut queue) => {
                                if queue.len() > 0 {
                                    controller.0 = queue.pop().unwrap().0;
                                } else {
                                    commands.entity(entity).remove::<Controller>();
                                }
                            }
                            None => {
                                commands.entity(entity).remove::<Controller>();
                            }
                        };
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn move_players(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut players: Query<(&mut Transform, &Controller)>,
) {
    for (mut transform, controller) in players.iter_mut() {
        let lstick_x_axis = GamepadAxis(controller.0, GamepadAxisType::LeftStickX);
        let lstick_y_axis = GamepadAxis(controller.0, GamepadAxisType::LeftStickY);

        let stick = match (axes.get(lstick_x_axis), axes.get(lstick_y_axis)) {
            (Some(stick_x), Some(stick_y)) => {
                let stick = Vec3::new(stick_x, stick_y, 0.0);
                if stick.length() > 0.1 {
                    stick
                } else {
                    Vec3::ZERO
                }
            }
            _ => Vec3::ZERO,
        };

        let up_button = GamepadButton(controller.0, GamepadButtonType::DPadUp);
        let down_button = GamepadButton(controller.0, GamepadButtonType::DPadDown);
        let left_button = GamepadButton(controller.0, GamepadButtonType::DPadLeft);
        let right_button = GamepadButton(controller.0, GamepadButtonType::DPadRight);
        let dpad_x = (buttons.pressed(right_button) as i32) - (buttons.pressed(left_button) as i32);
        let dpad_y = (buttons.pressed(up_button) as i32) - (buttons.pressed(down_button) as i32);
        let dpad = Vec3::new(dpad_x as f32, dpad_y as f32, 0.0);

        let total = dpad + stick;

        transform.translation += if total.length() != 0.0 {
            if total.length() > 1.0 {
                total.normalize()
            } else {
                total
            }
        } else {
            Vec3::ZERO
        };
    }
}
