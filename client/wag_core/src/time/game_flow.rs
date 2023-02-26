use crate::{Player, LOSS_BONUS};
use bevy::prelude::*;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum GameState {
    Loading,
    Combat,
    PostRound,
    Shop,
}
impl GameState {
    pub fn next(self) -> GameState {
        match self {
            GameState::Loading => GameState::Combat,

            GameState::Combat => GameState::PostRound,
            GameState::PostRound => GameState::Shop,
            GameState::Shop => GameState::Combat,
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct RoundLog {
    log: Vec<RoundResult>,
}
impl RoundLog {
    pub fn add(&mut self, result: RoundResult) {
        self.log.push(result);
    }
    pub fn last(&self) -> Option<RoundResult> {
        self.log.last().cloned()
    }
    pub fn loss_bonus(&self, player: Player) -> usize {
        let mut streak = 0;
        for round in self.log.iter().rev() {
            // Tie resets loss bonus for both players
            if round.winner == Some(player.other()) {
                streak += 1;
            } else {
                break;
            }
        }
        // This makes it so the last loss doesn't add to the streak.
        LOSS_BONUS * if streak > 0 { streak - 1 } else { 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoundResult {
    pub winner: Option<Player>,
}

#[derive(Debug, Component, Deref)]
pub struct OnlyShowInGameState(pub Vec<GameState>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loss_bonus_increments_correctly() {
        let mut log = RoundLog::default();

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), 0);

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), 0);

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), LOSS_BONUS);

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), 2 * LOSS_BONUS);
    }

    #[test]
    fn loss_bonus_resets_and_builds_back() {
        let mut log = RoundLog::default();

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), LOSS_BONUS);

        log.add(RoundResult {
            winner: Some(Player::Two),
        });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), 0);

        log.add(RoundResult {
            winner: Some(Player::Two),
        });

        assert_eq!(log.loss_bonus(Player::One), LOSS_BONUS);
        assert_eq!(log.loss_bonus(Player::Two), 0);
    }

    #[test]
    fn loss_bonus_resets_on_tie() {
        let mut log = RoundLog::default();

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        log.add(RoundResult {
            winner: Some(Player::One),
        });

        log.add(RoundResult { winner: None });

        assert_eq!(log.loss_bonus(Player::One), 0);
        assert_eq!(log.loss_bonus(Player::Two), 0);
    }
}
