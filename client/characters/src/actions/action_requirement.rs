use wag_core::{ActionId, ItemId};

use crate::{ResourceType, Situation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionRequirement {
    Grounded,
    Airborne,
    OngoingAction(Vec<ActionId>),
    ItemsOwned(Vec<ItemId>),
    ResourceFull(ResourceType),
    ResourceValue(ResourceType, i32),
}
impl ActionRequirement {
    // If one condition fails, the whole thing fails.
    pub fn check(requirements: &Vec<ActionRequirement>, situation: &Situation) -> bool {
        for requirement in requirements {
            match requirement {
                ActionRequirement::Grounded => {
                    if !situation.grounded {
                        return false;
                    }
                }
                ActionRequirement::Airborne => {
                    if situation.grounded {
                        return false;
                    }
                }
                ActionRequirement::OngoingAction(ids) => {
                    let Some(tracker) = &situation.tracker else {
                        return false;
                    };

                    if !ids.contains(&tracker.action_id) {
                        return false;
                    }
                }
                ActionRequirement::ItemsOwned(ids) => {
                    if !ids.iter().any(|item| situation.inventory.contains(item)) {
                        return false;
                    }
                }
                ActionRequirement::ResourceFull(resource) => {
                    if situation
                        .resources
                        .get(resource)
                        .map(|r| r.is_full())
                        .unwrap_or_else(|| panic!("Character to have resource {:#?}", resource))
                    {
                        return false;
                    }
                }
                ActionRequirement::ResourceValue(resource, value) => {
                    if situation
                        .resources
                        .get(resource)
                        .map(|r| r.current >= *value)
                        .unwrap_or_else(|| panic!("Character to have resource {:#?}", resource))
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}