use crate::Situation;

pub fn grounded(situation: Situation) -> bool {
    situation.grounded
}

pub fn not_grounded(situation: Situation) -> bool {
    !situation.grounded
}
