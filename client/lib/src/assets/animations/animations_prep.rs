use bevy::{animation::AnimationTargetId, asset::AssetPath, prelude::*, utils::HashMap};

use wag_core::{Animation, Facing, SamuraiAnimation};

#[derive(Debug, Default, Resource)]
pub struct Animations {
    pub monograph: Handle<AnimationGraph>,
    normal: HashMap<Animation, AnimationNodeIndex>,
    mirrored: HashMap<Animation, AnimationNodeIndex>,
}

impl Animations {
    pub fn new(
        monograph: Handle<AnimationGraph>,
        animations: HashMap<Animation, AnimationNodeIndex>,
    ) -> Self {
        Self {
            monograph,
            normal: animations,
            mirrored: default(),
        }
    }

    fn originals_loaded(
        &self,
        graph_assets: &Assets<AnimationGraph>,
        clip_assets: &Assets<AnimationClip>,
    ) -> bool {
        let monograph = graph_assets.get(&self.monograph).unwrap();

        self.normal.values().all(|handle| {
            let Some(node) = monograph.get(*handle) else {
                return false;
            };

            let clip_handle = node.clip.clone().unwrap();
            clip_assets.get(&clip_handle).is_some()
        })
    }

    pub(super) fn get(&self, animation: Animation, flipped: &Facing) -> AnimationNodeIndex {
        *match flipped {
            Facing::Right => self.normal.get(&animation),
            Facing::Left => self.mirrored.get(&animation),
        }
        .unwrap()
    }
}

pub fn mirror_after_load(
    mut animations: ResMut<Animations>,
    mut clip_assets: ResMut<Assets<AnimationClip>>,
    mut graph_assets: ResMut<Assets<AnimationGraph>>,
    mut done: Local<bool>,
) {
    if !animations.originals_loaded(&graph_assets, &clip_assets) || !animations.mirrored.is_empty()
    {
        return;
    }

    // TODO: This is horrendous. There is got to be a better way.
    let base_hierarchy = vec!["Samurai"]; // It's character specific (blender root object name)

    let hand_base = vec![
        "DEF-upper_arm.{}",
        "DEF-upper_arm.{}.001",
        "DEF-forearm.{}",
        "DEF-forearm.{}.001",
        "DEF-hand.{}",
    ];

    let hand = |new: Vec<&'static str>| {
        let mut hc = hand_base.clone();
        hc.extend(new);
        hc
    };

    // Flattening hierarchy when exporting means bone structure here doesn't match one in blender
    let mirror_map = vec![
        // Legs
        vec![
            "DEF-thigh.{}",
            "DEF-thigh.{}.001",
            "DEF-shin.{}",
            "DEF-shin.{}.001",
            "DEF-foot.{}",
            "DEF-toe.{}",
        ],
        // Face
        vec!["DEF-ear.{}"],
        vec!["DEF-ear.{}.001"],
        vec!["DEF-ear.{}.002", "DEF-ear.{}.003"],
        vec!["DEF-ear.{}.004"],
        vec!["DEF-nose.{}.001"],
        vec!["DEF-eye_master.{}"],
        vec!["DEF-eye.{}", "DEF-eye_iris.{}"],
        vec![
            "DEF-lid.B.{}",
            "DEF-lid.B.{}.001",
            "DEF-lid.B.{}.002",
            "DEF-lid.B.{}.003",
        ],
        vec![
            "DEF-lid.T.{}",
            "DEF-lid.T.{}.001",
            "DEF-lid.T.{}.002",
            "DEF-lid.T.{}.003",
        ],
        vec!["DEF-lip.B.{}", "DEF-lip.B.{}.001"],
        vec!["DEF-lip.T.{}", "DEF-lip.T.{}.001"],
        vec!["DEF-jaw.{}", "DEF-jaw.{}.001", "DEF-chin.{}"],
        vec![
            "DEF-brow.B.{}",
            "DEF-brow.B.{}.001",
            "DEF-brow.B.{}.002",
            "DEF-brow.B.{}.003",
        ],
        vec!["DEF-brow.B.{}.004"],
        vec!["DEF-brow.T.{}"],
        vec!["DEF-brow.T.{}.001", "DEF-brow.T.{}.002"],
        vec!["DEF-brow.T.{}.003"],
        vec!["DEF-cheek.B.{}", "DEF-cheek.B.{}.001"],
        vec!["DEF-cheek.T.{}", "DEF-cheek.T.{}.001"],
        vec!["DEF-forehead.{}"],
        vec!["DEF-forehead.{}.001"],
        vec!["DEF-forehead.{}.002"],
        vec!["DEF-temple.{}"],
        // Body
        vec!["DEF-breast.{}"],
        vec!["DEF-pelvis.{}"],
        vec!["DEF-shoulder.{}"],
        // Hands
        hand(vec!["DEF-palm.01.{}"]),
        hand(vec!["DEF-palm.02.{}"]),
        hand(vec!["DEF-palm.03.{}"]),
        hand(vec!["DEF-palm.04.{}"]),
        hand(vec![
            "DEF-thumb.01.{}",
            "DEF-thumb.02.{}",
            "DEF-thumb.03.{}",
        ]),
        hand(vec![
            "DEF-f_index.01.{}",
            "DEF-f_index.02.{}",
            "DEF-f_index.03.{}",
        ]),
        hand(vec![
            "DEF-f_middle.01.{}",
            "DEF-f_middle.02.{}",
            "DEF-f_middle.03.{}",
        ]),
        hand(vec![
            "DEF-f_ring.01.{}",
            "DEF-f_ring.02.{}",
            "DEF-f_ring.03.{}",
        ]),
        hand(vec![
            "DEF-f_pinky.01.{}",
            "DEF-f_pinky.02.{}",
            "DEF-f_pinky.03.{}",
        ]),
    ]
    .into_iter()
    .flat_map(|sides| {
        let mut coll = vec![];
        for size in 1..=sides.len() {
            let half_template = sides.clone().into_iter().take(size).collect::<Vec<_>>();

            let (lefts, rights) = base_hierarchy
                .clone()
                .into_iter()
                .chain(half_template.clone().into_iter())
                .map(|template| {
                    (
                        Name::new(template.replace("{}", "L")),
                        Name::new(template.replace("{}", "R")),
                    )
                })
                .collect::<(Vec<_>, Vec<_>)>();

            coll.push((
                AnimationTargetId::from_names(rights.iter()),
                AnimationTargetId::from_names(lefts.iter()),
            ));

            coll.push((
                AnimationTargetId::from_names(lefts.iter()),
                AnimationTargetId::from_names(rights.iter()),
            ));
        }

        coll
    })
    .collect::<HashMap<AnimationTargetId, AnimationTargetId>>();

    let graph = graph_assets.get_mut(&animations.monograph).unwrap();

    animations.mirrored = animations
        .normal
        .iter()
        .map(|(animation, normal_index)| {
            let node = graph.get(*normal_index).unwrap();
            let clip_handle = node.clip.clone().unwrap();
            let clip = clip_assets.get(&clip_handle).unwrap();
            let curves = clip.curves();

            let mirrored = curves.into_iter().fold(
                AnimationClip::default(),
                |mut clip_acc, (uuid, animation_curves)| {
                    let mirrored_uuid = mirror_map.get(uuid).cloned().unwrap_or(uuid.to_owned());
                    for curve in animation_curves.iter() {
                        clip_acc.add_curve_to_target(mirrored_uuid, mirror_curve(curve.to_owned()));
                    }

                    clip_acc
                },
            );

            let mirrored_index = graph.add_clip(clip_assets.add(mirrored), 1.0, graph.root);
            (animation.to_owned(), mirrored_index)
        })
        .collect();
    *done = true;
}

