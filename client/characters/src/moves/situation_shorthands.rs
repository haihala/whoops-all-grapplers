use crate::Situation;

pub fn grounded(situation: Situation) -> bool {
    situation.grounded
}

pub fn airborne(situation: Situation) -> bool {
    !situation.grounded
}
