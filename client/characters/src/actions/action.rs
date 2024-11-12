use crate::{ActionEvent, ActionRequirement, Situation};

pub type Script = Box<dyn Fn(&Situation) -> Vec<ActionEvent> + Send + Sync>;

pub struct Action {
    pub input: Option<&'static str>,
    pub requirement: ActionRequirement,
    pub script: Script,
}

impl std::fmt::Debug for Action {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move").field("input", &self.input).finish()
    }
}

#[macro_export]
macro_rules! throw_hit {
    ($animation:expr, $duration:expr) => {
        Action {
            input: None,
            script: Box::new(|situation: &Situation| {
                if situation.on_frame(0) {
                    return vec![
                        Into::<Animation>::into($animation).into(),
                        ActionEvent::Condition(StatusCondition {
                            flag: StatusFlag::MovementLock,
                            expiration: Some($duration),
                            ..default()
                        }),
                    ];
                }

                situation.end_at($duration)
            }),
            requirement: ActionRequirement::default(),
        }
    };
}

#[macro_export]
macro_rules! throw_target {
    ($animation:expr, $duration:expr, $damage:expr, $launch_impulse:expr) => {
        throw_target!(
            $animation,
            $duration - 1,
            $duration,
            $damage,
            $launch_impulse
        )
    };

    ($animation:expr, $lock_duration:expr, $animation_duration:expr, $damage:expr, $launch_impulse:expr) => {{
        use $crate::{AnimationRequest, FlashRequest};

        Action {
            input: None,
            script: Box::new(|situation: &Situation| {
                if situation.on_frame(0) {
                    vec![
                        AnimationRequest {
                            invert: true,
                            ..AnimationRequest::from(Animation::from($animation))
                        }
                        .into(),
                        ActionEvent::ModifyResource(ResourceType::Health, -$damage),
                        if $launch_impulse == Vec2::ZERO {
                            ActionEvent::Noop
                        } else {
                            ActionEvent::LaunchStun($launch_impulse)
                        },
                        ActionEvent::Flash(FlashRequest::hit_flash()),
                        ActionEvent::Hitstop,
                        ActionEvent::Condition(StatusCondition {
                            flag: StatusFlag::MovementLock,
                            expiration: Some($lock_duration),
                            ..default()
                        }),
                    ];
                }

                situation.end_at($animation_duration)
            }),
            requirement: ActionRequirement::default(),
        }
    }};
}
