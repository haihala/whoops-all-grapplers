use bevy::prelude::*;
use uuid::Uuid;

use super::movement::{DASH_BACK, DASH_FORWARD};
use super::PlayerState;
use crate::{
    animation::{Animation, AnimationBank},
    damage::{Hitbox, HitboxManager},
    Clock,
};
use input_parsing::{GameButton, InputReader, Normal, Special};

pub struct Ryan;

const PUNCH: Uuid = Uuid::from_u128(0x10);
const HADOUKEN: Uuid = Uuid::from_u128(0x11);

pub fn inputs() -> InputReader {
    let mut reader = InputReader::default();

    reader.register_special(
        HADOUKEN,
        Special {
            motion: vec![2, 3, 6].into(),
            button: Some(GameButton::Fast),
        },
    );

    reader.register_normal(
        PUNCH,
        Normal {
            button: GameButton::Fast,
            stick: None,
        },
    );

    reader.register_special(
        DASH_FORWARD,
        Special {
            motion: vec![6, 5, 6].into(),
            button: None,
        },
    );

    reader.register_special(
        DASH_BACK,
        Special {
            motion: vec![4, 5, 4].into(),
            button: None,
        },
    );
    reader
}

pub fn animations() -> AnimationBank {
    let mut bank = AnimationBank::default();

    bank.register(HADOUKEN, Animation::new(30, 10, 20));
    bank.register(PUNCH, Animation::new(10, 10, 10));

    bank
}

pub fn hitboxes() -> HitboxManager {
    let mut generator = HitboxManager::default();

    generator.register(
        HADOUKEN,
        Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2), Some(0.3)),
    );

    generator.register(
        PUNCH,
        Hitbox::new(Vec2::new(1.0, 0.5), Vec2::new(0.2, 0.3), Some(0.2)),
    );

    generator
}

pub fn move_starter(
    clock: Res<Clock>,
    mut query: Query<(&mut InputReader, &mut PlayerState, &mut AnimationBank), With<Ryan>>,
) {
    for (mut reader, mut state, mut animation) in query.iter_mut() {
        if *state == PlayerState::Standing {
            let events = reader.get_events();
            if events.is_empty() {
                continue;
            }

            let to_start = highest_priority_move(events);
            if to_start != DASH_FORWARD && to_start != DASH_BACK {
                *state = PlayerState::Startup;
                reader.consume_event(&to_start);
                animation.start(to_start, clock.frame);
            }
        }
    }
}

fn highest_priority_move(options: Vec<Uuid>) -> Uuid {
    if options.contains(&HADOUKEN) {
        HADOUKEN
    } else if options.contains(&PUNCH) {
        PUNCH
    } else if options.len() == 1 {
        options[0]
    } else {
        todo!()
    }
}
