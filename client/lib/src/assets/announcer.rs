use foundation::{Player, Sound, SoundRequest};

use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct Announcer {
    stack: Vec<Sound>,
}

impl Announcer {
    pub fn fight(&mut self) {
        self.stack = vec![Sound::AnnouncerFight];
    }

    pub fn round_start(&mut self, round_number: usize) {
        self.stack = vec![Sound::Number(round_number), Sound::AnnouncerRound];
    }

    pub fn round_win(&mut self, winner: Player) {
        self.stack = vec![
            Sound::AnnouncerWins,
            Sound::Number(winner.into()),
            Sound::AnnouncerPlayer,
        ];
    }

    pub fn tie(&mut self) {
        self.stack = vec![Sound::AnnouncerDraw];
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

    commands.trigger(SoundRequest::from(next));
}
