use foundation::{Player, Sound, SoundRequest};

use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct Announcer {
    stack: Vec<Sound>,
    clear_existing: bool,
}

impl Announcer {
    pub fn say(&mut self, stack: Vec<Sound>) {
        self.stack = stack;
        self.clear_existing = true;
    }

    pub fn fight(&mut self) {
        self.say(vec![Sound::AnnouncerFight]);
    }

    pub fn round_start(&mut self, round_number: usize) {
        self.say(vec![Sound::Number(round_number), Sound::AnnouncerRound]);
    }

    pub fn round_win(&mut self, winner: Player) {
        self.say(vec![
            Sound::AnnouncerWins,
            Sound::Number(winner.into()),
            Sound::AnnouncerPlayer,
        ]);
    }

    pub fn tie(&mut self) {
        self.say(vec![Sound::AnnouncerDraw]);
    }
}

#[derive(Debug, Component)]
pub struct AnnouncerMarker;

pub fn update_announcer(
    mut commands: Commands,
    marked: Query<Entity, With<AnnouncerMarker>>,
    mut announcer: ResMut<Announcer>,
) {
    if !marked.is_empty() && !announcer.clear_existing {
        return;
    }

    if announcer.clear_existing {
        for entity in &marked {
            commands.entity(entity).despawn_recursive();
        }
        announcer.clear_existing = false;
    }

    let Some(next) = announcer.stack.pop() else {
        return;
    };

    commands.trigger(SoundRequest::from(next));
}
