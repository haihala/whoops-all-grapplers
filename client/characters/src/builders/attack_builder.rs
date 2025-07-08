use std::{f32::consts::PI, sync::Arc};

use bevy::prelude::*;

use foundation::{
    ActionCategory, ActionId, Animation, Area, CancelType, Facing, GameButton, Icon, Model,
    RingPulse, SimpleState, Sound, StatusCondition, StatusFlag, VfxRequest, VisualEffect,
    VoiceLine, BIG_HIT_THRESHOLD, HIGH_OPENER_COLOR, JACKPOT_COLOR, JACKPOT_METER_GAIN,
    JACKPOT_RING_BASE_COLOR, LOW_OPENER_COLOR, MID_OPENER_COLOR, ON_BLOCK_HITSTOP, ON_HIT_HITSTOP,
    SMALL_HIT_THRESHOLD, THROW_TECH_RING_BASE_COLOR, THROW_TECH_RING_EDGE_COLOR,
};

use crate::{
    Action, ActionEvent, ActionRequirement, Attack, AttackHeight, BlockType, FlashRequest,
    GaugeType, HitEffect, HitInfo, Hitbox, Lifetime, Movement, OnHitEffect, Situation, ToHit,
};

use super::{ActionBuilder, CharacterUniversals, DynamicEvents, Events};

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
                debug_assert_ne!(tb.on_hit_action, ActionId::default());
                debug_assert_ne!(tb.target_action, ActionId::default());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stun {
    Relative(i32),
    Absolute(usize),
}

impl Default for Stun {
    fn default() -> Self {
        Stun::Absolute(0)
    }
}

#[derive(Default)]
pub struct AttackBuilder {
    action_builder: ActionBuilder,
    hits: Vec<(usize, HitBuilder)>,
}

impl AttackBuilder {
    pub fn special() -> Self {
        Self {
            action_builder: ActionBuilder::special(),
            ..default()
        }
    }

    pub fn normal() -> Self {
        Self {
            action_builder: ActionBuilder::normal(),
            ..default()
        }
    }

    pub fn button(btn: GameButton) -> Self {
        Self {
            action_builder: ActionBuilder::button(btn),
            ..Self::normal()
        }
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

    #[allow(unused)] // No metered attack builder attacks yet
    pub fn with_meter_cost(self) -> Self {
        Self {
            action_builder: self.action_builder.with_meter_cost(),
            ..self
        }
    }

    #[allow(unused)]
    pub fn with_charge(self) -> Self {
        Self {
            action_builder: self.action_builder.with_charge(),
            ..self
        }
    }

    pub fn with_sound(self, sound: Sound) -> Self {
        Self {
            action_builder: self.action_builder.with_sound(sound),
            ..self
        }
    }

    pub fn with_hit_on_frame(mut self, frame: usize, mut hit: HitBuilder) -> Self {
        if self.action_builder.state == Some(SimpleState::Air) {
            hit = {
                hit.sub_builder = match hit.sub_builder {
                    SubBuilder::Strike(sb) => {
                        SubBuilder::Strike(sb.with_height(AttackHeight::High))
                    }
                    throw => throw,
                };
                hit
            };
        }

        self.hits.push((frame, hit));
        self
    }

    pub fn with_vfx_on_frame(mut self, frame: usize, effect: VisualEffect, tf: Transform) -> Self {
        self.action_builder = self.action_builder.with_vfx_on_frame(frame, effect, tf);
        self
    }

    pub fn with_animation(self, animation: impl Into<Animation>) -> Self {
        Self {
            action_builder: self.action_builder.with_animation(animation),
            ..self
        }
    }

    pub fn with_total_duration(self, duration: usize) -> Self {
        Self {
            action_builder: self.action_builder.end_at(duration),
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
            // Automatically make existing strikes overheads
            hits: self
                .hits
                .into_iter()
                .map(|(frame, mut hit)| {
                    (frame, {
                        hit.sub_builder = match hit.sub_builder {
                            SubBuilder::Strike(sb) => {
                                SubBuilder::Strike(sb.with_height(AttackHeight::High))
                            }
                            throw => throw,
                        };
                        hit
                    })
                })
                .collect(),
        }
    }

    pub fn with_extra_initial_events(self, events: Vec<ActionEvent>) -> Self {
        Self {
            action_builder: self.action_builder.static_immediate_events(events),
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
        let is_normal = self.action_builder.category == ActionCategory::Normal;
        // If this is not considered, multi part normals (foot dive)
        // are always seen as cc'd
        let is_follow_up = self.action_builder.follows_up_from.is_some();

        let mut ab =
            self.action_builder
                .dyn_immediate_events(Arc::new(move |situation: &Situation| {
                    let was_cancelled_into = situation.tracker.unwrap().was_cancelled_into;
                    let mut evs = vec![];

                    if is_normal && !is_follow_up && was_cancelled_into {
                        // This was a comic cancel, as the flag would've gotten
                        // cleared if it was a raw activation
                        evs.extend([
                            ActionEvent::RelativeVisualEffect(VfxRequest {
                                effect: VisualEffect::Icon(Icon::ComicBook),
                                tf: Transform::from_translation(Vec3::Y),
                                ..default()
                            }),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::ComicCancelCooldown,
                                ..default()
                            }),
                        ]);
                    }

                    evs
                }));

        for hit in self.hits {
            let (frame, builder) = hit;
            let recovery = ab.total_duration - frame;
            ab = ab.events_on_frame(frame, builder.build(recovery as i32));
        }
        ab.build_script()
    }

