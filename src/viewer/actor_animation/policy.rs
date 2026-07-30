//! Engine-independent gameplay actor-animation selection and transition policy.

use bevyout_core::actor_animation::{
    PreparedActorAnimationClip, PreparedActorAnimationClipStatus, PreparedActorAnimationKind,
    PreparedActorAnimationLoopMode, PreparedActorAnimationRootMotionPolicy,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum ActorAnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    TurnLeft,
    TurnRight,
    Equip,
    Unequip,
}

impl ActorAnimationState {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Walk => "walk",
            Self::Run => "run",
            Self::TurnLeft => "turn_left",
            Self::TurnRight => "turn_right",
            Self::Equip => "equip",
            Self::Unequip => "unequip",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "idle" => Some(Self::Idle),
            "walk" => Some(Self::Walk),
            "run" => Some(Self::Run),
            "turnleft" | "turn_left" => Some(Self::TurnLeft),
            "turnright" | "turn_right" => Some(Self::TurnRight),
            "equip" => Some(Self::Equip),
            "unequip" => Some(Self::Unequip),
            _ => None,
        }
    }

    pub(crate) const fn is_one_shot(self) -> bool {
        matches!(
            self,
            Self::TurnLeft | Self::TurnRight | Self::Equip | Self::Unequip
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ActorAnimationContext<'a> {
    pub(crate) kind: PreparedActorAnimationKind,
    pub(crate) female: bool,
    pub(crate) weapon_prefix: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClipSelection {
    pub(crate) clip_name: String,
    pub(crate) source_path: String,
    pub(crate) state: ActorAnimationState,
    pub(crate) fallback_from: Option<ActorAnimationState>,
    pub(crate) loop_mode: PreparedActorAnimationLoopMode,
    pub(crate) root_motion_policy: PreparedActorAnimationRootMotionPolicy,
}

/// Maps FO3 `WEAP.DNAM.Animation Type` to the third-person actor KF prefix.
/// Energy/ballistic variants share the same animation group, matching the
/// engine's `GetWeaponAnimType` grouping.
pub(crate) const fn weapon_animation_prefix(value: u32) -> Option<&'static str> {
    match value {
        0 => Some("h2h"),
        1 => Some("1hm"),
        2 => Some("2hm"),
        3 | 4 => Some("1hp"),
        5 | 7 => Some("2hr"),
        6 => Some("2ha"),
        8 => Some("2hh"),
        9 => Some("2hl"),
        10 => Some("1gt"),
        11 => Some("1lm"),
        12 => Some("1md"),
        _ => None,
    }
}

pub(crate) fn resolve_clip(
    clips: &[PreparedActorAnimationClip],
    context: ActorAnimationContext<'_>,
    requested: ActorAnimationState,
) -> Option<ClipSelection> {
    select_for_state(clips, context, requested)
        .map(|clip| selection(clip, requested, None))
        .or_else(|| {
            (requested != ActorAnimationState::Idle)
                .then(|| select_for_state(clips, context, ActorAnimationState::Idle))
                .flatten()
                .map(|clip| selection(clip, ActorAnimationState::Idle, Some(requested)))
        })
}

fn selection(
    clip: &PreparedActorAnimationClip,
    state: ActorAnimationState,
    fallback_from: Option<ActorAnimationState>,
) -> ClipSelection {
    ClipSelection {
        clip_name: clip.name.clone(),
        source_path: clip.source_kf_path.clone(),
        state,
        fallback_from,
        loop_mode: clip.loop_mode,
        root_motion_policy: clip.root_motion_policy,
    }
}

fn select_for_state<'a>(
    clips: &'a [PreparedActorAnimationClip],
    context: ActorAnimationContext<'_>,
    state: ActorAnimationState,
) -> Option<&'a PreparedActorAnimationClip> {
    clips
        .iter()
        .filter(|clip| clip.status == PreparedActorAnimationClipStatus::Ready)
        .filter_map(|clip| clip_score(clip, context, state).map(|score| (score, clip)))
        .min_by(|(left_score, left), (right_score, right)| {
            left_score
                .cmp(right_score)
                .then_with(|| {
                    left.source_kf_path
                        .to_ascii_lowercase()
                        .cmp(&right.source_kf_path.to_ascii_lowercase())
                })
                .then_with(|| {
                    left.name
                        .to_ascii_lowercase()
                        .cmp(&right.name.to_ascii_lowercase())
                })
        })
        .map(|(_, clip)| clip)
}

fn clip_score(
    clip: &PreparedActorAnimationClip,
    context: ActorAnimationContext<'_>,
    state: ActorAnimationState,
) -> Option<u8> {
    let name = clip.name.to_ascii_lowercase();
    let source = clip.source_kf_path.to_ascii_lowercase();
    let sequence = clip
        .source_sequence_name
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if matches!(
        state,
        ActorAnimationState::Equip | ActorAnimationState::Unequip
    ) {
        let prefix = context.weapon_prefix?;
        let suffix = state.label();
        return (sequence == suffix && name.starts_with(prefix)).then_some(
            if name == format!("{prefix}{suffix}") {
                0
            } else {
                1
            },
        );
    }
    if state == ActorAnimationState::Idle
        && let Some(prefix) = context.weapon_prefix
        && name == format!("{prefix}idle")
    {
        return Some(0);
    }

    let (stem, sequence_name) = match state {
        ActorAnimationState::Idle => ("mtidle", "idle"),
        ActorAnimationState::Walk => ("mtforward", "forward"),
        ActorAnimationState::Run => ("mtfastforward", "fastforward"),
        ActorAnimationState::TurnLeft => ("mtturnleft", "turnleft"),
        ActorAnimationState::TurnRight => ("mtturnright", "turnright"),
        ActorAnimationState::Equip | ActorAnimationState::Unequip => unreachable!(),
    };

    match context.kind {
        PreparedActorAnimationKind::Creature => {
            if name == stem {
                Some(0)
            } else if sequence == sequence_name && name.starts_with(stem) {
                Some(1)
            } else {
                None
            }
        }
        PreparedActorAnimationKind::Npc => {
            let preferred_suffix = match state {
                ActorAnimationState::Walk | ActorAnimationState::Run => format!(
                    "/locomotion/{}/{stem}.kf",
                    if context.female { "female" } else { "male" }
                ),
                _ => format!("/locomotion/{stem}.kf"),
            };
            if source.ends_with(&preferred_suffix) {
                Some(0)
            } else if source.ends_with(&format!("/locomotion/{stem}.kf")) {
                Some(1)
            } else if sequence == sequence_name && name.starts_with(stem) {
                // Last-resort authored locomotion variant (child/hurt). It is
                // deliberately lower priority than the actor's sex/root path.
                Some(10)
            } else {
                None
            }
        }
    }
}

pub(crate) const fn state_after_completion(state: ActorAnimationState) -> ActorAnimationState {
    match state {
        ActorAnimationState::Equip
        | ActorAnimationState::Unequip
        | ActorAnimationState::TurnLeft
        | ActorAnimationState::TurnRight => ActorAnimationState::Idle,
        state => state,
    }
}

pub(crate) const fn should_advance_playback(cell_active: bool, actor_visible: bool) -> bool {
    cell_active && actor_visible
}

pub(crate) const fn should_repeat(
    state: ActorAnimationState,
    loop_mode: PreparedActorAnimationLoopMode,
) -> bool {
    !state.is_one_shot() && matches!(loop_mode, PreparedActorAnimationLoopMode::Loop)
}

#[cfg(test)]
#[path = "tests/policy.rs"]
mod tests;
