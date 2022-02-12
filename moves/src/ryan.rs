use bevy::prelude::*;
use types::{AttackHeight, GrabDescription, Hitbox, Lifetime, SpawnDescriptor};

use crate::{
    move_bank::MoveBank, moves, universal, CancelLevel, Move, MoveCondition, MoveMobility, Phase,
    PhaseCondition, PhaseKind, PhaseSwitch,
};

// Dashing
const DASH_START_DURATION_SECONDS: f32 = 0.1;
const DASH_RECOVERY_DURATION_SECONDS: f32 = 0.2;
const DASH_DISTANCE: f32 = 3.0;
const DASH_START_DISTANCE_FRACTION: f32 = 0.5;

const SHIFT_DURING_DASH_START: f32 = DASH_DISTANCE * DASH_START_DISTANCE_FRACTION;
const SHIFT_DURING_DASH_RECOVERY: f32 = DASH_DISTANCE * (1.0 - DASH_START_DISTANCE_FRACTION);

const DASH_START_SPEED: f32 = SHIFT_DURING_DASH_START / DASH_START_DURATION_SECONDS;
const DASH_RECOVERY_SPEED: f32 = SHIFT_DURING_DASH_RECOVERY / DASH_RECOVERY_DURATION_SECONDS;
const DASH_START_FRAMES: usize = (DASH_START_DURATION_SECONDS * constants::FPS) as usize;
const DASH_RECOVERY_FRAMES: usize = (DASH_RECOVERY_DURATION_SECONDS * constants::FPS) as usize;

moves!(
    2usize,
    (
        GRAB,
        HEAVY_HADOUKEN,
        HADOUKEN,
        AIR_PUNCH,
        COMMAND_PUNCH,
        PUNCH
    )
);

fn jump(input: &'static str, impulse: impl Into<Vec3>) -> Move {
    Move {
        input,
        cancel_level: CancelLevel::Jump,
        conditions: MoveCondition::GROUND,
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                mobility: Some(MoveMobility::Impulse(impulse.into())),
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                ..Default::default()
            }
            .into(),
        ],
        ..Default::default()
    }
}

fn dash(input: &'static str, start_speed: f32, recovery_speed: f32) -> Move {
    Move {
        input,
        cancel_level: CancelLevel::Dash,
        conditions: MoveCondition::GROUND,
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: DASH_START_FRAMES,
                mobility: Some(MoveMobility::Perpetual(Vec3::X * start_speed)),
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: DASH_RECOVERY_FRAMES,
                cancellable: true,
                mobility: Some(MoveMobility::Perpetual(Vec3::X * recovery_speed)),
            }
            .into(),
        ],
        ..Default::default()
    }
}

