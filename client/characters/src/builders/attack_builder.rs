use std::{f32::consts::PI, sync::Arc};

use bevy::prelude::*;
use wag_core::{
    ActionId, Animation, Area, CancelType, CancelWindow, GameButton, Model, SoundEffect,
    VfxRequest, VisualEffect, VoiceLine, BIG_HIT_THRESHOLD, HIGH_OPENER_COLOR, LOW_OPENER_COLOR,
    MID_OPENER_COLOR, SMALL_HIT_THRESHOLD,
};

use crate::{
    Action, ActionEvent, ActionRequirement, Attack, AttackHeight, BlockType, FlashRequest,
    HitEffect, HitInfo, Hitbox, Lifetime, Movement, OnHitEffect, ResourceType, Situation, ToHit,
};

use super::{ActionBuilder, CharacterUniversals};

#[derive(Debug, Clone, Copy, PartialEq)]
enum HitStun {
    Stun(Stun),
    Knockdown,
    Launch(f32),
}

impl Default for HitStun {
    fn default() -> Self {
        Self::Stun(Stun::Relative(5))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stun {
    Relative(i32),
    Absolute(i32),
}

impl Default for Stun {
    fn default() -> Self {
        Stun::Relative(0)
    }
}

#[derive(Debug, Clone, Copy)]
struct StrikeBuilder {
    hit_stun: HitStun,
    attacker_push_on_hit: f32,
    defender_push_on_hit: f32,
    damage: i32,
    chip_damage: i32,
    sharpness_scaling: i32,
    block_stun: Stun,
    block_pushback_distance: f32,
    block_height: AttackHeight,
}

impl Default for StrikeBuilder {
    fn default() -> Self {
        StrikeBuilder {
            // These gotta get set by hand
            hit_stun: HitStun::default(),
            block_stun: Stun::default(),
            damage: 0,

            // These can be set by hand
            attacker_push_on_hit: 0.3,
            defender_push_on_hit: 0.4,
            block_pushback_distance: 1.0,
            block_height: AttackHeight::Mid,
            sharpness_scaling: 0,
            chip_damage: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ThrowStartupBuilder {
    on_hit_action: ActionId,
    target_action: ActionId,
    sideswitch: bool,
}

#[derive(Debug, Clone, Copy)]
enum SubBuilder {
    Throw(ThrowStartupBuilder),
    Strike(StrikeBuilder),
}
impl SubBuilder {
    fn block_type(&self) -> BlockType {
        match self {
            SubBuilder::Throw(_) => BlockType::Grab,
            SubBuilder::Strike(strike_builder) => BlockType::Strike(strike_builder.block_height),
        }
    }

    fn assert_valid(&self) {
        match self {
            SubBuilder::Throw(tb) => {
                assert_ne!(tb.on_hit_action, ActionId::default());
                assert_ne!(tb.target_action, ActionId::default());
            }
            SubBuilder::Strike(sb) => {
                assert_ne!(sb.damage, 0);
                assert_ne!(sb.hit_stun, HitStun::default());
                assert_ne!(sb.block_stun, Stun::default());
            }
        };
    }
}

impl Default for SubBuilder {
    fn default() -> Self {
        Self::Strike(StrikeBuilder::default())
    }
}

#[derive(Default)]
pub struct AttackBuilder {
    action_builder: ActionBuilder,
    hitbox: Hitbox,
    startup: usize,
    recovery: usize,
    expand_hurtbox: Option<usize>,
    spawn: Option<Model>,
    projectile: bool,
    velocity: Vec2,
    gravity: f32,
    open_cancel: Option<CancelWindow>,
    sub_builder: SubBuilder,
    hit_count: usize,
    hitbox_lifetime: Lifetime,
}

impl AttackBuilder {
    pub fn special() -> Self {
        Self {
            action_builder: ActionBuilder::special(),
            open_cancel: Some(CancelWindow {
                require_hit: true,
                cancel_type: CancelType::Super,
                duration: 10,
            }),
            expand_hurtbox: Some(5),
            hit_count: 1,
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                chip_damage: 2,
                ..default()
            }),
            ..default()
        }
    }

    pub fn normal() -> Self {
        Self {
            action_builder: ActionBuilder::normal(),
            open_cancel: Some(CancelWindow {
                require_hit: true,
                cancel_type: CancelType::Special,
                duration: 10,
            }),
            expand_hurtbox: Some(5),
            hit_count: 1,
            sub_builder: SubBuilder::Strike(StrikeBuilder::default()),
            ..default()
        }
    }

    pub fn button(btn: GameButton) -> Self {
        Self {
            action_builder: ActionBuilder::button(btn),
            ..Self::normal()
        }
    }

    fn strike_builder(&self) -> StrikeBuilder {
        let SubBuilder::Strike(sb) = self.sub_builder else {
            panic!("Not a strike")
        };
        sb
    }

    fn throw_builder(&self) -> ThrowStartupBuilder {
        let SubBuilder::Throw(tb) = self.sub_builder else {
            panic!("Not a throw")
        };
        tb
    }

    pub fn with_character_universals(self, universals: CharacterUniversals) -> Self {
        Self {
            action_builder: self.action_builder.with_character_universals(universals),
            ..self
        }
    }

    pub fn follow_up_from(self, actions: Vec<ActionId>) -> Self {
        Self {
            action_builder: self.action_builder.follow_up_from(actions),
            ..self
        }
    }

    pub fn with_input(self, input: &'static str) -> Self {
        Self {
            action_builder: self.action_builder.with_input(input),
            ..self
        }
    }

    pub fn with_meter_cost(self) -> Self {
        Self {
            action_builder: self.action_builder.with_meter_cost(),
            ..self
        }
    }

    pub fn with_charge(self) -> Self {
        Self {
            action_builder: self.action_builder.with_charge(),
            ..self
        }
    }

    pub fn with_hitbox(self, hitbox: Area) -> Self {
        Self {
            hitbox: Hitbox(hitbox),
            ..self
        }
    }

    pub fn with_multiple_hits(self, hit_count: usize) -> Self {
        Self { hit_count, ..self }
    }

    pub fn with_hitbox_gravity(self, gravity: f32) -> Self {
        Self { gravity, ..self }
    }

    pub fn with_hitbox_velocity(self, velocity: Vec2) -> Self {
        Self { velocity, ..self }
    }

    pub fn with_hitbox_speed(self, speed: f32) -> Self {
        Self {
            velocity: Vec2::X * speed,
            ..self
        }
    }

    pub fn projectile(self) -> Self {
        Self {
            projectile: true,
            ..self
        }
    }

    pub fn with_spawn(self, projectile: Model) -> Self {
        Self {
            spawn: Some(projectile),
            hitbox_lifetime: Lifetime::until_owner_hit(),
            ..self
        }
    }

    pub fn with_sound(self, sound: SoundEffect) -> Self {
        Self {
            action_builder: self.action_builder.with_sound(sound),
            ..self
        }
    }

    pub fn with_timings(self, startup: usize, recovery: usize) -> Self {
        Self {
            startup,
            recovery,
            ..self
        }
    }

    pub fn with_frame_data(self, startup: usize, active: usize, recovery: usize) -> Self {
        Self {
            startup,
            hitbox_lifetime: Lifetime::frames(active),
            recovery,
            ..self
        }
    }

    pub fn with_animation(self, animation: impl Into<Animation>) -> Self {
        Self {
            action_builder: self.action_builder.with_animation(animation),
            ..self
        }
    }

    pub fn with_disjoint(self) -> Self {
        Self {
            expand_hurtbox: None,
            ..self
        }
    }

    pub fn with_cancels_to(self, cancel_type: CancelType, window_size: usize) -> Self {
        Self {
            open_cancel: Some(CancelWindow {
                require_hit: true,
                cancel_type,
                duration: window_size,
            }),
            ..self
        }
    }

    pub fn with_no_cancels(self) -> Self {
        Self {
            open_cancel: None,
            ..self
        }
    }

    pub fn with_damage(self, damage: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                damage,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_blockstun(self, frames: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                block_stun: Stun::Absolute(frames),
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_advantage_on_block(self, frame_advantage: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                block_stun: Stun::Relative(frame_advantage),
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_hitstun(self, frames: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                hit_stun: HitStun::Stun(Stun::Absolute(frames)),
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_advantage_on_hit(self, frame_advantage: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                hit_stun: HitStun::Stun(Stun::Relative(frame_advantage)),
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn knocks_down(self) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                hit_stun: HitStun::Knockdown,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn launches(self, impulse: Vec2) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                // It is natural to think of a positive X launch as going away, but to the
                // recipient the movement needs to be negative X
                hit_stun: HitStun::Launch(impulse.y),
                defender_push_on_hit: impulse.x,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn sword(self) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                chip_damage: 5,
                sharpness_scaling: 5,
                ..self.strike_builder()
            }),
            expand_hurtbox: None,
            ..self
        }
    }

    pub fn hits_overhead(self) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                block_height: AttackHeight::High,
                ..self.strike_builder()
            }),
            ..self
        }
    }
    pub fn hits_low(self) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                block_height: AttackHeight::Low,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn crouching(self) -> Self {
        Self {
            action_builder: self.action_builder.crouching(),
            ..self
        }
    }
    pub fn air_only(self) -> Self {
        Self {
            action_builder: self.action_builder.air_only(),
            // Automatically make air strikes overheads
            sub_builder: match self.sub_builder {
                SubBuilder::Strike(sb) => SubBuilder::Strike(StrikeBuilder {
                    block_height: AttackHeight::High,
                    ..sb
                }),
                throw => throw,
            },
            ..self
        }
    }

