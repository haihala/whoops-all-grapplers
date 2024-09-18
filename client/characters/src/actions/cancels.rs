use bevy::prelude::*;
use bevy::reflect::Reflect;
use wag_core::ActionId;

use super::action::ActionCategory;

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct CancelRule {
    pub requires_hit: bool,
    pub category: ActionCategory,
    pub specifics: Vec<ActionId>,
}

impl Default for CancelRule {
    fn default() -> Self {
        Self {
            requires_hit: true,
            category: ActionCategory::Forced,
            specifics: vec![],
        }
    }
}
impl CancelRule {
    pub fn never() -> Self {
        Self::default()
    }

    pub fn any() -> Self {
        Self {
            requires_hit: false,
            category: ActionCategory::Dash,
            ..default()
        }
    }

    pub fn dash() -> Self {
        Self {
            requires_hit: false,
            category: ActionCategory::Jump,
            ..default()
        }
    }

    pub fn jump() -> Self {
        Self {
            requires_hit: false,
            category: ActionCategory::Normal,
            ..default()
        }
    }

    pub fn normal_recovery() -> Self {
        Self {
            requires_hit: true,
            category: ActionCategory::Special,
            ..default()
        }
    }

    pub fn special_recovery() -> Self {
        Self {
            requires_hit: true,
            category: ActionCategory::Super,
            ..default()
        }
    }

    pub fn specific(targets: Vec<ActionId>) -> Self {
        Self {
            requires_hit: false,
            specifics: targets,
            ..default()
        }
    }

    pub fn specific_or_category(targets: Vec<ActionId>, category: ActionCategory) -> Self {
        Self {
            requires_hit: false,
            specifics: targets,
            category,
        }
    }

    pub fn can_cancel(&self, hit: bool, action_id: ActionId, action_type: ActionCategory) -> bool {
        if !hit && self.requires_hit {
            return false;
        }

        if self.specifics.contains(&action_id) {
            return true;
        }

        action_type.can_be_standard_cancelled_into() && self.category <= action_type
    }

    pub fn cancel_out_of(category: ActionCategory) -> Self {
        match category {
            ActionCategory::Normal => Self::normal_recovery(),
            ActionCategory::Special => Self::special_recovery(),
            _ => panic!("Cancels out of {:?} are not supported", category),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cancel_sanity_check() {
        assert!(CancelRule::any().can_cancel(true, ActionId::TestMove, ActionCategory::Special));
        assert!(CancelRule::any().can_cancel(false, ActionId::TestMove, ActionCategory::Special));
        assert!(!CancelRule::never().can_cancel(true, ActionId::TestMove, ActionCategory::Special));
    }

    #[test]
    fn cancel_steps() {
        assert!(!CancelRule::normal_recovery().can_cancel(
            true,
            ActionId::TestMove,
            ActionCategory::Normal
        ));
        assert!(CancelRule::normal_recovery().can_cancel(
            true,
            ActionId::TestMove,
            ActionCategory::Special
        ));
    }
}