fn mirror_curve(original: VariableCurve) -> VariableCurve {
    VariableCurve {
        keyframes: match original.keyframes {
            Keyframes::Rotation(frames) => Keyframes::Rotation(
                frames
                    .into_iter()
                    .map(|frame| {
                        let (axis, angle) = frame.to_axis_angle();
                        Quat::from_axis_angle(Vec3::new(-axis.x, axis.y, axis.z), -angle)
                    })
                    .collect(),
            ),
            Keyframes::Translation(frames) => Keyframes::Translation(
                frames
                    .into_iter()
                    .map(|frame| Vec3::new(-frame.x, frame.y, frame.z))
                    .collect(),
            ),
            scale => scale,
        },
        ..original
    }
}

pub fn animation_paths() -> HashMap<Animation, AssetPath<'static>> {
    // Every time a new animation is added, other animations may get affected
    // They are alphabetically ordered

    load_glb_animations(
        "samurai.glb".to_owned(),
        vec![
            SamuraiAnimation::Air,
            SamuraiAnimation::AirStab,
            SamuraiAnimation::AirStagger,
            SamuraiAnimation::AirThrowHit,
            SamuraiAnimation::AirThrowStartup,
            SamuraiAnimation::AirThrowTarget,
            SamuraiAnimation::Block,
            SamuraiAnimation::Crouch,
            SamuraiAnimation::CrouchBlock,
            SamuraiAnimation::CrouchStagger,
            SamuraiAnimation::CrouchThrowHit,
            SamuraiAnimation::CrouchThrowStartup,
            SamuraiAnimation::CrouchThrowTarget,
            SamuraiAnimation::DashBack,
            SamuraiAnimation::DashForward,
            SamuraiAnimation::FalconKnee,
            SamuraiAnimation::FootDiveHold,
            SamuraiAnimation::FootDiveRelease,
            SamuraiAnimation::Getup,
            SamuraiAnimation::GiParry,
            SamuraiAnimation::HeelKick,
            SamuraiAnimation::HighStab,
            SamuraiAnimation::Idle,
            SamuraiAnimation::Jump,
            SamuraiAnimation::KneeThrust,
            SamuraiAnimation::KunaiThrow,
            SamuraiAnimation::LowKick,
            SamuraiAnimation::SkyStab,
            SamuraiAnimation::Stagger,
            SamuraiAnimation::StandThrowHit,
            SamuraiAnimation::StandThrowStartup,
            SamuraiAnimation::StandThrowTarget,
            SamuraiAnimation::StandPose,
            SamuraiAnimation::SwordStance,
            SamuraiAnimation::StanceCancel,
            SamuraiAnimation::FastViperStrike,
            SamuraiAnimation::SlowViperStrike,
            SamuraiAnimation::FastSwordSlam,
            SamuraiAnimation::SlowSwordSlam,
            SamuraiAnimation::FastSharpen,
            SamuraiAnimation::SlowSharpen,
            SamuraiAnimation::FastRisingSun,
            SamuraiAnimation::SlowRisingSun,
            SamuraiAnimation::TPose,
            SamuraiAnimation::Uppercut,
            SamuraiAnimation::WalkBack,
            SamuraiAnimation::WalkForward,
        ],
    )
}

fn load_glb_animations(
    file_path: String,
    animations: Vec<impl Into<Animation>>,
) -> HashMap<Animation, AssetPath<'static>> {
    animations
        .into_iter()
        .enumerate()
        .map(|(index, animation)| {
            (
                animation.into(),
                GltfAssetLabel::Animation(index).from_asset(file_path.to_owned()),
            )
        })
        .collect()
}