    pub fn with_distance_on_block(self, distance: f32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                block_pushback_distance: distance,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_pushback_on_hit(self, amount: f32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                attacker_push_on_hit: amount,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn forward_throw(self) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowStartupBuilder::default()),
            ..self
        }
        .with_no_cancels()
    }

    pub fn back_throw(self) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowStartupBuilder {
                sideswitch: true,
                ..default()
            }),
            ..self
        }
        .with_no_cancels()
    }

    pub fn throw_target_action(self, target_action: impl Into<ActionId>) -> Self {
        assert!(matches!(self.sub_builder, SubBuilder::Throw(_)));

        Self {
            sub_builder: SubBuilder::Throw(ThrowStartupBuilder {
                target_action: target_action.into(),
                ..self.throw_builder()
            }),
            ..self
        }
    }

    pub fn throw_hit_action(self, on_hit_action: impl Into<ActionId>) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowStartupBuilder {
                on_hit_action: on_hit_action.into(),
                ..self.throw_builder()
            }),
            ..self
        }
    }

    pub fn with_extra_initial_events(self, events: Vec<ActionEvent>) -> Self {
        Self {
            action_builder: self.action_builder.immediate_events(events),
            ..self
        }
    }

    pub fn with_dynamic_initial_events(
        self,
        generator: impl Fn(&Situation) -> Vec<ActionEvent> + Send + Sync + 'static,
    ) -> Self {
        Self {
            action_builder: self
                .action_builder
                .dyn_immediate_events(Arc::new(generator)),
            ..self
        }
    }

    pub fn with_extra_activation_events(self, events: Vec<ActionEvent>) -> Self {
        assert_ne!(self.startup, 0, "Set startup before activation events");
        Self {
            action_builder: self.action_builder.events_on_frame(self.startup, events),
            ..self
        }
    }

    pub fn with_dynamic_activation_events(
        self,
        generator: impl Fn(&Situation) -> Vec<ActionEvent> + Send + Sync + 'static,
    ) -> Self {
        assert_ne!(self.startup, 0, "Set startup before activation events");
        Self {
            action_builder: self
                .action_builder
                .dyn_events_on_frame(self.startup, Arc::new(generator)),
            ..self
        }
    }

    pub fn with_extra_requirement(self, extra_requirement: ActionRequirement) -> Self {
        Self {
            action_builder: self.action_builder.with_requirement(extra_requirement),
            ..self
        }
    }

    fn build_script(self) -> impl Fn(&Situation) -> Vec<ActionEvent> {
        let mut activation_events = vec![];

        if let Some(can) = &self.open_cancel {
            activation_events.push(ActionEvent::AllowCancel(can.to_owned()));
        }

        if let Some(duration) = self.expand_hurtbox {
            activation_events.push(ActionEvent::ExpandHurtbox(
                self.hitbox.grow(0.1),
                self.hitbox_lifetime.frames.unwrap_or_default() + duration,
            ));
        }

        let to_hit = ToHit {
            projectile: self.projectile,
            lifetime: self.hitbox_lifetime,
            hitbox: self.hitbox,
            block_type: self.sub_builder.block_type(),
            velocity: self.velocity,
            gravity: self.gravity,
            model: self.spawn,
            hits: self.hit_count,
        };

        let on_hit = match self.sub_builder {
            SubBuilder::Throw(tb) => {
                build_throw_effect(tb.on_hit_action, tb.sideswitch, tb.target_action)
            }
            SubBuilder::Strike(sb) => {
                let block_stun = match sb.block_stun {
                    Stun::Relative(frames) => (self.recovery as i32 + frames) as usize,
                    Stun::Absolute(frames) => frames as usize,
                };
                let hit_stun_event = match sb.hit_stun {
                    HitStun::Stun(stun) => ActionEvent::HitStun(match stun {
                        Stun::Relative(frames) => (self.recovery as i32 + frames) as usize,
                        Stun::Absolute(frames) => frames as usize,
                    }),
                    HitStun::Knockdown => ActionEvent::LaunchStun(Vec2::ZERO),
                    HitStun::Launch(height) => ActionEvent::LaunchStun(Vec2::Y * height),
                };

                StrikeEffectBuilder::new(block_stun, sb.block_height, hit_stun_event, sb.damage)
                    .with_pushback_on_hit(sb.attacker_push_on_hit, sb.defender_push_on_hit)
                    .with_distance_on_block(sb.block_pushback_distance)
                    .with_chip_damage(sb.chip_damage)
                    .with_sharpness_scaling(sb.sharpness_scaling)
                    .build()
            }
        };
        activation_events.push(ActionEvent::SpawnHitbox(Attack { on_hit, to_hit }));

        self.action_builder
            .events_on_frame(self.startup, activation_events)
            .end_at(self.startup + self.recovery)
            .build_script()
    }

    pub fn build(self) -> Action {
        assert_ne!(self.startup, 0);
        assert_ne!(self.hitbox_lifetime, Lifetime::default());
        assert_ne!(self.recovery, 0);
        assert_ne!(self.hitbox, Hitbox(Area::default()));
        self.sub_builder.assert_valid();

        Action {
            input: self.action_builder.build_input(),
            requirement: self.action_builder.build_requirements(),
            script: Box::new(self.build_script()),
        }
    }
}

