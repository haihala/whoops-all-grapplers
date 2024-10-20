use std::f32::consts::PI;

use bevy::prelude::*;
use wag_core::{
    ActionCategory, ActionId, Animation, Area, CancelType, CancelWindow, Model, SoundEffect,
    VfxRequest, VisualEffect,
};

use crate::{ActionRequirement, ResourceType, Situation};

use super::{
    Action, ActionEvent, Attack, AttackHeight, BlockType, FlashRequest, Hitbox, Lifetime, Movement,
    Projectile, ToHit,
};

#[derive(Debug, Clone, Copy)]
enum HitStun {
    StunAdvantage(i32),
    Knockdown,
    Launch(Vec2),
}

impl Default for HitStun {
    fn default() -> Self {
        Self::StunAdvantage(5)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StrikeBuilder {
    hit_stun: HitStun,
    attacker_push_on_hit: f32,
    damage: i32,
    chip_damage: i32,
    sharpness_scaling: i32,
    advantage_on_block: i32,
    attacker_push_on_block: f32,
    defender_push_on_block: f32,
    block_height: AttackHeight,
}

#[derive(Debug, Clone, Copy, Default)]
struct ThrowBuilder {
    lock_duration: usize,
    on_hit_action: ActionId,
    target_action: ActionId,
    sideswitch: bool,
}

#[derive(Debug, Clone, Copy)]
enum SubBuilder {
    Throw(ThrowBuilder),
    Strike(StrikeBuilder),
}
impl SubBuilder {
    fn block_type(&self) -> BlockType {
        match self {
            SubBuilder::Throw(_) => BlockType::Grab,
            SubBuilder::Strike(strike_builder) => BlockType::Strike(strike_builder.block_height),
        }
    }
}

impl Default for SubBuilder {
    fn default() -> Self {
        Self::Strike(StrikeBuilder::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttackBuilder {
    input: &'static str,
    hitbox: Hitbox,
    startup: usize,
    recovery: usize,
    expand_hurtbox: Option<usize>,
    projectile: Option<Projectile>,
    velocity: Vec2,
    meter_cost: Option<i32>,
    needs_charge: bool,
    open_cancel: Option<CancelWindow>,
    air_move: bool,
    category: ActionCategory,
    animation: Animation,
    audio: SoundEffect,
    extra_initial_events: Vec<ActionEvent>,
    dynamic_initial_events: Option<fn(&Situation) -> Vec<ActionEvent>>,
    extra_activation_events: Vec<ActionEvent>,
    dynamic_activation_events: Option<fn(&Situation) -> Vec<ActionEvent>>,
    extra_requirements: Vec<ActionRequirement>,
    sub_builder: SubBuilder,
    hit_count: usize,
    follow_up_from: Option<Vec<ActionId>>,
    hitbox_lifetime: Lifetime,
}

impl AttackBuilder {
    pub fn special(input: &'static str) -> Self {
        Self {
            input,
            category: ActionCategory::Special,
            open_cancel: Some(CancelWindow {
                require_hit: true,
                cancel_type: CancelType::Super,
                duration: 10,
            }),
            expand_hurtbox: Some(5),
            hit_count: 1,
            audio: SoundEffect::FemaleExhale,
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                chip_damage: 2,
                damage: 10,
                ..default()
            }),
            ..default()
        }
    }

    pub fn normal(input: &'static str) -> Self {
        Self {
            input,
            category: ActionCategory::Normal,
            open_cancel: Some(CancelWindow {
                require_hit: true,
                cancel_type: CancelType::Special,
                duration: 10,
            }),
            expand_hurtbox: Some(5),
            hit_count: 1,
            audio: SoundEffect::FemaleExhale,
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                chip_damage: 1,
                damage: 5,
                ..default()
            }),
            ..default()
        }
    }

    pub fn follow_up_from(self, actions: Vec<ActionId>) -> Self {
        Self {
            category: ActionCategory::FollowUp,
            follow_up_from: Some(actions),
            ..self
        }
    }

    fn strike_builder(&self) -> StrikeBuilder {
        let SubBuilder::Strike(sb) = self.sub_builder else {
            panic!("Not a strike")
        };
        sb
    }

    fn throw_builder(&self) -> ThrowBuilder {
        let SubBuilder::Throw(tb) = self.sub_builder else {
            panic!("Not a throw")
        };
        tb
    }

    pub fn if_charged(self) -> Self {
        Self {
            needs_charge: true,
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

    pub fn with_projectile(self, projectile: Model, velocity: Vec2) -> Self {
        Self {
            velocity,
            projectile: Some(Projectile { model: projectile }),
            hitbox_lifetime: Lifetime::until_owner_hit(),
            ..self
        }
    }

    pub fn with_sound(self, sound: SoundEffect) -> Self {
        Self {
            audio: sound,
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
            animation: animation.into(),
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

    pub fn with_meter_cost(self, amount: i32) -> Self {
        Self {
            meter_cost: Some(amount),
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

    pub fn with_advantage_on_block(self, frame_advantage: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                advantage_on_block: frame_advantage,
                ..self.strike_builder()
            }),
            ..self
        }
    }

    pub fn with_advantage_on_hit(self, frame_advantage: i32) -> Self {
        Self {
            sub_builder: SubBuilder::Strike(StrikeBuilder {
                hit_stun: HitStun::StunAdvantage(frame_advantage),
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
                hit_stun: HitStun::Launch(impulse),
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

    pub fn air_only(self) -> Self {
        Self {
            air_move: true,
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
                attacker_push_on_block: distance * 0.33,
                defender_push_on_block: distance * 0.66,
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
            sub_builder: SubBuilder::Throw(ThrowBuilder::default()),
            ..self
        }
    }

    pub fn back_throw(self) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowBuilder {
                sideswitch: true,
                ..default()
            }),
            ..self
        }
    }

    pub fn throw_target_action(self, target_action: impl Into<ActionId>) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowBuilder {
                target_action: target_action.into(),
                ..self.throw_builder()
            }),
            ..self
        }
    }

    pub fn throw_hit_action(self, on_hit_action: impl Into<ActionId>) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowBuilder {
                on_hit_action: on_hit_action.into(),
                ..self.throw_builder()
            }),
            ..self
        }
    }

    pub fn throw_lock_duration(self, duration: usize) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowBuilder {
                lock_duration: duration,
                ..self.throw_builder()
            }),
            ..self
        }
    }

    pub fn with_extra_initial_events(self, extra_initial_events: Vec<ActionEvent>) -> Self {
        Self {
            extra_initial_events,
            ..self
        }
    }

    pub fn with_dynamic_initial_events(
        self,
        generator: fn(&Situation) -> Vec<ActionEvent>,
    ) -> Self {
        Self {
            dynamic_initial_events: Some(generator),
            ..self
        }
    }

    pub fn with_extra_activation_events(self, extra_activation_events: Vec<ActionEvent>) -> Self {
        Self {
            extra_activation_events,
            ..self
        }
    }

    pub fn with_dynamic_activation_events(
        self,
        generator: fn(&Situation) -> Vec<ActionEvent>,
    ) -> Self {
        Self {
            dynamic_activation_events: Some(generator),
            ..self
        }
    }

    pub fn with_extra_requirements(self, extra_requirements: Vec<ActionRequirement>) -> Self {
        Self {
            extra_requirements,
            ..self
        }
    }

    fn build_requirements(&self) -> Vec<ActionRequirement> {
        let mut temp = self.extra_requirements.clone();

        temp.push(if self.air_move {
            ActionRequirement::Airborne
        } else {
            ActionRequirement::Grounded
        });

        if let Some(cost) = self.meter_cost {
            temp.push(ActionRequirement::ResourceValue(ResourceType::Meter, cost));
        }

        if self.needs_charge {
            temp.push(ActionRequirement::ResourceFull(ResourceType::Charge));
        }

        if let Some(ongoing) = self.follow_up_from.clone() {
            temp.push(ActionRequirement::ActionOngoing(ongoing));
        }

        temp
    }

    fn build_script(&self) -> impl Fn(&Situation) -> Vec<ActionEvent> {
        let startup = self.startup;
        let duration = self.startup + self.recovery;

        let mut initial_events: Vec<ActionEvent> = vec![self.animation.into(), self.audio.into()];
        if !self.extra_initial_events.is_empty() {
            initial_events.extend(self.extra_initial_events.clone());
        }
        if let Some(cost) = self.meter_cost {
            initial_events.extend(vec![
                ActionEvent::ModifyResource(ResourceType::Meter, cost),
                ActionEvent::Flash(FlashRequest::meter_use()),
            ]);
        }
        if self.needs_charge {
            initial_events.push(ActionEvent::ClearResource(ResourceType::Charge));
        }

        let mut activation_events: Vec<ActionEvent> = self.extra_activation_events.clone();
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
            lifetime: self.hitbox_lifetime,
            hitbox: self.hitbox,
            block_type: self.sub_builder.block_type(),
            velocity: self.velocity,
            projectile: self.projectile,
            hits: self.hit_count,
        };

        let intermediate = match self.sub_builder {
            SubBuilder::Throw(tb) => IntermediateAttack::Throw(IntermediateThrow {
                lock_duration: tb.lock_duration,
                sideswitch: tb.sideswitch,
                self_move: tb.on_hit_action,
                target_move: tb.target_action,
                hitbox_position: self.hitbox.center(),
            }),
            SubBuilder::Strike(sb) => IntermediateAttack::Strike(IntermediateStrike {
                sharpness_scaling: sb.sharpness_scaling,
                base_damage: sb.damage,
                attacker_push_on_block: sb.attacker_push_on_block,
                attacker_push_on_hit: sb.attacker_push_on_hit,
                defender_push_on_block: sb.defender_push_on_block,
                chip_damage: sb.chip_damage,
                hit_stun_event: match sb.hit_stun {
                    HitStun::StunAdvantage(frame_advantage) => {
                        ActionEvent::HitStun((self.recovery as i32 + frame_advantage) as usize)
                    }
                    HitStun::Knockdown => ActionEvent::LaunchStun(Vec2::ZERO),
                    HitStun::Launch(impulse) => ActionEvent::LaunchStun(impulse),
                },
                block_stun: (self.recovery as i32 + sb.advantage_on_block) as usize,
                hitbox_position: self.hitbox.center(),
                block_height: sb.block_height,
            }),
        };

        let init_fun = self.dynamic_initial_events.unwrap_or(|_| vec![]);
        let activation_fun = self.dynamic_activation_events.unwrap_or(|_| vec![]);

        move |situation: &Situation| {
            if situation.elapsed() == 0 {
                let mut events = initial_events.clone();
                events.extend(init_fun(situation));
                return events;
            }

            if situation.elapsed() == startup {
                let atk = Attack {
                    to_hit,
                    ..match &intermediate {
                        IntermediateAttack::Strike(strike) => strike.build_attack(situation),
                        IntermediateAttack::Throw(throw) => throw.build_attack(situation),
                    }
                }
                .into();
                return vec![atk]
                    .into_iter()
                    .chain(activation_events.clone())
                    .chain(activation_fun(situation))
                    .collect();
            }

            situation.end_at(duration)
        }
    }

    pub fn build(self) -> Action {
        assert!(self.startup != 0);
        assert!(self.hitbox_lifetime != Lifetime::default());
        assert!(self.recovery != 0);
        assert!(self.hitbox != Hitbox(Area::default()));

        Action {
            input: Some(self.input),
            requirements: self.build_requirements(),
            category: self.category.clone(),
            script: Box::new(self.build_script()),
        }
    }
}

