use std::time::Duration;

use bevy::prelude::*;
use types::Player;

use crate::test_helpers::{InputClump, TestRunner, TestSpec};

#[test]
#[ignore]
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

    let mut p1_height = None;
    let mut p2_height = None;
    for (player, tf) in wrapper
        .query::<(&Player, &Transform)>()
        .iter(wrapper.world())
    {
        match player {
            Player::One => p1_height = Some(tf.translation.y),
            Player::Two => p2_height = Some(tf.translation.y),
        }
    }
    assert!(p1_height.is_some() && p2_height.is_some());
    assert!(p1_height.unwrap() > p2_height.unwrap());
}
