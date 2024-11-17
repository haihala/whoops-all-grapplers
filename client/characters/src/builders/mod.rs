mod action_builder;
mod attack_builder;
mod throw_builders;

pub use action_builder::*;
pub use attack_builder::*;
pub use throw_builders::*;

use crate::{ActionEvent, Situation};
use std::sync::Arc;

pub type DynamicEvents = Arc<dyn Fn(&Situation) -> Vec<ActionEvent> + Send + Sync>;
