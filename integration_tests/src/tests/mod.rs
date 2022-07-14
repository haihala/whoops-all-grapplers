use bevy::prelude::*;
use std::time::Duration;

use crate::test_helpers::{InputClump, TestRunner, TestSpec};

#[test]
fn round_start() {
    let mut test_runner = TestRunner::new();

    let mut wrapper = test_runner.run(
        "Jump",
        TestSpec::new(
            vec![
                InputClump::Idle(Duration::from_secs_f32(0.1)),
                InputClump::InputStream("8"),
                InputClump::Idle(Duration::from_secs_f32(0.1)),
            ],
            vec![],
        ),
    );

    let (p1, p2) = wrapper.get_players();

    let p1_height = wrapper
        .query::<&Transform>()
        .get(wrapper.world(), p1)
        .unwrap()
        .translation
        .y;
    let p2_height = wrapper
        .query::<&Transform>()
        .get(wrapper.world(), p2)
        .unwrap()
        .translation
        .y;

    assert!(p1_height > p2_height);
}