    pub fn build(self) -> Action {
        debug_assert!(!self.hits.is_empty());

        Action {
            transient: false,
            input: self.action_builder.build_input(),
            requirement: self.action_builder.build_requirements(),
            script: Box::new(self.build_script()),
        }
    }
}

#[derive(Default)]
pub struct HitBuilder {
    hitbox: Hitbox,
    expand_hurtbox: Option<usize>,
    spawn: Option<Model>,
    projectile: bool,
    velocity: Vec2,
    gravity: f32,
    sub_builder: SubBuilder,
    hit_count: usize,
    hitbox_lifetime: Lifetime,
    additional_events: Events,
}

impl HitBuilder {
    fn build(mut self, recovery: i32) -> Events {
        debug_assert_ne!(self.hitbox_lifetime, Lifetime::default());
        debug_assert_ne!(self.hitbox, Hitbox(Area::default()));
        self.sub_builder.assert_valid();

        if let Some(duration) = self.expand_hurtbox {
            self.additional_events
                .constant
                .push(ActionEvent::ExpandHurtbox(
                    self.hitbox.flag_grow(0.1),
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
            SubBuilder::Strike(sb) => sb.build(recovery),
        };

        self.additional_events
            .constant
            .push(ActionEvent::SpawnHitbox(Attack { on_hit, to_hit }));

        self.additional_events
    }

    pub fn special() -> Self {
        Self {
            expand_hurtbox: Some(5),
            hit_count: 1,
            sub_builder: SubBuilder::Strike(
                StrikeEffectBuilder::default()
                    .with_chip_damage(2)
                    .with_cancel(CancelType::Super),
            ),
            ..default()
        }
    }

    pub fn normal() -> Self {
        Self {
            expand_hurtbox: Some(5),
            hit_count: 1,
            ..default()
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

    #[allow(unused)]
    fn is_throw(&self) -> bool {
        matches!(self.sub_builder, SubBuilder::Throw(_))
    }

    fn throw_builder(&self) -> ThrowStartupBuilder {
        let SubBuilder::Throw(tb) = self.sub_builder else {
            panic!("Not a throw")
        };
        tb
    }

    pub fn with_additional_events(mut self, mut events: Vec<ActionEvent>) -> Self {
        self.additional_events.constant.append(&mut events);
        self
    }

    pub fn with_dynamic_on_hit_events(self, events: DynamicEvents) -> Self {
        self.with_strike_builder(move |mut sb| {
            debug_assert!(sb.on_hit_effects.dynamic.is_none());
            sb.on_hit_effects.dynamic = Some(events.clone());
            sb
        })
    }

    pub fn with_active_frames(mut self, frames: usize) -> Self {
        self.hitbox_lifetime = Lifetime::frames(frames);
        self
    }

    pub fn with_hitbox(self, hitbox: Area) -> Self {
        Self {
            hitbox: Hitbox(hitbox),
            ..self
        }
    }

    #[allow(unused)]
    pub fn with_multiple_hits(self, hit_count: usize) -> Self {
        Self { hit_count, ..self }
    }

    #[allow(unused)]
    pub fn with_hitbox_gravity(self, gravity: f32) -> Self {
        Self { gravity, ..self }
    }

    #[allow(unused)]
    pub fn with_hitbox_velocity(self, velocity: Vec2) -> Self {
        Self { velocity, ..self }
    }

    #[allow(unused)]
    pub fn with_hitbox_speed(self, speed: f32) -> Self {
        Self {
            velocity: Vec2::X * speed,
            ..self
        }
    }

    #[allow(unused)]
    pub fn projectile(self) -> Self {
        Self {
            projectile: true,
            ..self
        }
    }

    #[allow(unused)]
    pub fn with_spawn(self, projectile: Model) -> Self {
        Self {
            spawn: Some(projectile),
            hitbox_lifetime: Lifetime::until_despawned(),
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
        self.with_strike_builder(|sb| sb.with_cancel_window(cancel_type.clone(), window_size))
    }

    #[allow(unused)]
    pub fn with_no_cancels(self) -> Self {
        self.with_strike_builder(|sb| sb.without_cancel())
    }

    pub fn with_damage(self, damage: i32) -> Self {
        self.with_strike_builder(|sb| sb.with_damage(damage))
    }

    pub fn with_blockstun(self, frames: usize) -> Self {
        self.with_strike_builder(|sb| sb.with_blockstun(Stun::Absolute(frames)))
    }

    pub fn with_hitstun(self, frames: usize) -> Self {
        self.with_strike_builder(|sb| sb.with_hitstun(Stun::Absolute(frames)))
    }

    pub fn with_advantage_on_block(self, frame_advantage: i32) -> Self {
        self.with_strike_builder(|sb| sb.with_blockstun(Stun::Relative(frame_advantage)))
    }

    pub fn with_advantage_on_hit(self, frame_advantage: i32) -> Self {
        self.with_strike_builder(|sb| sb.with_hitstun(Stun::Relative(frame_advantage)))
    }

    #[allow(unused)]
    pub fn knocks_down(self) -> Self {
        self.with_strike_builder(|sb| sb.knocks_down())
    }

    pub fn launches(self, impulse: Vec2) -> Self {
        self.with_strike_builder(move |sb| sb.launches(impulse))
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

    pub fn with_distance_on_block(self, distance: f32) -> Self {
        self.with_strike_builder(|sb| sb.with_distance_on_block(distance))
    }
    pub fn with_distance_on_hit(self, distance: f32) -> Self {
        self.with_strike_builder(|sb| sb.with_distance_on_hit(distance))
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
    }

    pub fn back_throw(self) -> Self {
        Self {
            sub_builder: SubBuilder::Throw(ThrowStartupBuilder {
                sideswitch: true,
                ..default()
            }),
            ..self
        }
    }

    pub fn throw_target_action(self, target_action: impl Into<ActionId>) -> Self {
        debug_assert!(matches!(self.sub_builder, SubBuilder::Throw(_)));

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
                    ActionEvent::Sound(Sound::BottleBonk.into()),
                    Movement::impulse(Vec2::X * -2.0).into(),
                    ActionEvent::AbsoluteVisualEffect(VfxRequest {
                        effect: VisualEffect::RingPulse(RingPulse {
                            base_color: THROW_TECH_RING_BASE_COLOR,
                            edge_color: THROW_TECH_RING_EDGE_COLOR,
                            ..default()
                        }),
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
                    ActionEvent::Sound(Sound::PastaPat.into()),
                ],
                defender: vec![
                    ActionEvent::SnapToOpponent { sideswitch },
                    ActionEvent::StartAction(target_action),
                ],
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
pub enum HitStun {
    Stun(Stun),
    Launch(Vec2),
    Knockdown,
}

impl Default for HitStun {
    fn default() -> Self {
        HitStun::Stun(Stun::default())
    }
}

#[derive(Debug, Clone)]
pub struct StrikeEffectBuilder {
    block_stun: Stun,
    block_height: AttackHeight,
    attacker_push_on_block: f32,
    defender_push_on_block: f32,
    chip_damage: i32,
    hit_stun: HitStun,
    attacker_push_on_hit: f32,
    defender_push_on_hit: f32,
    base_damage: i32,
    sharpness_scaling: i32,
    on_hit_effects: Events,
    cancel: Option<(CancelType, Option<usize>)>,
}
impl Default for StrikeEffectBuilder {
    fn default() -> Self {
        Self {
            // These gotta get set
            block_stun: Default::default(),
            hit_stun: Default::default(),
            base_damage: 0,
            on_hit_effects: Events::default(),
            cancel: Some((CancelType::Special, None)),
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

    pub fn with_blockstun(self, stun: Stun) -> Self {
        Self {
            block_stun: stun,
            ..self
        }
    }

    pub fn with_hitstun(self, stun: Stun) -> Self {
        Self {
            hit_stun: HitStun::Stun(stun),
            ..self
        }
    }

    pub fn launches(mut self, impulse: Vec2) -> Self {
        self.defender_push_on_hit = impulse.x;
        self.hit_stun = HitStun::Launch(Vec2::Y * impulse.y);
        self
    }

    pub fn knocks_down(mut self) -> Self {
        self.hit_stun = HitStun::Knockdown;
        self
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

    pub fn with_on_hit_events(mut self, mut events: Vec<ActionEvent>) -> Self {
        self.on_hit_effects.constant.append(&mut events);
        self
    }

    pub fn with_cancel_window(mut self, ct: CancelType, window: usize) -> Self {
        self.cancel = Some((ct, Some(window)));
        self
    }

    pub fn with_cancel(mut self, ct: CancelType) -> Self {
        self.cancel = Some((ct, None));
        self
    }

    pub fn without_cancel(mut self) -> Self {
        self.cancel = None;
        self
    }

    pub fn build(self, recovery: i32) -> OnHitEffect {
        Arc::new(move |situation: &Situation, hit_data: &HitInfo| {
            let sharpness = situation
                .get_resource(GaugeType::Sharpness)
                .map(|re| re.current)
                .unwrap_or_default();

            let jackpot_level = if let Some(StatusFlag::Jackpot { target_frame }) = situation
                .status_flags
                .iter()
                .find(|sf| matches!(sf, StatusFlag::Jackpot { target_frame: _ }))
            {
                let offset = (situation.abs_frame as isize - *target_frame as isize).abs();
                match offset {
                    0 => 3,
                    1..3 => 2,
                    3..6 => 1,
                    _ => 0,
                }
            } else {
                0
            };

            let jackpot_multiplier = match jackpot_level {
                3 => 1.0,
                2 => 0.5,
                1 => 0.25,
                0 => 0.0,
                _ => panic!("Jackpot level is not 0-3, but {jackpot_level}"),
            };

            let (effect, offset, rotation) = if situation.combo.ongoing() {
                (VisualEffect::Hit, Vec2::ZERO, Quat::default())
            } else {
                // First hit gets a fancier effect
                match self.block_height {
                    AttackHeight::High => (
                        VisualEffect::OpenerSpark(HIGH_OPENER_COLOR),
                        situation.facing.visual.mirror_vec2(Vec2::new(0.5, -0.5)),
                        Quat::from_rotation_z(match situation.facing.visual {
                            Facing::Right => 0.0,
                            Facing::Left => -PI / 2.0,
                        }),
                    ),
                    AttackHeight::Mid => (
                        VisualEffect::OpenerSpark(MID_OPENER_COLOR),
                        situation.facing.visual.mirror_vec2(Vec2::X * 0.5),
                        Quat::from_rotation_z(match situation.facing.visual {
                            Facing::Right => PI / 6.0,
                            Facing::Left => PI * (8.0 / 6.0),
                        }),
                    ),
                    AttackHeight::Low => (
                        VisualEffect::OpenerSpark(LOW_OPENER_COLOR),
                        situation.facing.visual.mirror_vec2(Vec2::new(0.5, 0.5)),
                        Quat::from_rotation_z(match situation.facing.visual {
                            Facing::Right => PI / 2.0,
                            Facing::Left => PI,
                        }),
                    ),
                }
            };

            let cancel_event = if let Some((ct, duration)) = &self.cancel {
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::Cancel(ct.clone()),
                    expiration: *duration,
                    ..default()
                })
            } else {
                ActionEvent::Noop
            };

            if hit_data.avoided {
                HitEffect {
                    attacker: vec![
                        cancel_event,
                        Movement::impulse(-Vec2::X * self.attacker_push_on_block).into(),
                        ActionEvent::CameraTilt(-Vec2::X * 0.01),
                        ActionEvent::Hitstop(ON_BLOCK_HITSTOP),
                        ActionEvent::Sound(Sound::PlasticCupTap.into()),
                        ActionEvent::AbsoluteVisualEffect(VfxRequest {
                            effect: VisualEffect::Block,
                            tf: Transform::from_translation(hit_data.hitbox_pos.extend(0.0)),
                            mirror: situation.facing.visual.to_flipped(),
                        }),
                    ],
                    defender: vec![
                        if hit_data.defender_stats.chip_damage && self.chip_damage > 0 {
                            ActionEvent::ModifyResource(GaugeType::Health, -self.chip_damage)
                        } else {
                            ActionEvent::Noop
                        },
                        ActionEvent::BlockStun({
                            match self.block_stun {
                                Stun::Relative(advantage) => (recovery + advantage) as usize,
                                Stun::Absolute(frames) => frames,
                            }
                        }),
                        Movement::impulse(-Vec2::X * self.defender_push_on_block).into(),
                        ActionEvent::Hitstop(ON_BLOCK_HITSTOP),
                        ActionEvent::CharacterShake(0.25),
                    ],
                }
            } else {
                let damage = ((self.base_damage + self.sharpness_scaling * sharpness) as f32
                    * (1.0 + jackpot_multiplier)) as i32;
                let voice_line_event = if damage >= BIG_HIT_THRESHOLD {
                    ActionEvent::SayVoiceLine(VoiceLine::BigHit)
                } else if damage >= SMALL_HIT_THRESHOLD {
                    ActionEvent::SayVoiceLine(VoiceLine::SmallHit)
                } else {
                    ActionEvent::Noop
                };

                let (launcher, stun_event) = match self.hit_stun {
                    HitStun::Stun(stun) => (
                        false,
                        ActionEvent::HitStun(
                            match stun {
                                Stun::Relative(advantage) => (recovery + advantage) as usize,
                                Stun::Absolute(frames) => frames,
                            } + (jackpot_multiplier * 20.0) as usize,
                        ),
                    ),
                    HitStun::Launch(vec2) => (true, ActionEvent::LaunchStun(vec2)),
                    HitStun::Knockdown => (true, ActionEvent::LaunchStun(Vec2::ZERO)),
                };

                let jackpot_events = if jackpot_level != 0 {
                    vec![
                        ActionEvent::Sound(Sound::BoxingBell(jackpot_level).into()),
                        ActionEvent::Flash(FlashRequest::jackpot(jackpot_level)),
                        ActionEvent::RelativeVisualEffect(VfxRequest {
                            effect: VisualEffect::RingPulse(RingPulse {
                                base_color: JACKPOT_RING_BASE_COLOR,
                                edge_color: JACKPOT_COLOR,
                                rings: jackpot_level,
                                thickness: 0.15,
                                offset: 0.2,
                                duration: 0.4 * (jackpot_level + 1) as f32,
                            }),
                            tf: Transform {
                                rotation: Quat::from_axis_angle(Vec3::X, -PI / 2.0),
                                translation: Vec3::Y * 0.2,
                                scale: Vec3::splat(2.0),
                            },
                            ..default()
                        }),
                    ]
                } else {
                    vec![]
                };

                HitEffect {
                    attacker: vec![
                        cancel_event,
                        Movement::impulse(-Vec2::X * self.attacker_push_on_hit).into(),
                        ActionEvent::CameraTilt(Vec2::X * 0.02),
                        ActionEvent::CameraShake,
                        ActionEvent::Hitstop(ON_HIT_HITSTOP),
                        ActionEvent::Sound(Sound::PastaPat.into()),
                        ActionEvent::AbsoluteVisualEffect(VfxRequest {
                            effect,
                            tf: Transform {
                                translation: (hit_data.hitbox_pos + offset).extend(0.0),
                                rotation,
                                ..default()
                            },
                            mirror: situation.facing.visual.to_flipped(),
                        }),
                        ActionEvent::ModifyResource(
                            GaugeType::Meter,
                            (jackpot_multiplier * JACKPOT_METER_GAIN as f32) as i32,
                        ),
                    ]
                    .into_iter()
                    .chain(jackpot_events)
                    .collect(),
                    defender: self
                        .on_hit_effects
                        .constant
                        .clone()
                        .into_iter()
                        .chain(if let Some(dyn_effect) = &self.on_hit_effects.dynamic {
                            (dyn_effect)(situation)
                        } else {
                            vec![]
                        })
                        .chain([
                            ActionEvent::MultiplyMomentum(Vec2::new(0.3, 0.2)),
                            stun_event,
                            Movement::impulse(-Vec2::X * self.defender_push_on_hit).into(),
                            ActionEvent::ModifyResource(GaugeType::Health, -damage),
                            if hit_data.airborne && !launcher {
                                ActionEvent::LaunchStun(Vec2::new(-1.0, 5.0))
                            } else {
                                ActionEvent::Noop
                            },
                            voice_line_event,
                            ActionEvent::Hitstop(ON_HIT_HITSTOP),
                            ActionEvent::Flash(FlashRequest::hit_flash()),
                            ActionEvent::CharacterShake(0.5),
                        ])
                        .collect(),
                }
            }
        })
    }

    fn assert_valid(&self) {
        debug_assert_ne!(self.base_damage, 0);
        debug_assert_ne!(self.block_stun, Stun::Absolute(0));
        debug_assert_ne!(self.hit_stun, HitStun::Stun(Stun::Absolute(0)));
    }
}
