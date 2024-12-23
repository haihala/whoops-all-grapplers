mod animation_helper;
mod animations_prep;

pub use animation_helper::{AnimationHelper, AnimationHelperSetup};
pub use animations_prep::Animations;

pub(super) use animation_helper::{setup_helpers, update_animation};
pub(super) use animations_prep::animation_paths;
