use bevy::{prelude::*, utils::HashMap};

use wag_core::{GameButton, Stats, StickPosition};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum ResourceType {
    Health,
    Meter,
    Charge,
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct WAGResources(pub HashMap<ResourceType, WAGResource>);

impl WAGResources {
    pub fn from_stats(
        stats: &Stats,
        additional_properties: Vec<(ResourceType, WAGResource)>,
    ) -> Self {
        WAGResources(
            vec![
                (
                    ResourceType::Health,
                    WAGResource {
                        max: stats.max_health,
                        current: stats.max_health,
                        render_instructions: ResourceBarVisual::default_health(),
                        ..default()
                    },
                ),
                (
                    ResourceType::Meter,
                    WAGResource {
                        // TODO: Add more stats attributes here and in reset
                        max: 100,
                        render_instructions: ResourceBarVisual::default_meter(),
                        ..default()
                    },
                ),
            ]
            .into_iter()
            .chain(additional_properties)
            .collect(),
        )
    }

    pub fn reset(&mut self, stats: &Stats) {
        for (prop_type, prop) in self.iter_mut() {
            match prop_type {
                ResourceType::Health => {
                    prop.max = stats.max_health;
                    prop.current = stats.max_health;
                }
                _ => {
                    prop.current = prop.min;
                }
            }
        }
    }

    pub fn testing_default() -> Self {
        Self::from_stats(&Stats::testing_default(), vec![])
    }
}

#[derive(Debug, Default, Clone)]
pub struct WAGResource {
    pub max: i32,
    pub min: i32,
    pub current: i32,
    pub render_instructions: ResourceBarVisual,
    pub special: Option<SpecialProperty>,
}
impl WAGResource {
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
        assert!(amount >= 0);
        self.change(amount)
    }

    pub fn drain(&mut self, amount: i32) {
        assert!(amount >= 0);
        self.change(-amount)
    }

    pub fn clear(&mut self) {
        self.current = self.min;
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct ResourceBarVisual {
    pub height: f32,
    pub default_color: Color,
    pub full_color: Option<Color>,
    pub segments: usize,
    // TODO: Padding between segments
}

impl Default for ResourceBarVisual {
    fn default() -> Self {
        Self {
            height: 4.0,
            segments: 1,
            default_color: Default::default(),
            full_color: Default::default(),
        }
    }
}
impl ResourceBarVisual {
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
            segments: 5,
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
    pub charging: bool,
}
impl Default for ChargeProperty {
    fn default() -> Self {
        Self {
            directions: vec![StickPosition::SW, StickPosition::S, StickPosition::W],
            buttons: vec![],
            clear_time: 20,
            last_gain_frame: 0,
            charging: false,
        }
    }
}