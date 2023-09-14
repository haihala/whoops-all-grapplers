#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum CancelCategory {
    Any,
    Jump,
    Dash,
    Normal,
    CommandNormal,
    Special,
    Everything, // Usable for tests as a "this is cancellable from anything that is cancellable"
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CancelRule {
    pub requires_hit: bool,
    pub category: CancelCategory,
}

#[derive(Clone, PartialEq, Debug, Default)]
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

    pub fn can_cancel(&self, hit: bool, cancel_category: CancelCategory) -> bool {
        self.0
            .iter()
            .any(|rule| (hit || !rule.requires_hit) && rule.category <= cancel_category)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cancel_sanity_check() {
        assert!(CancelPolicy::any().can_cancel(true, CancelCategory::Everything));
        assert!(CancelPolicy::any().can_cancel(false, CancelCategory::Everything));
        assert!(!CancelPolicy::never().can_cancel(true, CancelCategory::Everything));
    }
}
