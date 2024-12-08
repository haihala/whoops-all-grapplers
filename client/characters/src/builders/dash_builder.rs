use std::{f32::consts::PI, sync::Arc};

use bevy::prelude::*;

use wag_core::{
    ActionCategory, ActionId, Animation, ItemId, StatusCondition, StatusFlag, VfxRequest,
    VisualEffect,
};

use crate::{Action, ActionEvent, ActionRequirement, Movement, Situation};

use super::{ActionBuilder, CharacterUniversals};

#[derive(Debug, Default)]
pub struct DashBuilder {
    backdash: bool,
    animation: Option<Animation>,
    phases: Vec<(usize, Movement)>,
    universals: Option<CharacterUniversals>,
    total_duration: usize,
}

impl DashBuilder {
    fn dash(&self, metered: bool) -> Action {
        let mut builder = if metered {
            ActionBuilder::for_category(ActionCategory::MegaInterrupt)
                .with_meter_cost()
                .with_requirement(ActionRequirement::ItemOwned(ItemId::TrackSpikes))
                // This prevents using track spikes in neutral
                .with_requirement(ActionRequirement::AnyActionOngoing)
                // This prevents immediately using bar when doing dashes in neutral
                .with_requirement(ActionRequirement::ActionNotOngoing(vec![
                    ActionId::DashForward,
                    ActionId::DashBack,
                    ActionId::TrackSpikesDashForward,
                    ActionId::TrackSpikesDashBack,
                ]))
        } else {
            ActionBuilder::for_category(ActionCategory::Dash)
        };

        let (input, vfx_rotation) = if self.backdash {
            builder = builder.dyn_immediate_events(Arc::new(move |situation: &Situation| {
                if situation.stats.backdash_invuln > 0 {
                    vec![ActionEvent::Condition(StatusCondition {
                        flag: StatusFlag::Intangible,
                        expiration: Some(situation.stats.backdash_invuln),
                        ..default()
                    })]
                } else {
                    vec![]
                }
            }));

            ("454", -PI / 2.0)
        } else {
            ("656", PI / 2.0)
        };

        debug_assert_ne!(self.total_duration, 0);

        builder = builder
            .with_input(input)
            .with_animation(self.animation.unwrap())
            .static_immediate_events(vec![VfxRequest {
                effect: VisualEffect::SpeedLines,
                tf: Transform {
                    translation: Vec3::new(0.0, 1.3, 0.0),
                    rotation: Quat::from_rotation_z(vfx_rotation),
                    ..default()
                },
                ..default()
            }
            .into()])
            .with_character_universals(self.universals.unwrap())
            .end_at(self.total_duration);

        for (frame, movement) in &self.phases {
            let mut evs = vec![(*movement).into()];
            let goes_up = movement.amount.y > 0.0;
            if goes_up {
                evs.push(ActionEvent::ForceAir);
            }
            builder = builder.static_events_on_frame(*frame, evs);
        }

        builder.build()
    }

    pub fn build(self) -> impl Iterator<Item = (ActionId, Action)> {
        debug_assert!(!self.phases.is_empty());

        let (basic_action, super_action) = if self.backdash {
            (ActionId::DashBack, ActionId::TrackSpikesDashBack)
        } else {
            (ActionId::DashForward, ActionId::TrackSpikesDashForward)
        };

        vec![
            (basic_action, self.dash(false)),
            (super_action, self.dash(true)),
        ]
        .into_iter()
    }

    pub fn back() -> Self {
        Self {
            backdash: true,
            ..default()
        }
    }

    pub fn forward() -> Self {
        Self::default()
    }

    pub fn with_character_universals(self, universals: CharacterUniversals) -> Self {
        Self {
            universals: Some(universals),
            ..self
        }
    }

    pub fn with_animation(self, anim: impl Into<Animation>) -> Self {
        Self {
            animation: Some(anim.into()),
            ..self
        }
    }

    pub fn on_frame(mut self, frame: usize, movement: Movement) -> Self {
        self.phases.push((
            frame,
            Movement {
                duration: movement.duration,
                amount: if self.backdash {
                    Vec2::new(
                        if self.backdash { -1.0 } else { 1.0 } * movement.amount.x,
                        movement.amount.y,
                    )
                } else {
                    movement.amount
                },
            },
        ));
        self
    }

    pub fn end_at(self, frame: usize) -> Self {
        Self {
            total_duration: frame,
            ..self
        }
    }
}