pub struct IntermediateThrow {
    pub sideswitch: bool,
    pub lock_duration: usize,
    pub target_move: ActionId,
    pub self_move: ActionId,
    pub hitbox_position: Vec2,
}
impl IntermediateThrow {
    fn build_attack(&self, situation: &Situation) -> Attack {
        let hitbox_pos = situation
            .facing
            .mirror_vec2(self.hitbox_position)
            .extend(0.0);

        Attack {
            self_on_hit: vec![
                ActionEvent::StartAction(self.self_move),
                ActionEvent::Hitstop,
                ActionEvent::Lock(self.lock_duration),
                ActionEvent::Sound(SoundEffect::PastaPat),
                ActionEvent::VisualEffect(VfxRequest {
                    effect: VisualEffect::ThrowTarget,
                    tf: Transform::from_translation(hitbox_pos),
                    ..default()
                }),
            ],
            self_on_avoid: vec![
                ActionEvent::Sound(SoundEffect::BottleBonk),
                Movement::impulse(Vec2::X * -2.0).into(),
                ActionEvent::VisualEffect(VfxRequest {
                    effect: VisualEffect::ThrowTech,
                    tf: Transform::from_translation(hitbox_pos),
                    ..default()
                }),
            ],

            target_on_hit: vec![
                ActionEvent::SnapToOpponent {
                    sideswitch: self.sideswitch,
                },
                ActionEvent::StartAction(self.target_move),
                ActionEvent::Lock(self.lock_duration),
            ],
            target_on_avoid: vec![Movement::impulse(Vec2::X * -2.0).into()],

            ..default()
        }
    }
}