pub fn build_throw_effect(
    on_hit_action: ActionId,
    sideswitch: bool,
    target_action: ActionId,
) -> OnHitEffect {
    Arc::new(move |_situation: &Situation, hit_data: &HitInfo| {
        let tf = Transform::from_translation(hit_data.hitbox_pos.extend(0.0));
        if hit_data.avoided {
            HitEffect {
                attacker: vec![
                    ActionEvent::Sound(SoundEffect::BottleBonk),
                    Movement::impulse(Vec2::X * -2.0).into(),
                    ActionEvent::AbsoluteVisualEffect(VfxRequest {
                        effect: VisualEffect::ThrowTech,
                        tf,
                        ..default()
                    }),
                ],
                defender: vec![Movement::impulse(Vec2::X * -2.0).into()],
            }
        } else {
            HitEffect {
                attacker: vec![
                    ActionEvent::StartAction(on_hit_action),
                    ActionEvent::Sound(SoundEffect::PastaPat),
                    ActionEvent::AbsoluteVisualEffect(VfxRequest {
                        effect: VisualEffect::ThrowTarget,
                        tf,
                        ..default()
                    }),
                ],
                defender: vec![
                    ActionEvent::SnapToOpponent { sideswitch },
                    ActionEvent::StartAction(target_action),
                ],
            }
        }
    })
}

