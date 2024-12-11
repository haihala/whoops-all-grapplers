use foundation::{ActionCategory, ActionId, CancelType, GameButton, ItemId, StatusFlag};

use crate::{GaugeType, Situation};

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub enum ActionRequirement {
    #[default]
    None,
    Grounded,
    Airborne,
    AnyActionOngoing,
    ActionOngoing(Vec<ActionId>),
    ActionNotOngoing(Vec<ActionId>),
    ItemOwned(ItemId),
    ResourceFull(GaugeType),
    ResourceValue(GaugeType, i32),
    ButtonPressed(GameButton),
    ButtonNotPressed(GameButton),
    StatusNotActive(StatusFlag),
    Starter(ActionCategory),
    And(Vec<ActionRequirement>),
    Or(Vec<ActionRequirement>),
}
impl ActionRequirement {
    pub fn check(
        &self,
        self_id: ActionId,
        windows: &Vec<CancelType>,
        situation: &Situation,
    ) -> bool {
        match self {
            ActionRequirement::None => true,
            ActionRequirement::Grounded => situation.grounded,
            ActionRequirement::Airborne => !situation.grounded,
            ActionRequirement::ActionOngoing(ids) => {
                let Some(tracker) = &situation.tracker else {
                    return false;
                };

                ids.contains(&tracker.action_id)
            }
            ActionRequirement::ActionNotOngoing(ids) => {
                let Some(tracker) = &situation.tracker else {
                    return true;
                };

                !ids.contains(&tracker.action_id)
            }
            ActionRequirement::AnyActionOngoing => situation.tracker.is_some(),
            ActionRequirement::ItemOwned(item_id) => situation.inventory.contains(item_id),
            ActionRequirement::ResourceFull(resource) => situation
                .get_resource(*resource)
                .expect("Character to have resource")
                .is_full(),
            ActionRequirement::ResourceValue(resource, value) => {
                situation
                    .get_resource(*resource)
                    .expect("Character to have resource")
                    .current
                    >= *value
            }
            ActionRequirement::ButtonPressed(button) => situation.held_buttons.contains(button),
            ActionRequirement::ButtonNotPressed(button) => !situation.held_buttons.contains(button),
            ActionRequirement::StatusNotActive(status) => !situation.status_flags.contains(status),
            ActionRequirement::And(list) => {
                for inner in list {
                    if !inner.check(self_id, windows, situation) {
                        return false;
                    }
                }
                true
            }
            ActionRequirement::Or(list) => {
                for inner in list {
                    if inner.check(self_id, windows, situation) {
                        return true;
                    }
                }
                false
            }
            ActionRequirement::Starter(category) => {
                if situation.stunned {
                    // TODO: Reconsider this one for bursts
                    return false;
                }

                // Raw activation
                if situation.tracker.is_none() {
                    return true;
                }

                // Mega interrupts
                if *category == ActionCategory::MegaInterrupt && !situation.stunned {
                    return true;
                }

                for win in windows {
                    let matching_cancel = match win {
                        CancelType::Special => {
                            // Comic cancel uses the same cancel window
                            // as the special cancel
                            if *category == ActionCategory::Normal
                                && situation.inventory.contains(&ItemId::ComicBook)
                                && !situation
                                    .status_flags
                                    .contains(&StatusFlag::ComicCancelCooldown)
                            {
                                return true;
                            }

                            matches!(category, ActionCategory::Special | ActionCategory::Super)
                        }
                        CancelType::Super => *category == ActionCategory::Super,
                        CancelType::Specific(ref options) => options.contains(&self_id),
                        CancelType::Anything => true,
                    };

                    if matching_cancel {
                        return true;
                    }
                }
                false
            }
        }
    }
}
