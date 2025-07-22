use foundation::{ActionCategory, ActionId, CancelType, GameButton, ItemId, StatusFlag};

use crate::{GaugeType, Situation};

#[derive(Debug, Clone, Hash, Default)]
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
    StatusMatches(fn(sf: &StatusFlag) -> bool),
    NoStatusMatches(fn(sf: &StatusFlag) -> bool),
    Starter(ActionCategory),
    And(Vec<ActionRequirement>),
    Or(Vec<ActionRequirement>),
}
impl ActionRequirement {
    pub fn check(
        &self,
        self_id: ActionId,
        situation: &Situation,
        input_registration_time: bool,
    ) -> bool {
        match (input_registration_time, self) {
            (false, ActionRequirement::None) => true,
            (false, ActionRequirement::Grounded) => situation.grounded,
            (false, ActionRequirement::Airborne) => !situation.grounded,
            (false, ActionRequirement::ActionOngoing(ids)) => {
                let Some(tracker) = &situation.tracker else {
                    return false;
                };

                ids.contains(&tracker.action_id)
            }
            (false, ActionRequirement::ActionNotOngoing(ids)) => {
                let Some(tracker) = &situation.tracker else {
                    return true;
                };

                !ids.contains(&tracker.action_id)
            }
            (false, ActionRequirement::AnyActionOngoing) => situation.tracker.is_some(),
            (false, ActionRequirement::ItemOwned(item_id)) => {
                situation.inventory.contains(*item_id)
            }
            (_, ActionRequirement::ResourceFull(resource)) => {
                // Only consider charge if at input registration time, don't consider charge if not
                let consider_value = (input_registration_time && *resource == GaugeType::Charge)
                    || (!input_registration_time && *resource != GaugeType::Charge);

                if consider_value {
                    situation
                        .get_resource(*resource)
                        .expect("Character to have resource")
                        .is_full()
                } else {
                    true
                }
            }
            (_, ActionRequirement::ResourceValue(resource, value)) => {
                // Only consider charge if at input registration time, don't consider charge if not
                let consider_value = (input_registration_time && *resource == GaugeType::Charge)
                    || (!input_registration_time && *resource != GaugeType::Charge);

                if consider_value {
                    situation
                        .get_resource(*resource)
                        .expect("Character to have resource")
                        .current
                        >= *value
                } else {
                    true
                }
            }
            (false, ActionRequirement::ButtonPressed(button)) => {
                situation.held_buttons.contains(button)
            }
            (false, ActionRequirement::ButtonNotPressed(button)) => {
                !situation.held_buttons.contains(button)
            }
            (false, ActionRequirement::StatusNotActive(status)) => {
                !situation.status_flags.contains(status)
            }
            (_, ActionRequirement::And(list)) => {
                for inner in list {
                    if !inner.check(self_id, situation, input_registration_time) {
                        return false;
                    }
                }
                true
            }
            (_, ActionRequirement::Or(list)) => {
                for inner in list {
                    if inner.check(self_id, situation, input_registration_time) {
                        return true;
                    }
                }
                false
            }
            (false, ActionRequirement::Starter(category)) => {
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

                for win in &situation.cancel_windows {
                    let matching_cancel = match win {
                        CancelType::Special => {
                            // Comic cancel uses the same cancel window
                            // as the special cancel
                            if *category == ActionCategory::Normal
                                && situation.inventory.contains(ItemId::ComicBook)
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
            (false, ActionRequirement::StatusMatches(checker)) => {
                situation.status_flags.iter().any(checker)
            }
            (false, ActionRequirement::NoStatusMatches(checker)) => {
                !situation.status_flags.iter().any(checker)
            }
            (true, _) => {
                // This is to ignore most requirements at input time
                true
            }
        }
    }
}
