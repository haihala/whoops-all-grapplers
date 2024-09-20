use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, CancelWindow, DummyAnimation, ItemId, MizkuAnimation, SoundEffect,
    StatusCondition, VfxRequest,
};

use crate::{Attack, FlashRequest, Movement, ResourceType};

use super::AnimationRequest;

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct ActionEvents {
    events: Vec<ActionEvent>,
}
impl ActionEvents {
    pub fn get_matching_events<T>(&self, predicate: impl Fn(&ActionEvent) -> Option<T>) -> Vec<T> {
        self.events.iter().filter_map(predicate).collect()
    }

    pub fn add_events(&mut self, actions: Vec<ActionEvent>) {
        self.events.extend(
            actions
                .into_iter()
                .filter(|ev| !matches!(ev, ActionEvent::Noop)),
        );
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ActionEvent {
    AllowCancel(CancelWindow),
    Animation(AnimationRequest),
    Consume(ItemId),
    Sound(SoundEffect),
    StartAction(ActionId),
    Attack(Attack),
    ClearMovement,
    Movement(Movement),
    Condition(StatusCondition),
    ForceStand,
    ModifyResource(ResourceType, i32),
    ClearResource(ResourceType),
    SnapToOpponent,
    SideSwitch,
    HitStun(usize),
    BlockStun(usize),
    Launch {
        impulse: Vec2,
    },
    Hitstop, // TODO: Add strength
    CameraTilt(Vec2),
    CameraShake, // TODO: Add strength
    Flash(FlashRequest),
    VisualEffect(VfxRequest),
    Lock(usize), // duration, sideswitch
    #[default]
    Noop, // makes writing macros easier
    End,         // Ends the move, return to neutral
}

impl From<Attack> for ActionEvent {
    fn from(value: Attack) -> Self {
        ActionEvent::Attack(value)
    }
}
impl From<Animation> for ActionEvent {
    fn from(value: Animation) -> Self {
        ActionEvent::Animation(value.into())
    }
}
impl From<Movement> for ActionEvent {
    fn from(value: Movement) -> Self {
        ActionEvent::Movement(value)
    }
}
impl From<AnimationRequest> for ActionEvent {
    fn from(value: AnimationRequest) -> Self {
        ActionEvent::Animation(value)
    }
}
impl From<SoundEffect> for ActionEvent {
    fn from(value: SoundEffect) -> Self {
        ActionEvent::Sound(value)
    }
}
impl From<VfxRequest> for ActionEvent {
    fn from(value: VfxRequest) -> Self {
        ActionEvent::VisualEffect(value)
    }
}
// This isn't a great way to do this, but it's the best I can think of for now
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
impl From<MizkuAnimation> for ActionEvent {
    fn from(value: MizkuAnimation) -> Self {
        ActionEvent::Animation(Animation::from(value).into())
    }
}
