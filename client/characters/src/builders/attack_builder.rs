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

#[derive(Debug, Clone, Copy, Default)]
struct ThrowStartupBuilder {
    on_hit_action: ActionId,
    target_action: ActionId,
    sideswitch: bool,
}

#[derive(Debug, Clone)]
enum SubBuilder {
    Throw(ThrowStartupBuilder),
    Strike(StrikeEffectBuilder),
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
                sb.assert_valid();
            }
        };
    }
}

impl Default for SubBuilder {
    fn default() -> Self {
        Self::Strike(StrikeEffectBuilder::default())
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
            sub_builder: SubBuilder::Strike(StrikeEffectBuilder::default().with_chip_damage(2)),
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
            ..default()
        }
    }

    pub fn button(btn: GameButton) -> Self {
        Self {
            action_builder: ActionBuilder::button(btn),
            ..Self::normal()
        }
    }

    fn with_strike_builder(
        mut self,
        transformer: impl Fn(StrikeEffectBuilder) -> StrikeEffectBuilder,
    ) -> Self {
        let SubBuilder::Strike(sb) = &self.sub_builder else {
            panic!("Not a strike")
        };

        self.sub_builder = SubBuilder::Strike(transformer(sb.clone()));

        self
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
        self.with_strike_builder(|sb| sb.with_damage(damage))
    }

    pub fn with_blockstun(self, frames: usize) -> Self {
        self.with_strike_builder(|sb| sb.with_blockstun(frames))
    }

    pub fn with_advantage_on_block(self, frame_advantage: i32) -> Self {
        assert_ne!(self.recovery, 0);

        let amount = (self.recovery as i32 + frame_advantage) as usize;
        self.with_blockstun(amount)
    }

    pub fn with_hitstun(self, frames: usize) -> Self {
        self.with_strike_builder(|sb| sb.with_on_hit_events(vec![ActionEvent::HitStun(frames)]))
    }

    pub fn with_advantage_on_hit(self, frame_advantage: i32) -> Self {
        assert_ne!(self.recovery, 0);

        let amount = (self.recovery as i32 + frame_advantage) as usize;
        self.with_hitstun(amount)
    }

    pub fn knocks_down(self) -> Self {
        self.with_strike_builder(|sb| {
            sb.with_on_hit_events(vec![ActionEvent::LaunchStun(Vec2::ZERO)])
        })
    }

    pub fn launches(self, impulse: Vec2) -> Self {
        self.with_strike_builder(move |sb| {
            let attacker_pushback = sb.attacker_push_on_hit;
            sb.with_on_hit_events(vec![ActionEvent::LaunchStun(Vec2::Y * impulse.y)])
                .with_pushback_on_hit(attacker_pushback, impulse.x)
        })
    }

    pub fn sword(self) -> Self {
        self.with_strike_builder(|sb| sb.with_chip_damage(5).with_sharpness_scaling(5))
            .with_disjoint()
    }

    pub fn hits_overhead(self) -> Self {
        self.with_strike_builder(|sb| sb.with_height(AttackHeight::High))
    }
    pub fn hits_low(self) -> Self {
        self.with_strike_builder(|sb| sb.with_height(AttackHeight::Low))
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
                SubBuilder::Strike(sb) => SubBuilder::Strike(sb.with_height(AttackHeight::High)),
                throw => throw,
            },
            ..self
        }
    }

    pub fn with_distance_on_block(self, distance: f32) -> Self {
        self.with_strike_builder(|sb| sb.with_distance_on_block(distance))
    }

    pub fn with_pushback_on_hit(self, amount: f32) -> Self {
        self.with_strike_builder(|sb| {
            let defender_push = sb.defender_push_on_hit;
            sb.with_pushback_on_hit(amount, defender_push)
        })
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
            SubBuilder::Strike(sb) => sb.build(),
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

#[derive(Debug, Clone)]
pub struct StrikeEffectBuilder {
    block_stun: usize,
    block_height: AttackHeight,
    attacker_push_on_block: f32,
    defender_push_on_block: f32,
    chip_damage: i32,
    attacker_push_on_hit: f32,
    defender_push_on_hit: f32,
    base_damage: i32,
    sharpness_scaling: i32,
    on_hit_effects: Vec<ActionEvent>,
}
impl Default for StrikeEffectBuilder {
    fn default() -> Self {
        Self {
            // These gotta get set
            block_stun: 0,
            base_damage: 0,
            on_hit_effects: vec![],

            block_height: AttackHeight::Mid,
            attacker_push_on_block: 0.0,
            defender_push_on_block: 0.0,
            attacker_push_on_hit: 0.0,
            defender_push_on_hit: 0.0,
            chip_damage: 1,
            sharpness_scaling: 0,
        }
        .with_distance_on_hit(0.7)
        .with_distance_on_block(1.2)
    }
}

impl StrikeEffectBuilder {
    pub fn with_height(self, block_height: AttackHeight) -> Self {
        Self {
            block_height,
            ..self
        }
    }

    pub fn with_blockstun(self, duration: usize) -> Self {
        Self {
            block_stun: duration,
            ..self
        }
    }

    pub fn with_damage(self, damage: i32) -> Self {
        Self {
            base_damage: damage,
            ..self
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

    pub fn with_on_hit_events(self, events: Vec<ActionEvent>) -> Self {
        Self {
            on_hit_effects: events,
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

                let launcher = self
                    .on_hit_effects
                    .iter()
                    .any(|ev| matches!(ev, ActionEvent::LaunchStun(_)));

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
                        .on_hit_effects
                        .clone()
                        .into_iter()
                        .chain(vec![
                            Movement::impulse(-Vec2::X * self.defender_push_on_hit).into(),
                            ActionEvent::ModifyResource(ResourceType::Health, -damage),
                            if hit_data.airborne && !launcher {
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

    fn assert_valid(&self) {
        assert_ne!(self.base_damage, 0);
        assert_ne!(self.block_stun, 0);
        assert!(!self.on_hit_effects.is_empty())
    }
}
