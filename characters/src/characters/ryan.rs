use bevy::prelude::*;
use types::Area;

use crate::{
    AttackHeight, Branch, CancelLevel, Cost, GrabDescription, Hitbox, Item, ItemId, Lifetime, Move,
    MoveId, MoveMobility, Phase, PhaseKind, Requirements, SpawnDescriptor,
};

use super::{dash, get_equipment_move, jump, Character};

pub fn ryan() -> Character {
    Character::new(ryan_moves(), ryan_items())
}

// Dashing
const DASH_DURATION: usize = (0.5 * constants::FPS) as usize;
const DASH_IMPULSE: f32 = 10.0;

fn ryan_moves() -> Vec<(MoveId, Move)> {
    // Technically this is a slight performance loss, but
    // having all the components formated like this is more readable
    vec![]
        .into_iter()
        .chain(items().into_iter())
        .chain(movement().into_iter())
        .chain(attacks().into_iter())
        .collect()
}

fn items() -> Vec<(MoveId, Move)> {
    vec![MoveId::HandMeDownKen, MoveId::Gunshot, MoveId::Shoot]
        .into_iter()
        .map(|id| (id, get_equipment_move(id)))
        .collect()
}

fn movement() -> Vec<(MoveId, Move)> {
    vec![
        (
            MoveId::BackJump,
            jump(
                "7",
                (-constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y, 0.0),
            ),
        ),
        (
            MoveId::NeutralJump,
            jump("8", (0.0, constants::NEUTRAL_JUMP_Y, 0.0)),
        ),
        (
            MoveId::ForwardJump,
            jump(
                "9",
                (constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y, 0.0),
            ),
        ),
        (
            MoveId::BackSuperJump,
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
            MoveId::NeutralSuperJump,
            jump("[123]8", (0.0, constants::NEUTRAL_SUPERJUMP_Y, 0.0)),
        ),
        (
            MoveId::ForwardSuperJump,
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
            MoveId::DashForward,
            dash("656", DASH_DURATION, DASH_IMPULSE),
        ),
        (MoveId::DashBack, dash("454", DASH_DURATION, -DASH_IMPULSE)),
    ]
}

fn attacks() -> Vec<(MoveId, Move)> {
    vec![
        (
            MoveId::Punch,
            Move {
                input: Some("f"),
                requirements: Requirements {
                    grounded: Some(true),
                    cancel_level: Some(CancelLevel::LightNormal),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                            ..default()
                        }),
                        duration: 10,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::Low,
            Move {
                input: Some("[123]f"),
                requirements: Requirements {
                    grounded: Some(true),
                    cancel_level: Some(CancelLevel::LightNormal),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 0.2, 0.3, 0.2)),
                            ..default()
                        }),
                        duration: 10,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::CommandPunch,
            Move {
                input: Some("6f"),
                requirements: Requirements {
                    grounded: Some(true),
                    cancel_level: Some(CancelLevel::LightNormal),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        mobility: Some(MoveMobility::Perpetual(Vec3::new(1.0, 0.0, 0.0))),
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.5, 0.5, 0.5)),
                            ..default()
                        }),
                        duration: 20,
                        mobility: Some(MoveMobility::Perpetual(Vec3::new(2.0, 0.0, 0.0))),
                        ..default()
                    }
                    .into(),
                    Branch {
                        default: Phase {
                            kind: PhaseKind::Animation,
                            duration: 60,
                            ..default()
                        }
                        .into(),
                        branches: vec![(
                            Requirements::has_hit(),
                            Phase {
                                kind: PhaseKind::Animation,
                                duration: 1,
                                cancellable: true,
                                ..default()
                            }
                            .into(),
                        )],
                    },
                ],
            },
        ),
        (
            MoveId::BudgetBoom,
            Move {
                input: Some("[41]6f"),
                requirements: Requirements {
                    grounded: Some(true),
                    cancel_level: Some(CancelLevel::LightSpecial),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                            speed: 5.0 * Vec3::X,
                            lifetime: Lifetime::Frames((constants::FPS * 0.25) as usize),
                            attached_to_player: false,
                            ..default()
                        }),
                        duration: 4,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 5,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::SonicBoom,
            Move {
                input: Some("[41]6f"),
                requirements: Requirements {
                    cancel_level: Some(CancelLevel::HeavySpecial),
                    cost: Some(Cost {
                        charge: true,
                        ..default()
                    }),
                    grounded: Some(true),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.4, 0.3)),
                            speed: 6.0 * Vec3::X,
                            lifetime: Lifetime::Forever,
                            damage: (20, 3).into(),
                            attached_to_player: false,
                            ..default()
                        }),
                        duration: 4,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 5,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::Hadouken,
            Move {
                input: Some("236f"),
                requirements: Requirements {
                    grounded: Some(true),
                    cancel_level: Some(CancelLevel::LightSpecial),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 30,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                            speed: 4.0 * Vec3::X,
                            lifetime: Lifetime::Forever,
                            attached_to_player: false,
                            ..default()
                        }),
                        duration: 4,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 30,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::HeavyHadouken,
            Move {
                input: Some("236s"),
                requirements: Requirements {
                    cancel_level: Some(CancelLevel::HeavySpecial),
                    cost: Some(Cost {
                        meter: 30,
                        ..default()
                    }),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 30,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.4, 0.5)),
                            speed: 5.0 * Vec3::X,
                            lifetime: Lifetime::Forever,
                            hits: 2,
                            attached_to_player: false,
                            ..default()
                        }),
                        duration: 4,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 20,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::AirPunch,
            Move {
                input: Some("f"),
                requirements: Requirements {
                    cancel_level: Some(CancelLevel::LightNormal),
                    grounded: Some(false),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 5,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 0.1, 0.3, 0.5)),
                            fixed_height: Some(AttackHeight::High),
                            ..default()
                        }),
                        duration: 10,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 10,
                        cancellable: true,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
        (
            MoveId::Grab,
            Move {
                input: Some("g"),
                requirements: Requirements {
                    cancel_level: Some(CancelLevel::Grab),
                    grounded: Some(true),
                    ..default()
                },
                phases: vec![
                    Phase {
                        kind: PhaseKind::Animation,
                        duration: 5,
                        ..default()
                    }
                    .into(),
                    Phase {
                        kind: PhaseKind::Grab(GrabDescription {
                            damage: 25,
                            ..default()
                        }),
                        duration: 40,
                        ..default()
                    }
                    .into(),
                ],
            },
        ),
    ]
}

fn ryan_items() -> Vec<(ItemId, Item)> {
    vec![
        (
            ItemId::Drugs,
            Item {
                cost: 100,
                tier: 1,
                is_starter: false,
            },
        ),
        (
            ItemId::HandMeDownKen,
            Item {
                cost: 0,
                tier: 0,
                is_starter: true,
            },
        ),
        (
            ItemId::Gi,
            Item {
                cost: 100,
                tier: 2,
                is_starter: true,
            },
        ),
        (
            ItemId::Gun,
            Item {
                cost: 100,
                tier: 2,
                is_starter: true,
            },
        ),
    ]
}

#[cfg(test)]
mod test {
    use bevy::utils::HashSet;

    use super::*;

    #[test]
    fn no_duplicate_move_ids() {
        let mut ids: HashSet<MoveId> = vec![].into_iter().collect();

        for (id, _) in ryan_moves() {
            assert!(
                !ids.contains(&id),
                "ID {:?} was found twice in move list",
                id
            );
            ids.insert(id);
        }
    }
}