pub struct StrikeEffectBuilder {
    block_stun: usize,
    block_height: AttackHeight,
    attacker_push_on_block: f32,
    defender_push_on_block: f32,
    chip_damage: i32,
    hit_stun_event: ActionEvent,
    attacker_push_on_hit: f32,
    defender_push_on_hit: f32,
    base_damage: i32,
    sharpness_scaling: i32,
    extra_on_hit_effects: Vec<ActionEvent>,
}
impl StrikeEffectBuilder {
    pub fn new(
        block_stun: usize,
        block_height: AttackHeight,
        hit_stun_event: ActionEvent,
        base_damage: i32,
    ) -> Self {
        Self {
            block_stun,
            block_height,
            hit_stun_event,
            base_damage,
            attacker_push_on_block: 0.0,
            defender_push_on_block: 0.0,
            attacker_push_on_hit: 0.0,
            defender_push_on_hit: 0.0,
            chip_damage: 1,
            sharpness_scaling: 0,
            extra_on_hit_effects: vec![],
        }
    }

    pub fn with_defender_block_pushback(self, distance: f32) -> Self {
        Self {
            defender_push_on_block: distance,
            ..self
        }
    }

    pub fn with_distance_on_hit(self, distance: f32) -> Self {
        self.with_pushback_on_hit(distance * 0.3, distance * 0.7)
    }

