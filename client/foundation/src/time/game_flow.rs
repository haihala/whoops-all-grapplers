use crate::Player;
use bevy::prelude::*;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum LocalState {
    ControllerAssignment,
    CharacterSelect,
    Match,
}
impl ComputedStates for LocalState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Local(ls) => Some(ls),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum OnlineState {
    CharacterSelect,
    Lobby,
    Match,
}
impl ComputedStates for OnlineState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Online(os) => Some(os),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, States, Default)]
pub enum MatchState {
    #[default]
    None,
    Loading,
    PreRound, // TODO: Rename to countdown
    Combat,
    PostRound,
    Shop,
    EndScreen,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Credits,

    Local(LocalState),
    Online(OnlineState),
    Synctest,
}

impl GameState {
    pub fn is_online(&self) -> bool {
        matches!(self, GameState::Online(_) | GameState::Synctest)
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
pub struct InMatch;

impl ComputedStates for InMatch {
    type SourceStates = MatchState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if !matches!(sources, MatchState::None) {
            Some(InMatch)
        } else {
            None
        }
    }
}

#[derive(Debug, Resource, Default, Clone)]
pub struct RoundLog {
    log: Vec<RoundResult>,
}
impl RoundLog {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub fn add(&mut self, result: RoundResult) {
        self.log.push(result);
    }
    pub fn last(&self) -> Option<RoundResult> {
        self.log.last().cloned()
    }

    pub fn wins(&self, player: Player) -> usize {
        self.log
            .iter()
            .filter(|round| round.winner == Some(player))
            .count()
    }

    pub fn rounds_played(&self) -> usize {
        self.log.len()
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
