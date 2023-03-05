use bevy::prelude::*;

use wag_core::{GameButton, Stats, StickPosition};

#[derive(Debug, Component)]
pub struct Properties {
    pub health: Property,
    pub meter: Property,
    pub special_properties: Vec<Property>,
}
impl Properties {
    pub fn from_stats(stats: &Stats) -> Self {
        Self {
            health: Property {
                max: stats.max_health,
                current: stats.max_health,
                ..default()
            },
            meter: Property {
                // TODO: Add more stats attributes here
                max: 100,
                ..default()
            },
            special_properties: vec![],
        }
    }

    pub fn with_specials(mut self, specials: Vec<Property>) -> Self {
        self.special_properties = specials;
        self
    }

    pub fn testing_default() -> Self {
        Self::from_stats(&Stats::testing_default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Property {
    pub max: i32,
    pub min: i32,
    pub current: i32,
    pub special: Option<SpecialProperty>,
}
impl Property {
    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn is_empty(&self) -> bool {
        self.current == self.min
    }

    pub fn get_percentage(&self) -> f32 {
        100.0 * ((self.current - self.min) as f32) / ((self.max - self.min) as f32)
    }

    pub fn change(&mut self, amount: i32) {
        self.current = (self.current + amount).clamp(self.min, self.max);
    }

    pub fn gain(&mut self, amount: i32) {
        assert!(amount > 0);
        self.change(amount)
    }

    pub fn drain(&mut self, amount: i32) {
        assert!(amount > 0);
        self.change(-amount)
    }

    pub fn clear(&mut self) {
        self.current = self.min;
    }
}

#[derive(Debug, Clone)]
pub enum SpecialProperty {
    Charge(ChargeProperty),
}

#[derive(Debug, Clone)]
pub struct ChargeProperty {
    pub directions: Vec<StickPosition>,
    pub buttons: Vec<GameButton>,

    pub clear_time: usize,
    pub last_gain_frame: usize,
}
impl Default for ChargeProperty {
    fn default() -> Self {
        Self {
            directions: vec![StickPosition::SW, StickPosition::S, StickPosition::W],
            buttons: vec![],
            clear_time: 20,
            last_gain_frame: 0,
        }
    }
}