    pub fn with_pushback_on_hit(
        self,
        attacker_push_on_hit: f32,
        defender_push_on_hit: f32,
    ) -> Self {
        Self {
            attacker_push_on_hit,
            defender_push_on_hit,
            ..self
        }
    }

    pub fn with_distance_on_block(self, distance: f32) -> Self {
        Self {
            attacker_push_on_block: 0.3 * distance,
            defender_push_on_block: 0.7 * distance,
            ..self
        }
    }

    pub fn with_chip_damage(self, chip_damage: i32) -> Self {
        Self {
            chip_damage,
            ..self
        }
    }

    pub fn with_sharpness_scaling(self, sharpness_scaling: i32) -> Self {
        Self {
            sharpness_scaling,
            ..self
        }
    }

    pub fn with_extra_on_hit_events(self, extra_events: Vec<ActionEvent>) -> Self {
        Self {
            extra_on_hit_effects: extra_events,
            ..self
        }
    }

    pub fn build(self) -> OnHitEffect {
        Arc::new(move |situation: &Situation, hit_data: &HitInfo| {
            let sharpness = situation
                .get_resource(ResourceType::Sharpness)
                .unwrap()
                .current;

            let (effect, offset, rotation) = if situation.combo.is_some() {
                (VisualEffect::Hit, Vec2::ZERO, Quat::default())
            } else {
                // First hit gets a fancier effect
                match self.block_height {
                    AttackHeight::High => (
                        VisualEffect::OpenerSpark(HIGH_OPENER_COLOR),
                        situation.facing.mirror_vec2(Vec2::new(0.5, -0.5)),
                        Quat::from_rotation_z(match situation.facing {
                            wag_core::Facing::Right => 0.0,
                            wag_core::Facing::Left => -PI / 2.0,
                        }),
                    ),
                    AttackHeight::Mid => (
                        VisualEffect::OpenerSpark(MID_OPENER_COLOR),
                        situation.facing.mirror_vec2(Vec2::X * 0.5),
                        Quat::from_rotation_z(match situation.facing {
                            wag_core::Facing::Right => PI / 6.0,
                            wag_core::Facing::Left => PI * (8.0 / 6.0),
                        }),
                    ),
                    AttackHeight::Low => (
                        VisualEffect::OpenerSpark(LOW_OPENER_COLOR),
                        situation.facing.mirror_vec2(Vec2::new(0.5, 0.5)),
                        Quat::from_rotation_z(match situation.facing {
                            wag_core::Facing::Right => PI / 2.0,
                            wag_core::Facing::Left => PI,
                        }),
                    ),
                }
            };

            if hit_data.avoided {
                HitEffect {
                    attacker: vec![
                        Movement::impulse(-Vec2::X * self.attacker_push_on_block).into(),
                        ActionEvent::CameraTilt(-Vec2::X * 0.01),
                        ActionEvent::Hitstop,
                        ActionEvent::Sound(SoundEffect::PlasticCupTap),
                        ActionEvent::AbsoluteVisualEffect(VfxRequest {
                            effect: VisualEffect::Block,
                            tf: Transform::from_translation(hit_data.hitbox_pos.extend(0.0)),
                            mirror: situation.facing.to_flipped(),
                        }),
                    ],
                    defender: vec![
                        if hit_data.defender_stats.chip_damage && self.chip_damage > 0 {
                            ActionEvent::ModifyResource(ResourceType::Health, -self.chip_damage)
                        } else {
                            ActionEvent::Noop
                        },
                        ActionEvent::BlockStun(self.block_stun),
                        Movement::impulse(-Vec2::X * self.defender_push_on_block).into(),
                        ActionEvent::CharacterShake(0.25),
                    ],
                }
            } else {
                let damage = self.base_damage + self.sharpness_scaling * sharpness;
                let voice_line_event = if damage >= BIG_HIT_THRESHOLD {
                    ActionEvent::SayVoiceLine(VoiceLine::BigHit)
                } else if damage >= SMALL_HIT_THRESHOLD {
                    ActionEvent::SayVoiceLine(VoiceLine::SmallHit)
                } else {
                    ActionEvent::Noop
                };

                HitEffect {
                    attacker: vec![
                        Movement::impulse(-Vec2::X * self.attacker_push_on_hit).into(),
                        ActionEvent::CameraTilt(Vec2::X * 0.02),
                        ActionEvent::CameraShake,
                        ActionEvent::Hitstop,
                        ActionEvent::Sound(SoundEffect::PastaPat),
                        ActionEvent::AbsoluteVisualEffect(VfxRequest {
                            effect,
                            tf: Transform {
                                translation: (hit_data.hitbox_pos + offset).extend(0.0),
                                rotation,
                                ..default()
                            },
                            mirror: situation.facing.to_flipped(),
                        }),
                    ],
                    defender: self
                        .extra_on_hit_effects
                        .clone()
                        .into_iter()
                        .chain(vec![
                            Movement::impulse(-Vec2::X * self.defender_push_on_hit).into(),
                            ActionEvent::ModifyResource(ResourceType::Health, -damage),
                            self.hit_stun_event.clone(),
                            if hit_data.airborne
                                && !matches!(self.hit_stun_event, ActionEvent::LaunchStun(_))
                            {
                                ActionEvent::LaunchStun(Vec2::new(-1.0, 5.0))
                            } else {
                                ActionEvent::Noop
                            },
                            voice_line_event,
                            ActionEvent::Flash(FlashRequest::hit_flash()),
                            ActionEvent::CharacterShake(0.5),
                        ])
                        .collect(),
                }
            }
        })
    }
}