#[derive(Debug, Default)]
pub struct IntermediateStrike {
    pub attacker_push_on_block: f32,
    pub attacker_push_on_hit: f32,
    pub defender_push_on_block: f32,
    pub base_damage: i32,
    pub chip_damage: i32,
    pub hit_stun_event: ActionEvent,
    pub block_stun: usize,
    pub sharpness_scaling: i32,
    pub hitbox_position: Vec2,
    pub block_height: AttackHeight,
}
impl IntermediateStrike {
    pub fn build_attack(&self, situation: &Situation) -> Attack {
        let sharpness = situation
            .get_resource(ResourceType::Sharpness)
            .unwrap()
            .current;

        let hitbox_pos = situation
            .facing
            .mirror_vec2(self.hitbox_position)
            .extend(0.0);

        let (effect, offset, rotation) = if situation.combo.is_some() {
            (VisualEffect::Hit, Vec3::ZERO, Quat::default())
        } else {
            // First hit gets a fancier effect
            match self.block_height {
                AttackHeight::Low => (
                    VisualEffect::Sparks,
                    situation.facing.mirror_vec3(Vec3::new(0.9, 0.9, 0.0)),
                    Quat::default(),
                ),
                AttackHeight::Mid => (
                    VisualEffect::MidFlash,
                    situation.facing.mirror_vec3(Vec3::X * 0.5),
                    Quat::from_rotation_z(match situation.facing {
                        wag_core::Facing::Right => PI / 6.0,
                        wag_core::Facing::Left => PI * (8.0 / 6.0),
                    }),
                ),
                AttackHeight::High => (VisualEffect::Lightning, Vec3::ZERO, Quat::default()),
            }
        };

        Attack {
            self_on_hit: vec![
                Movement::impulse(-Vec2::X * self.attacker_push_on_hit).into(),
                ActionEvent::CameraTilt(Vec2::X * 0.02),
                ActionEvent::CameraShake,
                ActionEvent::Hitstop,
                ActionEvent::Sound(SoundEffect::PastaPat),
                ActionEvent::VisualEffect(VfxRequest {
                    effect,
                    tf: Transform {
                        translation: (hitbox_pos + offset),
                        rotation,
                        ..default()
                    },
                    ..default()
                }),
            ],
            self_on_avoid: vec![
                Movement::impulse(-Vec2::X * self.attacker_push_on_block).into(),
                ActionEvent::CameraTilt(-Vec2::X * 0.01),
                ActionEvent::Hitstop,
                ActionEvent::Sound(SoundEffect::PlasticCupTap),
                ActionEvent::VisualEffect(VfxRequest {
                    effect: VisualEffect::Block,
                    tf: Transform::from_translation(hitbox_pos),
                    ..default()
                }),
            ],
            target_on_hit: vec![
                ActionEvent::ModifyResource(
                    ResourceType::Health,
                    -(self.base_damage + self.sharpness_scaling * sharpness),
                ),
                self.hit_stun_event.clone(),
                Movement::impulse(-Vec2::X * self.defender_push_on_block).into(),
                ActionEvent::Flash(FlashRequest::hit_flash()),
            ],
            target_on_avoid: vec![
                if self.chip_damage > 0 {
                    ActionEvent::ModifyResource(ResourceType::Health, -self.chip_damage)
                } else {
                    ActionEvent::Noop
                },
                ActionEvent::BlockStun(self.block_stun),
                Movement::impulse(-Vec2::X * self.defender_push_on_block).into(),
            ],
            ..default()
        }
    }
}

enum IntermediateAttack {
    Strike(IntermediateStrike),
    Throw(IntermediateThrow),
}
