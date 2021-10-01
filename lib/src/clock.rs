use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::labels::StartupStageLabel;
use crate::{Colors, Fonts};

#[derive(Inspectable, Default)]
pub struct Clock {
    pub frame: usize,
    elapsed_time: f32,
}
#[derive(Debug)]
pub struct Timer;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Clock::default())
            .add_startup_system_to_stage(StartupStageLabel::UI, setup.system())
            .add_system_to_stage(CoreStage::First, tick.system())
            .add_system(draw_timer.system());
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
                        "value",
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

fn draw_timer(mut query: Query<&mut Text, With<Timer>>, clock: Res<Clock>) {
    // TODO: Timer doesn't show up for some reason
    for mut text in query.iter_mut() {
        text.sections[0].value = (crate::ROUND_TIME - clock.elapsed_time).floor().to_string();
    }
}
