use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game_flow::GameState;
use crate::labels::StartupStageLabel;
use crate::{Colors, Fonts};

#[derive(Inspectable, Default)]
pub struct Clock {
    pub frame: usize,
    elapsed_time: f32,
}
impl Clock {
    pub fn time_out(&self) -> bool {
        self.elapsed_time >= constants::ROUND_TIME - 1.0
    }

    fn reset(&mut self) {
        self.frame = 0;
        self.elapsed_time = 0.0;
    }
}
#[derive(Debug)]
pub struct Timer;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Clock::default())
            .add_startup_system_to_stage(StartupStageLabel::UI, setup.system())
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(constants::FPS_F64))
                    .with_system(tick.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Combat).with_system(update_timer.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Combat).with_system(reset_timer.system()),
            );
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>, colors: Res<Colors>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position: Rect {
                    top: Val::Percent(5.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: colors.transparent.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        constants::ROUND_TIME.round().to_string(),
                        TextStyle {
                            font: fonts.basic.clone(),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(Timer);
        });
}

fn tick(mut clock: ResMut<Clock>, bevy_clock: Res<Time>) {
    clock.frame += 1;
    clock.elapsed_time += bevy_clock.delta_seconds();
}

fn update_timer(mut query: Query<&mut Text, With<Timer>>, clock: Res<Clock>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = (constants::ROUND_TIME - clock.elapsed_time)
        .floor()
        .to_string();
}

fn reset_timer(mut clock: ResMut<Clock>) {
    clock.reset();
}
