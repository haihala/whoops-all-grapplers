use bevy::prelude::*;

use crate::ActionId;

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect, Default)]
pub enum ActionCategory {
    Dash,
    Jump,
    Throw,
    Other,
    #[default]
    Normal,
    Special,
    Super,
    FollowUp,
    Forced, // For throw recipients
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum CancelType {
    #[default]
    Special,
    Super,
    Specific(Vec<ActionId>),
    Anything,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CancelWindow {
    pub duration: usize,
    pub require_hit: bool,
    pub cancel_type: CancelType,
}
impl CancelWindow {
    pub fn open_at(self, frame: usize) -> OpenCancelWindow {
        OpenCancelWindow {
            from: frame,
            to: frame + self.duration,
            require_hit: self.require_hit,
            cancel_type: self.cancel_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenCancelWindow {
    pub from: usize,
    pub to: usize,
    pub require_hit: bool,
    pub cancel_type: CancelType,
}

#[derive(Debug, Default, Component, Clone)]
pub struct AvailableCancels(pub Vec<OpenCancelWindow>);
impl AvailableCancels {
    pub fn can_cancel_to(&self, category: ActionCategory, id: ActionId, has_hit: bool) -> bool {
        for win in self.0.iter() {
            let matching_cancel = match win.cancel_type {
                CancelType::Special => {
                    matches!(category, ActionCategory::Special | ActionCategory::Super)
                }
                CancelType::Super => category == ActionCategory::Super,
                CancelType::Specific(ref options) => options.contains(&id),
                CancelType::Anything => true,
            };

            if matching_cancel && (has_hit || !win.require_hit) {
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn open(&mut self, new: CancelWindow, frame: usize) {
        self.0.retain(|window| window.to >= frame);
        self.0.push(new.open_at(frame));
    }
}
