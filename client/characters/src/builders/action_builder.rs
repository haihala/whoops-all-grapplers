use std::sync::Arc;

use bevy::{prelude::*, utils::HashMap};
use wag_core::{ActionCategory, ActionId, Animation, GameButton, SoundEffect, METER_BAR_SEGMENT};

use crate::{
    Action, ActionEvent, ActionRequirement, AnimationRequest, FlashRequest, ResourceType, Situation,
};

use super::DynamicEvents;

#[derive(Debug)]
pub struct CharacterUniversals {
    pub normal_grunt: SoundEffect,
}

#[derive(Clone, Default)]
struct Events {
    constant: Vec<ActionEvent>,
    dynamic: Option<DynamicEvents>,
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
                    format!("{}|123", game_button.to_dsl())
                } else {
                    game_button.to_dsl()
                }
            }
            Input::Motion(dsl) => dsl.clone(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    Air,
    #[default]
    Stand,
    Crouch,
}

#[derive(Default)]
pub struct ActionBuilder {
    input: Option<Input>,
    state: State,
    category: ActionCategory,
    blobs: Vec<EventBlob>,
    needs_charge: bool,
    needs_meter: bool,
    extra_requirements: Vec<ActionRequirement>,
    follow_up_from: Option<Vec<ActionId>>,
}

impl ActionBuilder {
    pub fn other() -> Self {
        Self {
            category: ActionCategory::Other,
            ..default()
        }
    }

    pub fn special() -> Self {
        Self {
            category: ActionCategory::Special,
            ..default()
        }
    }

    pub fn normal() -> Self {
        Self {
            category: ActionCategory::Normal,
            ..default()
        }
    }

    pub fn button(btn: GameButton) -> Self {
        Self {
            input: Some(Input::Button(btn)),
            ..Self::normal()
        }
    }

    pub fn with_character_universals(self, universals: CharacterUniversals) -> Self {
        if self.category == ActionCategory::Normal {
            return self.with_sound(universals.normal_grunt);
        }

        self
    }

    pub fn follow_up_from(self, actions: Vec<ActionId>) -> Self {
        Self {
            follow_up_from: Some(actions),
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
            state: State::Crouch,
            ..self
        }
    }

    pub fn air_only(self) -> Self {
        Self {
            state: State::Air,
            ..self
        }
    }

    pub fn immediate_events(mut self, events: Vec<ActionEvent>) -> Self {
        self.blobs.push(EventBlob {
            events: Events {
                constant: events,
                ..default()
            },
            timing: Timing::OnFrame(0),
        });
        self
    }

    pub fn events_on_frame(mut self, frame: usize, events: Vec<ActionEvent>) -> Self {
        self.blobs.push(EventBlob {
            events: Events {
                constant: events,
                ..default()
            },
            timing: Timing::OnFrame(frame),
        });
        self
    }

    pub fn events_after_frame(mut self, frame: usize, events: Vec<ActionEvent>) -> Self {
        self.blobs.push(EventBlob {
            events: Events {
                constant: events,
                ..default()
            },
            timing: Timing::After(frame),
        });
        self
    }

    pub fn dyn_immediate_events(mut self, events: DynamicEvents) -> Self {
        self.blobs.push(EventBlob {
            events: Events {
                dynamic: Some(events),
                ..default()
            },
            timing: Timing::OnFrame(0),
        });
        self
    }

    pub fn dyn_events_on_frame(mut self, frame: usize, events: DynamicEvents) -> Self {
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
        self.blobs.push(EventBlob {
            events: Events {
                dynamic: Some(events),
                ..default()
            },
            timing: Timing::After(frame),
        });
        self
    }

    pub fn end_at(self, frame: usize) -> Self {
        self.events_after_frame(frame, vec![ActionEvent::End])
    }

    pub fn with_sound(self, sound: SoundEffect) -> Self {
        self.immediate_events(vec![ActionEvent::Sound(sound)])
    }

    pub fn with_animation(self, animation: impl Into<Animation>) -> Self {
        let anim = AnimationRequest::from(animation.into());
        self.immediate_events(vec![ActionEvent::Animation(anim)])
    }

    pub fn with_requirement(mut self, extra_requirement: ActionRequirement) -> Self {
        self.extra_requirements.push(extra_requirement);
        self
    }

    pub fn build_input(&self) -> Option<String> {
        self.input
            .clone()
            .map(|input| input.to_dsl(self.state == State::Crouch))
    }

    pub fn build_requirements(&self) -> ActionRequirement {
        let mut temp = self.extra_requirements.clone();

        temp.push(match self.state {
            State::Air => ActionRequirement::Airborne,
            State::Stand | State::Crouch => ActionRequirement::Grounded,
        });

        if self.needs_meter {
            temp.push(ActionRequirement::ResourceValue(
                ResourceType::Meter,
                METER_BAR_SEGMENT,
            ));
        }

        if self.needs_charge {
            temp.push(ActionRequirement::ResourceFull(ResourceType::Charge));
        }

        if let Some(ongoing) = self.follow_up_from.clone() {
            temp.push(ActionRequirement::ActionOngoing(ongoing));
        }
        temp.push(ActionRequirement::Starter(self.category));

        ActionRequirement::And(temp)
    }

    pub fn build_script(mut self) -> impl Fn(&Situation) -> Vec<ActionEvent> {
        if self.needs_meter {
            self = self.immediate_events(vec![
                ActionEvent::ModifyResource(ResourceType::Meter, METER_BAR_SEGMENT),
                ActionEvent::Flash(FlashRequest::meter_use()),
            ]);
        }
        if self.needs_charge {
            self = self.immediate_events(vec![ActionEvent::ClearResource(ResourceType::Charge)]);
        }
        match self.state {
            State::Air => {}
            State::Stand => self = self.immediate_events(vec![ActionEvent::ForceStand]),
            State::Crouch => self = self.immediate_events(vec![ActionEvent::ForceCrouch]),
        }

        let folded_events: Vec<(Timing, Events)> = self
            .blobs
            .clone()
            .into_iter()
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
            input: self.build_input(),
            requirement: self.build_requirements(),
            script: Box::new(self.build_script()),
        }
    }
}
