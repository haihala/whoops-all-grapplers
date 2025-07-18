use std::sync::Arc;

use bevy::{platform::collections::HashMap, prelude::*};
use foundation::{
    ActionCategory, ActionId, Animation, GameButton, SimpleState, Sound, VfxRequest, VisualEffect,
    METER_BAR_SEGMENT,
};

use crate::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, FlashRequest, GaugeType, Situation,
};

use super::DynamicEvents;

#[derive(Debug, Clone, Copy)]
pub struct CharacterUniversals {
    pub normal_grunt: Sound,
    pub primary_color: Color,
    pub secondary_color: Color,
}

#[derive(Clone, Default)]
pub struct Events {
    pub constant: Vec<ActionEvent>,
    pub dynamic: Option<DynamicEvents>,
}

impl std::fmt::Debug for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Events, constant: {:?}, dynamic is_some: {}",
            self.constant,
            self.dynamic.is_some()
        )
    }
}

impl Events {
    fn merge_with(self, other: Events) -> Events {
        Self {
            constant: [self.constant, other.constant].concat(),
            dynamic: match (self.dynamic, other.dynamic) {
                (None, None) => None,
                (Some(val), None) | (None, Some(val)) => Some(val),
                (Some(a), Some(b)) => Some(Arc::new(move |situation: &Situation| {
                    [a.clone()(situation), b.clone()(situation)].concat()
                })),
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Timing {
    OnFrame(usize),
    After(usize),
    Always,
}

#[derive(Clone)]
struct EventBlob {
    timing: Timing,
    events: Events,
}

#[derive(Debug, Clone)]
enum Input {
    Button(GameButton),
    Motion(String),
}

impl Input {
    fn to_dsl(&self, crouching: bool) -> String {
        match self {
            Input::Button(game_button) => {
                if crouching {
                    format!("{{123}}{}", game_button.to_dsl())
                } else {
                    game_button.to_dsl()
                }
            }
            Input::Motion(dsl) => dsl.clone(),
        }
    }
}

#[derive(Default)]
pub struct ActionBuilder {
    input: Option<Input>,
    transient: bool,
    pub state: Option<SimpleState>,
    pub category: ActionCategory,
    blobs: Vec<EventBlob>,
    needs_charge: bool,
    needs_meter: bool,
    extra_requirements: Vec<ActionRequirement>,
    pub follows_up_from: Option<Vec<ActionId>>,
    pub total_duration: usize,
    pub character_universals: Option<CharacterUniversals>,
}

impl ActionBuilder {
    pub fn for_category(category: ActionCategory) -> Self {
        Self {
            category,
            state: Some(SimpleState::Stand),
            ..default()
        }
    }

    pub fn special() -> Self {
        Self::for_category(ActionCategory::Special)
    }

    pub fn normal() -> Self {
        Self::for_category(ActionCategory::Normal)
    }

    pub fn button(btn: GameButton) -> Self {
        Self {
            input: Some(Input::Button(btn)),
            ..Self::normal()
        }
    }

    pub fn make_transient(mut self) -> Self {
        debug_assert!(self.total_duration == 0);
        self.transient = true;
        self
    }

    pub fn with_character_universals(mut self, universals: CharacterUniversals) -> Self {
        if self.category == ActionCategory::Normal {
            self = self.with_sound(universals.normal_grunt);
        }

        self.character_universals = Some(universals);
        self
    }

    pub fn follow_up_from(self, actions: Vec<ActionId>) -> Self {
        Self {
            follows_up_from: Some(actions),
            ..self
        }
    }

    pub fn with_input(self, input: &'static str) -> Self {
        Self {
            input: Some(Input::Motion(input.into())),
            ..self
        }
    }

    pub fn with_meter_cost(self) -> Self {
        Self {
            needs_meter: true,
            ..self
        }
    }
    pub fn with_charge(self) -> Self {
        Self {
            needs_charge: true,
            ..self
        }
    }

    pub fn crouching(self) -> Self {
        Self {
            state: Some(SimpleState::Crouch),
            ..self
        }
    }

    pub fn air_only(self) -> Self {
        Self {
            state: Some(SimpleState::Air),
            ..self
        }
    }

    pub fn air_or_ground(self) -> Self {
        Self {
            state: None,
            ..self
        }
    }

    #[allow(unused)]
    pub fn immediate_events(mut self, events: Events) -> Self {
        self.blobs.push(EventBlob {
            events,
            timing: Timing::OnFrame(0),
        });
        self
    }

    pub fn events_on_frame(mut self, frame: usize, events: Events) -> Self {
        self.blobs.push(EventBlob {
            events,
            timing: Timing::OnFrame(frame),
        });
        self
    }

    #[allow(unused)]
    pub fn events_after_frame(mut self, frame: usize, events: Events) -> Self {
        self.blobs.push(EventBlob {
            events,
            timing: Timing::After(frame),
        });
        self
    }

    pub fn every_frame(mut self, events: Events) -> Self {
        self.blobs.push(EventBlob {
            events,
            timing: Timing::Always,
        });
        self
    }

    pub fn static_immediate_events(self, events: Vec<ActionEvent>) -> Self {
        self.static_events_on_frame(0, events)
    }

    pub fn static_events_on_frame(mut self, frame: usize, events: Vec<ActionEvent>) -> Self {
        debug_assert!(!self.transient);
        self.blobs.push(EventBlob {
            events: Events {
                constant: events,
                ..default()
            },
            timing: Timing::OnFrame(frame),
        });
        self
    }

    pub fn static_events_after_frame(mut self, frame: usize, events: Vec<ActionEvent>) -> Self {
        debug_assert!(!self.transient);
        self.blobs.push(EventBlob {
            events: Events {
                constant: events,
                ..default()
            },
            timing: Timing::After(frame),
        });
        self
    }

    pub fn dyn_immediate_events(self, events: DynamicEvents) -> Self {
        self.dyn_events_on_frame(0, events)
    }

    pub fn dyn_events_on_frame(mut self, frame: usize, events: DynamicEvents) -> Self {
        debug_assert!(!self.transient);
        self.blobs.push(EventBlob {
            events: Events {
                dynamic: Some(events),
                ..default()
            },
            timing: Timing::OnFrame(frame),
        });
        self
    }

    pub fn dyn_events_after_frame(mut self, frame: usize, events: DynamicEvents) -> Self {
        debug_assert!(!self.transient);
        self.blobs.push(EventBlob {
            events: Events {
                dynamic: Some(events),
                ..default()
            },
            timing: Timing::After(frame),
        });
        self
    }

    pub fn end_at(mut self, frame: usize) -> Self {
        debug_assert!(!self.transient);
        self.total_duration = frame;
        self
    }

    pub fn with_sound(self, sound: Sound) -> Self {
        self.static_immediate_events(vec![ActionEvent::Sound(sound.into())])
    }

    pub fn with_animation(self, animation: impl Into<Animation>) -> Self {
        let anim = AnimationRequest::from(animation.into());
        self.static_immediate_events(vec![ActionEvent::Animation(anim)])
    }

    pub fn with_requirement(mut self, extra_requirement: ActionRequirement) -> Self {
        self.extra_requirements.push(extra_requirement);
        self
    }

    pub fn with_vfx_on_frame(self, frame: usize, effect: VisualEffect, tf: Transform) -> Self {
        self.dyn_events_on_frame(
            frame,
            Arc::new(move |situation: &Situation| {
                vec![ActionEvent::RelativeVisualEffect(VfxRequest {
                    effect: effect.clone(),
                    tf: situation.facing.visual.mirror_transform(tf),
                    ..default()
                })]
            }),
        )
    }

    pub fn build_input(&self) -> Option<String> {
        self.input
            .clone()
            .map(|input| input.to_dsl(self.state == Some(SimpleState::Crouch)))
    }

    pub fn build_requirements(&self) -> ActionRequirement {
        let mut temp = self.extra_requirements.clone();

        if let Some(state) = self.state {
            temp.push(match state {
                SimpleState::Air => ActionRequirement::Airborne,
                SimpleState::Stand | SimpleState::Crouch => ActionRequirement::Grounded,
            });
        }

        if self.needs_meter {
            temp.push(ActionRequirement::ResourceValue(
                GaugeType::Meter,
                METER_BAR_SEGMENT,
            ));
        }

        if self.needs_charge {
            temp.push(ActionRequirement::ResourceFull(GaugeType::Charge));
        }

        if let Some(ongoing) = self.follows_up_from.clone() {
            temp.push(ActionRequirement::ActionOngoing(ongoing));
        }

        temp.push(ActionRequirement::Starter(self.category));

        ActionRequirement::And(temp)
    }

    pub fn build_script(mut self) -> impl Fn(&Situation) -> Vec<ActionEvent> {
        if self.needs_meter {
            self = self.static_immediate_events(vec![
                ActionEvent::ModifyResource(GaugeType::Meter, -METER_BAR_SEGMENT),
                ActionEvent::Flash(FlashRequest::meter_use()),
            ]);
        }
        if self.needs_charge {
            self =
                self.static_immediate_events(vec![ActionEvent::ClearResource(GaugeType::Charge)]);
        }

        if !self.transient {
            if let Some(state) = self.state {
                match state {
                    SimpleState::Air => {}
                    SimpleState::Stand => {
                        self = self.static_immediate_events(vec![ActionEvent::ForceStand])
                    }
                    SimpleState::Crouch => {
                        self = self.static_immediate_events(vec![ActionEvent::ForceCrouch])
                    }
                };
            }
        }

        let folded_events: Vec<(Timing, Events)> = self
            .blobs
            .clone()
            .into_iter()
            .chain(if !self.transient {
                vec![EventBlob {
                    timing: Timing::After(self.total_duration),
                    events: Events {
                        constant: vec![ActionEvent::End],
                        ..default()
                    },
                }]
            } else {
                vec![]
            })
            .fold(HashMap::<Timing, Events>::new(), |mut res, new| {
                if let Some(events) = res.get(&new.timing) {
                    // This timing already exists
                    res.insert(new.timing, events.clone().merge_with(new.events));
                } else {
                    // New timing
                    res.insert(new.timing, new.events);
                }

                res
            })
            .into_iter()
            .collect();

        move |situation: &Situation| {
            let mut out = vec![];

            for (timing, events) in &folded_events {
                if match timing {
                    Timing::OnFrame(frame) => situation.on_frame(*frame),
                    Timing::After(frame) => situation.after_frame(*frame),
                    Timing::Always => true,
                } {
                    out.extend(events.constant.clone());
                    if let Some(generator) = events.dynamic.clone() {
                        out.extend(generator(situation));
                    }
                }
            }

            out
        }
    }

    pub fn build(self) -> Action {
        Action {
            transient: self.transient,
            input: self.build_input(),
            requirement: self.build_requirements(),
            script: Box::new(self.build_script()),
        }
    }
}
