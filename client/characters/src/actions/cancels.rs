use core::panic;

use bevy::reflect::Reflect;
use wag_core::ActionId;

#[derive(Clone, Default, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect)]
pub enum CancelCategory {
    Any,
    Jump,
    Dash,
    Normal,
    CommandNormal,
    Special,
    #[default]
    Uncancellable,
    Everything, // Usable for tests as a "this is cancellable from anything that is cancellable"

    Specific(Vec<ActionId>),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct CancelRule {
    pub requires_hit: bool,
    pub category: CancelCategory,
}
impl CancelRule {
    pub fn never() -> Self {
        Self::default()
    }

    pub fn any() -> Self {
        Self {
            requires_hit: false,
            category: CancelCategory::Any,
        }
    }

    pub fn neutral_normal_recovery() -> Self {
        Self {
            requires_hit: true,
            category: CancelCategory::CommandNormal,
        }
    }

    pub fn command_normal_recovery() -> Self {
        Self {
            requires_hit: true,
            category: CancelCategory::Special,
        }
    }

    pub fn specific(targets: Vec<ActionId>) -> Self {
        Self {
            requires_hit: false,
            category: CancelCategory::Specific(targets),
        }
    }

    pub fn can_cancel(
        &self,
        hit: bool,
        action_id: ActionId,
        cancel_category: CancelCategory,
    ) -> bool {
        if self.category == CancelCategory::Uncancellable {
            return false;
        }

        if !hit && self.requires_hit {
            return false;
        }

        if let CancelCategory::Specific(options) = &self.category {
            if options.contains(&action_id) {
                return true;
            }
        }

        // self.rule will be elevated by one level to prevent self cancels
        if self.category <= cancel_category {
            return true;
        }
        false
    }

    pub fn cancel_out_of(category: CancelCategory) -> Self {
        match category {
            CancelCategory::Normal => Self::neutral_normal_recovery(),
            CancelCategory::CommandNormal => Self::command_normal_recovery(),
            _ => panic!("Cancels out of {:?} are not supported", category),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cancel_sanity_check() {
        assert!(CancelRule::any().can_cancel(true, ActionId::TestMove, CancelCategory::Everything));
        assert!(CancelRule::any().can_cancel(
            false,
            ActionId::TestMove,
            CancelCategory::Everything
        ));
        assert!(!CancelRule::never().can_cancel(
            true,
            ActionId::TestMove,
            CancelCategory::Everything
        ));
    }

    #[test]
    fn cancel_steps() {
        assert!(CancelRule::neutral_normal_recovery().can_cancel(
            true,
            ActionId::TestMove,
            CancelCategory::CommandNormal
        ));
        assert!(CancelRule::command_normal_recovery().can_cancel(
            true,
            ActionId::TestMove,
            CancelCategory::Special
        ));
    }
}
