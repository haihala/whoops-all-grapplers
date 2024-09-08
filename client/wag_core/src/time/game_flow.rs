use crate::{Player, LOSS_BONUS};
use bevy::prelude::*;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum LocalState {
    ControllerAssignment,
    CharacterSelect,
    Loading,
    SetupMatch,
    Match(MatchState),
    EndScreen,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum OnlineState {
    CharacterSelect,
    Lobby,
    Loading,
    SetupMatch,
    Match(MatchState),
    EndScreen,
}

impl ComputedStates for OnlineState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Online(ol) => Some(ol),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum MatchState {
    PreRound,
    Combat,
    PostRound,
    Shop,
}

impl ComputedStates for MatchState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Local(LocalState::Match(cs)) | GameState::Online(OnlineState::Match(cs)) => {
                Some(cs)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,

    Local(LocalState),
    Online(OnlineState),
}

impl GameState {
    pub fn is_online(&self) -> bool {
        matches!(self, GameState::Online(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InMenu;
impl ComputedStates for InMenu {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::MainMenu
                | GameState::Local(LocalState::ControllerAssignment)
                | GameState::Local(LocalState::CharacterSelect)
                | GameState::Local(LocalState::EndScreen)
                | GameState::Online(OnlineState::CharacterSelect)
                | GameState::Online(OnlineState::EndScreen)
        ) {
            Some(InMenu)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InCharacterSelect;
impl ComputedStates for InCharacterSelect {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::CharacterSelect)
                | GameState::Online(OnlineState::CharacterSelect)
        ) {
            Some(InCharacterSelect)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InMatchSetup;

impl ComputedStates for InMatchSetup {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::SetupMatch) | GameState::Online(OnlineState::SetupMatch)
        ) {
            Some(InMatchSetup)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InEndScreen;

impl ComputedStates for InEndScreen {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::EndScreen) | GameState::Online(OnlineState::EndScreen)
        ) {
            Some(InEndScreen)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InCombat;

impl ComputedStates for InCombat {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::Match(MatchState::Combat))
                | GameState::Online(OnlineState::Match(MatchState::Combat))
        ) {
            Some(InCombat)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InLoadingScreen;

impl ComputedStates for InLoadingScreen {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::Loading) | GameState::Online(OnlineState::Loading)
        ) {
            Some(InLoadingScreen)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InMatch;

impl ComputedStates for InMatch {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if matches!(
            sources,
            GameState::Local(LocalState::Loading)
                | GameState::Local(LocalState::SetupMatch)
                | GameState::Local(LocalState::Match(_))
                | GameState::Online(OnlineState::Loading)
                | GameState::Online(OnlineState::SetupMatch)
                | GameState::Online(OnlineState::Match(_))
        ) {
            Some(InMatch)
        } else {
            None
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct RoundLog {
    log: Vec<RoundResult>,
}
impl RoundLog {
    pub fn clear(&mut self) {
        self.log.clear();
    }
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

    pub fn wins(&self, player: Player) -> usize {
        self.log
            .iter()
            .filter(|round| round.winner == Some(player))
            .count()
    }
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct GameResult {
    pub winner: Player,
}

#[derive(Debug, Clone, Copy)]
pub struct RoundResult {
    pub winner: Option<Player>,
}

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
