use crate::Situation;

pub fn grounded(situation: Situation) -> bool {
    situation.grounded
}

pub fn airborne(situation: Situation) -> bool {
    !situation.grounded
}

fn holding_down(situation: Situation) -> bool {
    let down_inputs = 1..3;
    down_inputs.contains(&situation.parser.get_absolute_stick_position().into())
}

pub fn standing(situation: Situation) -> bool {
    situation.grounded && !holding_down(situation)
}

pub fn crouching(situation: Situation) -> bool {
    situation.grounded && holding_down(situation)
}
