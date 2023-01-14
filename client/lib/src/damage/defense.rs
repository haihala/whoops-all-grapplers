use bevy::prelude::*;
use wag_core::Clock;

const MIN_STREAK_FOR_REWARD: i32 = 3;
const REWARD_FLOOR: i32 = 5;
const REWARD_RAMP: i32 = 3;
const TIME_UNTIL_RESET: usize = (wag_core::FPS * 1.0) as usize;

#[derive(Debug, Default, Component)]
pub struct Defense {
    streak: i32,
    streak_last_event: Option<usize>,
}
impl Defense {
    pub fn get_reward(&self) -> i32 {
        if self.streak > MIN_STREAK_FOR_REWARD {
            REWARD_FLOOR + REWARD_RAMP * (self.streak - MIN_STREAK_FOR_REWARD - 1)
        } else {
            0
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn bump_streak(&mut self, current_frame: usize) {
        self.streak += 1;
        self.streak_last_event = Some(current_frame);
    }
}

pub fn timeout_defense_streak(mut query: Query<&mut Defense>, clock: Res<Clock>) {
    for mut defense in &mut query {
        if let Some(last_event) = defense.streak_last_event {
            if last_event + TIME_UNTIL_RESET < clock.frame {
                defense.reset();
            }
        }
    }
}
