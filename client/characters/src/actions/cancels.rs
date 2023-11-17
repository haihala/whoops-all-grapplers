use bevy::reflect::Reflect;
use wag_core::ActionId;

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect)]
pub enum CancelCategory {
    Any,
    Jump,
    Dash,
    Normal,
    CommandNormal,
    Special,
    Everything, // Usable for tests as a "this is cancellable from anything that is cancellable"

    Specific(Vec<ActionId>),
}

#[derive(Clone, PartialEq, Eq, Debug, Reflect)]
pub struct CancelRule {
    pub requires_hit: bool,
    pub category: CancelCategory,
}

#[derive(Clone, PartialEq, Debug, Default, Reflect)]
pub struct CancelPolicy(pub Vec<CancelRule>);
impl CancelPolicy {
    pub fn never() -> Self {
        Self(vec![])
    }

    pub fn any() -> Self {
        Self(vec![CancelRule {
            requires_hit: false,
            category: CancelCategory::Any,
        }])
    }

    pub fn neutral_normal_recovery() -> Self {
        Self(vec![CancelRule {
            requires_hit: true,
            category: CancelCategory::CommandNormal,
        }])
    }

    pub fn command_normal_recovery() -> Self {
        Self(vec![CancelRule {
            requires_hit: true,
            category: CancelCategory::Special,
        }])
    }

    pub fn specific(targets: Vec<ActionId>) -> Self {
        Self(vec![CancelRule {
            requires_hit: false,
            category: CancelCategory::Specific(targets),
        }])
    }

    pub fn can_cancel(
        &self,
        hit: bool,
        action_id: ActionId,
        cancel_category: CancelCategory,
    ) -> bool {
        self.0.iter().any(|rule| {
            (hit || !rule.requires_hit)
                && (rule.category <= cancel_category
                    || if let CancelCategory::Specific(ids) = &rule.category {
                        ids.contains(&action_id)
                    } else {
                        false
                    })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cancel_sanity_check() {
        assert!(CancelPolicy::any().can_cancel(
            true,
            ActionId::TestMove,
            CancelCategory::Everything
        ));
        assert!(CancelPolicy::any().can_cancel(
            false,
            ActionId::TestMove,
            CancelCategory::Everything
        ));
        assert!(!CancelPolicy::never().can_cancel(
            true,
            ActionId::TestMove,
            CancelCategory::Everything
        ));
    }
}