fn ryan_moves() -> Vec<(MoveId, Move)> {
    vec![
        (
            universal::BACK_JUMP,
            jump(
                "7",
                (-constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y, 0.0),
            ),
        ),
        (
            universal::NEUTRAL_JUMP,
            jump("8", (0.0, constants::NEUTRAL_JUMP_Y, 0.0)),
        ),
        (
            universal::FORWARD_JUMP,
            jump(
                "9",
                (constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y, 0.0),
            ),
        ),
        (
            universal::BACK_SUPER_JUMP,
            jump(
                "[123]7",
                (
                    -constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                    0.0,
                ),
            ),
        ),
        (
            universal::NEUTRAL_SUPER_JUMP,
            jump("[123]8", (0.0, constants::NEUTRAL_SUPERJUMP_Y, 0.0)),
        ),
        (
            universal::FORWARD_SUPER_JUMP,
            jump(
                "[123]9",
                (
                    constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                    0.0,
                ),
            ),
        ),
        (
            universal::DASH_FORWARD,
            dash("656", DASH_START_SPEED, DASH_RECOVERY_SPEED),
        ),
        (
            universal::DASH_BACK,
            dash("454", -DASH_START_SPEED, -DASH_RECOVERY_SPEED),
        ),
        (
            PUNCH,
            Move {
                input: "f",
                cancel_level: CancelLevel::LightNormal,
                conditions: MoveCondition::GROUND,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox::new(Vec2::new(1.0, 0.5), Vec2::new(0.2, 0.3)),
                            attached_to_player: true,
                            damage: Some(20.into()),
                            ..Default::default()
                        }),
                        duration: 10,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            },
        ),
        (
            COMMAND_PUNCH,
            Move {
                input: "6f",
                cancel_level: CancelLevel::LightNormal,
                conditions: MoveCondition::GROUND,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        mobility: Some(MoveMobility::Perpetual(Vec3::new(1.0, 0.0, 0.0))),
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(1.0, 1.0)),
                            attached_to_player: true,
                            ..Default::default()
                        }),
                        duration: 20,
                        mobility: Some(MoveMobility::Perpetual(Vec3::new(5.0, 0.0, 0.0))),
                        ..Default::default()
                    }
                    .into(),
                    PhaseSwitch {
                        default: Phase {
                            kind: PhaseKind::Animation,
                            duration: 60,
                            ..Default::default()
                        },
                        branches: vec![(
                            PhaseCondition::HIT,
                            Phase {
                                kind: PhaseKind::Animation,
                                duration: 10,
                                cancellable: true,
                                ..Default::default()
                            },
                        )],
                    },
                ],
                ..Default::default()
            },
        ),
        (
            HADOUKEN,
            Move {
                input: "236f",
                cancel_level: CancelLevel::LightSpecial,
                conditions: MoveCondition::GROUND,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 30,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                            speed: Some(1.0 * Vec3::X),
                            lifetime: Lifetime::Forever,
                            ..Default::default()
                        }),
                        duration: 4,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            },
        ),
        (
            HEAVY_HADOUKEN,
            Move {
                input: "236s",
                cancel_level: CancelLevel::HeavySpecial,
                meter_cost: 10,
                conditions: MoveCondition::GROUND | MoveCondition::AIR,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 30,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.7, 0.7)),
                            speed: Some(2.0 * Vec3::X),
                            lifetime: Lifetime::Forever,
                            ..Default::default()
                        }),
                        duration: 4,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            },
        ),
        (
            AIR_PUNCH,
            Move {
                input: "f",
                cancel_level: CancelLevel::LightNormal,
                conditions: MoveCondition::AIR,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox::new(Vec2::new(0.5, -1.2), Vec2::new(0.6, 0.3)),
                            knockback: Some(Vec3::new(2.0, 2.0, 0.0).into()),
                            fixed_height: Some(AttackHeight::High),
                            attached_to_player: true,
                            ..Default::default()
                        }),
                        duration: 10,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            },
        ),
        (
            GRAB,
            Move {
                input: "g",
                cancel_level: CancelLevel::Grab,
                conditions: MoveCondition::GROUND,
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 1,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Grab(GrabDescription {
                            damage: 40,
                            impulse: Vec3::Y * 2.0,
                            range: 1.0,
                            ..Default::default()
                        }),
                        duration: 1,
                        ..Default::default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            },
        ),
    ]
}

pub fn ryan_bank() -> MoveBank {
    MoveBank::new(ryan_moves().into_iter().collect())
}

#[cfg(test)]
mod test {
    use bevy::utils::{HashMap, HashSet};

    use super::*;

    #[test]
    fn no_duplicate_move_ids() {
        let mut ids: HashSet<MoveId> = vec![].into_iter().collect();

        for (id, _) in ryan_moves() {
            assert!(!ids.contains(&id), "ID {} was found twice in move list", id);
            ids.insert(id);
        }
    }

    #[test]
    fn no_duplicate_inputs() {
        let mut inputs: HashMap<&'static str, (MoveId, MoveCondition)> =
            vec![].into_iter().collect();

        for (id, move_data) in ryan_moves() {
            let input = move_data.input;

            if inputs.contains_key(&input) {
                let (other_id, conditions) = inputs.get(&input).unwrap();
                if conditions.intersects(move_data.conditions) {
                    panic!(
                        "Input {} has two overlapping uses on ids {} and {}",
                        input, id, other_id
                    );
                }
            }

            inputs.insert(input, (id, move_data.conditions));
        }
    }
}
