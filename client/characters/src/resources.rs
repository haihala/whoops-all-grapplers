use bevy::prelude::*;

use wag_core::{
    GameButton, Stats, StickPosition, HEALTH_BAR_COLOR, METER_BAR_FULL_SEGMENT_COLOR,
    METER_BAR_PARTIAL_SEGMENT_COLOR,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
/// This is a quick handle that can be referred to in requirement checks
pub enum ResourceType {
    Health,
    Meter,
    Charge,
    Sharpness,
    KunaiCounter,
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct WAGResources(pub Vec<(ResourceType, WAGResource)>);

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
                        max: Some(stats.max_health),
                        current: stats.max_health,
                        render_instructions: RenderInstructions::Bar(
                            ResourceBarVisual::default_health(),
                        ),
                        ..default()
                    },
                ),
                (
                    ResourceType::Meter,
                    WAGResource {
                        // TODO: Add more stats attributes here and in reset
                        max: Some(100),
                        render_instructions: RenderInstructions::Bar(
                            ResourceBarVisual::default_meter(),
                        ),
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
                    prop.max = Some(stats.max_health);
                    prop.current = stats.max_health;
                }
                ResourceType::Meter => {
                    prop.current = stats.starting_meter;
                }
                ResourceType::KunaiCounter => {
                    prop.current = stats.extra_kunais + 1;
                }
                ResourceType::Sharpness => {
                    if !stats.retain_sharpness {
                        prop.current = prop.min;
                    }
                }
                _ => {
                    prop.current = prop.min;
                }
            }
        }
    }

    pub fn testing_default() -> Self {
        Self::from_stats(&Stats::default(), vec![])
    }

    pub fn get(&self, resource_type: ResourceType) -> Option<&WAGResource> {
        self.iter()
            .find_map(|(t, r)| if *t == resource_type { Some(r) } else { None })
    }

    pub fn get_mut(&mut self, resource_type: ResourceType) -> Option<&mut WAGResource> {
        self.iter_mut()
            .find_map(|(t, r)| if *t == resource_type { Some(r) } else { None })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderInstructions {
    Bar(ResourceBarVisual),
    Counter(CounterVisual),
    None,
}

impl Default for RenderInstructions {
    fn default() -> Self {
        Self::Bar(ResourceBarVisual::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct WAGResource {
    pub max: Option<i32>,
    pub min: i32,
    pub current: i32,
    pub render_instructions: RenderInstructions,
    pub special: Option<SpecialProperty>,
}
impl WAGResource {
    pub fn is_full(&self) -> bool {
        self.current == self.max.unwrap_or(i32::MAX)
    }

    pub fn is_empty(&self) -> bool {
        self.current == self.min
    }

    pub fn get_percentage(&self) -> f32 {
        100.0 * ((self.current - self.min) as f32)
            / ((self.max.unwrap_or(i32::MAX) - self.min) as f32)
    }

    pub fn change(&mut self, amount: i32) {
        self.current = (self.current + amount).clamp(self.min, self.max.unwrap_or(i32::MAX));
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
    pub segment_gap: f32,
}

impl Default for ResourceBarVisual {
    fn default() -> Self {
        Self {
            height: 4.0,
            segments: 1,
            segment_gap: 1.0,
            default_color: Default::default(),
            full_color: Default::default(),
        }
    }
}
impl ResourceBarVisual {
    pub fn default_health() -> Self {
        Self {
            height: 50.0,
            default_color: HEALTH_BAR_COLOR,
            ..default()
        }
    }

    pub fn default_meter() -> Self {
        Self {
            default_color: METER_BAR_PARTIAL_SEGMENT_COLOR,
            full_color: Some(METER_BAR_FULL_SEGMENT_COLOR), // TODO: Move colors to a theme, they don't need to be a resource
            segments: 5,
            ..default()
        }
    }

    pub fn segment_width(&self) -> f32 {
        (100.0 - (self.segments - 1) as f32 * self.segment_gap) / self.segments as f32
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct CounterVisual {
    pub label: &'static str,
}

#[derive(Debug, Clone)]
/// This is for adding properties that cannot be included in the ResourceType
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
