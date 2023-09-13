use bevy::prelude::*;

const FRAMES_BETWEEN_HITS: usize = 10;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect, Component)]
pub struct HitTracker {
    pub hits: usize,
    pub last_hit_frame: Option<usize>,
    pub hit_intangible: bool,
}
impl HitTracker {
    pub fn new(hits: usize) -> Self {
        Self { hits, ..default() }
    }
    pub fn active(&self, current_frame: usize) -> bool {
        self.last_hit_frame
            .map(|frame| frame + FRAMES_BETWEEN_HITS <= current_frame)
            .unwrap_or(true)
    }
    pub fn register_hit(&mut self, current_frame: usize) {
        self.hits -= 1;
        self.last_hit_frame = Some(current_frame);
    }
}
impl Default for HitTracker {
    fn default() -> Self {
        Self {
            hits: 1,
            last_hit_frame: None,
            hit_intangible: false,
        }
    }
}
