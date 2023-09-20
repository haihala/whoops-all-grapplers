use std::{collections::HashMap, f32::consts::PI};

use bevy::prelude::*;
use wag_core::{ActionId, Animation, AnimationType, Area, ItemId, Model, Stats};

use crate::{resources::ResourceType, Action, Item, WAGResource};

use super::jump;

#[derive(Debug, Component, Clone)]
pub struct Character {
    moves: HashMap<ActionId, Action>,
    pub items: HashMap<ItemId, Item>,
    pub model: Model,
    pub low_block_height: f32,
    pub high_block_height: f32,
    pub standing_pushbox: Area,
    pub crouching_pushbox: Area,
    pub generic_animations: HashMap<AnimationType, Animation>,
    pub gravity: f32,
    pub base_stats: Stats,
    pub special_properties: Vec<(ResourceType, WAGResource)>,
}
impl Character {
    // TODO: Consider making a builder for this
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        model: Model,
        generic_animations: HashMap<AnimationType, Animation>,
        mut moves: HashMap<ActionId, Action>,
        items: HashMap<ItemId, Item>,
        jump_height: f32,
        jump_duration: f32,
        base_stats: Stats,
        special_properties: Vec<(ResourceType, WAGResource)>,
    ) -> Character {
        let (jumps, gravity) = Self::jumps(jump_height, jump_duration);
        moves.extend(jumps);

        Self {
            model,
            generic_animations,
            moves,
            items,
            low_block_height: 0.5,
            high_block_height: 1.2,
            standing_pushbox: Area::from_center_size(Vec2::Y * 0.7, Vec2::new(0.4, 1.4)),
            crouching_pushbox: Area::from_center_size(Vec2::Y * 0.5, Vec2::new(0.4, 1.0)),
            gravity,
            special_properties,
            base_stats,
        }
    }

    pub fn get_move(&self, id: ActionId) -> Option<Action> {
        self.moves.get(&id).map(|opt| opt.to_owned())
    }

    pub fn get_pushbox(&self, crouching: bool) -> Area {
        if crouching {
            self.crouching_pushbox
        } else {
            self.standing_pushbox
        }
    }

    pub fn get_inputs(&self) -> HashMap<ActionId, &'static str> {
        self.moves
            .iter()
            .filter_map(|(key, move_data)| move_data.input.map(|input| (*key, input)))
            .collect()
    }

    fn jumps(height: f32, duration: f32) -> (impl Iterator<Item = (ActionId, Action)>, f32) {
        /*
        // Math for gravity
        x = x0 + v0*t + 1/2*a*t^2

        From the apex down
        x0 = jump height,
        x = 0
        v0 = 0

        0 = -h + 1/2*a*t^2
        1/2*a*t^2 = h
        a = 2*h/t^2
        */
        let gravity_force: f32 = 2.0 * height / (duration / 2.0).powf(2.0);
        let gravity_per_frame: f32 = gravity_force / wag_core::FPS;

        /*
        Math for initial jump velocity
        x = x0 + v0*t + 1/2*a*t^2
        From start to end

        x0 = 0
        x = 0
        t and a = known, solve v0

        0 = v0*t + 1/2*a*t^2
        v0 = -1/2*a*t
        */
        let neutral_jump_y: f32 = 0.5 * gravity_force * duration;

        const DIAGONAL_JUMP_ANGLE: f32 = 60.0 * PI / 180.0;
        let diagonal_jump_x: f32 = neutral_jump_y * DIAGONAL_JUMP_ANGLE.cos();
        let diagonal_jump_y: f32 = neutral_jump_y * DIAGONAL_JUMP_ANGLE.sin();

        const SUPERJUMP_HEIGHT_MULTIPLIER: f32 = 1.3;
        let neutral_superjump_y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * neutral_jump_y;
        let diagonal_superjump_x: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * diagonal_jump_x;
        let diagonal_superjump_y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * diagonal_jump_y;

        let jumps = vec![
            (
                ActionId::BackJump,
                jump("7", Vec2::new(-diagonal_jump_x, diagonal_jump_y)),
            ),
            (ActionId::NeutralJump, jump("8", Vec2::Y * neutral_jump_y)),
            (
                ActionId::ForwardJump,
                jump("9", Vec2::new(diagonal_jump_x, diagonal_jump_y)),
            ),
            (
                ActionId::BackSuperJump,
                jump(
                    "[123]7",
                    Vec2::new(-diagonal_superjump_x, diagonal_superjump_y),
                ),
            ),
            (
                ActionId::NeutralSuperJump,
                jump("[123]8", Vec2::Y * neutral_superjump_y),
            ),
            (
                ActionId::ForwardSuperJump,
                jump(
                    "[123]9",
                    Vec2::new(diagonal_superjump_x, diagonal_superjump_y),
                ),
            ),
        ]
        .into_iter();

        (jumps, gravity_per_frame)
    }
}
