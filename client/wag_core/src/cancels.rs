use bevy::prelude::*;

use crate::{ActionId, METERED_KARA_WINDOW};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect, Default, Hash)]
pub enum ActionCategory {
    Dash,
    Jump,
    Other, // For gi parry and fast fall
    #[default]
    Normal,
    Special,
    Super,
    MegaInterrupt,
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
    pub fn kara_to(action: impl Into<ActionId>) -> Self {
        CancelWindow {
            require_hit: false,
            cancel_type: CancelType::Specific(vec![action.into()]),
            duration: METERED_KARA_WINDOW,
        }
    }

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
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn open(&mut self, new: CancelWindow, frame: usize) {
        self.0.retain(|window| window.to >= frame);
        self.0.push(new.open_at(frame));
    }
}
