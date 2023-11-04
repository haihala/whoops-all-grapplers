use bevy::prelude::*;

use wag_core::{ActionId, Animation, DummyAnimation, ItemId, SoundEffect, StatusCondition};

use crate::{Attack, Movement, ResourceType};

#[derive(Debug, Clone, PartialEq, Default, Reflect)]
pub enum ActionEvent {
    // TODO: Figure out a better way to handle actions that change depending on game state
    // Maybe hoist AnimationRequest?
    Animation(Animation),
    Consume(ItemId),
    RecipientAnimation(Animation),
    Sound(SoundEffect),
    Move(ActionId),
    Attack(Attack),
    Movement(Movement),
    Condition(StatusCondition),
    #[default]
    ForceStand,
    ModifyProperty(ResourceType, i32),
    ClearProperty(ResourceType),
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
impl From<DummyAnimation> for ActionEvent {
    fn from(value: DummyAnimation) -> Self {
        ActionEvent::Animation(Animation::Dummy(value))
    }
}
