use foundation::{Player, RoundLog, SoundEffect};

use bevy::prelude::*;

use crate::event_spreading::PlaySound;

#[derive(Debug, Resource, Default)]
pub struct Announcer {
    stack: Vec<SoundEffect>,
}

impl Announcer {
    pub fn fight(&mut self) {
        self.stack = vec![SoundEffect::AnnouncerFight];
    }

    pub fn round_start(&mut self, round_number: usize) {
        self.stack = vec![
            SoundEffect::Number(round_number),
            SoundEffect::AnnouncerRound,
        ];
    }

    pub fn round_win(&mut self, winner: Player) {
        self.stack = vec![
            SoundEffect::AnnouncerWins,
            SoundEffect::Number(winner.into()),
            SoundEffect::AnnouncerPlayer,
        ];
    }

    pub fn tie(&mut self) {
        self.stack = vec![SoundEffect::AnnouncerDraw];
    }
}

#[derive(Debug, Component)]
pub struct AnnouncerMarker;

pub fn update_announcer(
    mut commands: Commands,
    marked: Query<&AnnouncerMarker>,
    mut announcer: ResMut<Announcer>,
) {
    if !marked.is_empty() {
        return;
    }

    let Some(next) = announcer.stack.pop() else {
        return;
    };

    commands.trigger(PlaySound(next));
}

pub fn preround(mut announcer: ResMut<Announcer>, round_log: Res<RoundLog>) {
    announcer.round_start(round_log.rounds_played() + 1);
}

pub fn combat(mut announcer: ResMut<Announcer>) {
    announcer.fight();
}
