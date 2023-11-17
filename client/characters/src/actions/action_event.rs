use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, DummyAnimation, ItemId, MizkuAnimation, SoundEffect, StatusCondition,
};

use crate::{Attack, Movement, ResourceType};

#[derive(Debug, Clone, PartialEq, Default, Reflect)]
pub enum ActionEvent {
    // TODO: Figure out a better way to handle actions that change depending on game state
    // Maybe hoist AnimationRequest?
    Animation(Animation),
    Consume(ItemId),
    RecipientAnimation(Animation),
    Sound(SoundEffect),
    StartAction(ActionId),
    Attack(Attack),
    ClearMovement,
    Movement(Movement),
    Condition(StatusCondition),
    #[default]
    ForceStand,
    ModifyResource(ResourceType, i32),
    ClearResource(ResourceType),
    SnapToOpponent,
    SideSwitch,
    HitStun(usize),
    BlockStun(usize),
    Launch,
}
impl From<Attack> for ActionEvent {
    fn from(value: Attack) -> Self {
        ActionEvent::Attack(value)
    }
}
impl From<Animation> for ActionEvent {
    fn from(value: Animation) -> Self {
        ActionEvent::Animation(value)
    }
}
impl From<Movement> for ActionEvent {
    fn from(value: Movement) -> Self {
        ActionEvent::Movement(value)
    }
}
// This isn't a great way to do this, but it's the best I can think of for now
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(value.into())
    }
}
impl From<MizkuAnimation> for ActionEvent {
    fn from(value: MizkuAnimation) -> Self {
        ActionEvent::Animation(value.into())
    }
}
