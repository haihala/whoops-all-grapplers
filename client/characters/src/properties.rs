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
                render_instructions: BarRenderInstructions::default_health(),
                ..default()
            },
            meter: Property {
                // TODO: Add more stats attributes here
                max: 100,
                render_instructions: BarRenderInstructions::default_meter(),
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
    pub render_instructions: BarRenderInstructions,
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

#[derive(Debug, Clone, Component)]
pub struct BarRenderInstructions {
    pub height: f32,
    pub default_color: Color,
    pub full_color: Option<Color>,
    pub segments: Option<i32>, // TODO: This does nothing for now
}

impl Default for BarRenderInstructions {
    fn default() -> Self {
        Self {
            height: 4.0,
            default_color: Default::default(),
            full_color: Default::default(),
            segments: Default::default(),
        }
    }
}
impl BarRenderInstructions {
    pub fn default_health() -> Self {
        Self {
            height: 50.0,
            default_color: Color::rgb(0.9, 0.0, 0.0),
            ..default()
        }
    }

    pub fn default_meter() -> Self {
        Self {
            default_color: Color::rgb(0.04, 0.5, 0.55),
            full_color: Some(Color::rgb(0.14, 0.7, 0.8)),
            segments: Some(25),
            ..default()
        }
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
